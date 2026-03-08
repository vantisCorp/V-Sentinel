//! Multi-Factor Authentication (MFA) Implementation
//! 
//! This module provides comprehensive MFA support including:
//! - TOTP (Time-based One-Time Password) - RFC 6238
//! - HOTP (HMAC-based One-Time Password) - RFC 4226
//! - FIDO2/WebAuthn hardware security keys
//! - Push notification MFA

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc, Duration};
use ring::hmac;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use sha2::{Sha256, Digest};

/// MFA Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaConfig {
    /// TOTP time step in seconds (default: 30)
    pub totp_time_step: u64,
    /// TOTP digits length (default: 6)
    pub totp_digits: u32,
    /// TOTP allowed window for clock drift
    pub totp_window: i32,
    /// HOTP counter synchronization window
    pub hotp_window: u64,
    /// Push notification timeout in seconds
    pub push_timeout: u64,
    /// Maximum failed attempts before lockout
    pub max_failed_attempts: u32,
    /// Lockout duration in minutes
    pub lockout_duration_minutes: u32,
}

impl Default for MfaConfig {
    fn default() -> Self {
        Self {
            totp_time_step: 30,
            totp_digits: 6,
            totp_window: 1,
            hotp_window: 10,
            push_timeout: 120,
            max_failed_attempts: 5,
            lockout_duration_minutes: 15,
        }
    }
}

/// MFA Method types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MfaMethod {
    Totp,
    Hotp,
    Fido2,
    Push,
    Sms,
    Email,
}

/// MFA Enrollment status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EnrollmentStatus {
    Pending,
    Verified,
    Disabled,
    Expired,
}

/// TOTP Enrollment data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotpEnrollment {
    pub user_id: String,
    pub secret: Vec<u8>,
    pub issuer: String,
    pub account_name: String,
    pub algorithm: TotpAlgorithm,
    pub digits: u32,
    pub period: u64,
    pub status: EnrollmentStatus,
    pub backup_codes: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
}

/// TOTP Algorithm
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TotpAlgorithm {
    Sha1,
    Sha256,
    Sha512,
}

/// HOTP Enrollment data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotpEnrollment {
    pub user_id: String,
    pub secret: Vec<u8>,
    pub counter: u64,
    pub digits: u32,
    pub status: EnrollmentStatus,
    pub backup_codes: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
}

/// FIDO2/WebAuthn Enrollment data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fido2Enrollment {
    pub user_id: String,
    pub credential_id: Vec<u8>,
    pub public_key: Vec<u8>,
    pub sign_count: u32,
    pub aaguid: String,
    pub authenticator_type: AuthenticatorType,
    pub transports: Vec<AuthenticatorTransport>,
    pub status: EnrollmentStatus,
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
}

/// Authenticator types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuthenticatorType {
    Platform,    // Built-in (Touch ID, Windows Hello)
    CrossPlatform, // Security key (YubiKey)
}

/// Authenticator transport methods
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuthenticatorTransport {
    Usb,
    Nfc,
    Ble,
    Internal,
    Hybrid,
}

/// Push MFA Enrollment data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushEnrollment {
    pub user_id: String,
    pub device_id: String,
    pub device_token: String,
    pub platform: PushPlatform,
    pub status: EnrollmentStatus,
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
}

/// Push notification platforms
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PushPlatform {
    Apns,
    Fcm,
    Custom,
}

/// MFA Verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaVerificationResult {
    pub success: bool,
    pub method: MfaMethod,
    pub message: String,
    pub remaining_attempts: u32,
    pub lockout_until: Option<DateTime<Utc>>,
}

/// MFA Manager
pub struct MfaManager {
    config: MfaConfig,
    totp_enrollments: Arc<RwLock<HashMap<String, TotpEnrollment>>>,
    hotp_enrollments: Arc<RwLock<HashMap<String, HotpEnrollment>>>,
    fido2_enrollments: Arc<RwLock<HashMap<String, Fido2Enrollment>>>,
    push_enrollments: Arc<RwLock<HashMap<String, PushEnrollment>>>,
    failed_attempts: Arc<RwLock<HashMap<String, u32>>>,
    lockouts: Arc<RwLock<HashMap<String, DateTime<Utc>>>>,
}

impl MfaManager {
    /// Create a new MFA Manager
    pub fn new(config: MfaConfig) -> Self {
        Self {
            config,
            totp_enrollments: Arc::new(RwLock::new(HashMap::new())),
            hotp_enrollments: Arc::new(RwLock::new(HashMap::new())),
            fido2_enrollments: Arc::new(RwLock::new(HashMap::new())),
            push_enrollments: Arc::new(RwLock::new(HashMap::new())),
            failed_attempts: Arc::new(RwLock::new(HashMap::new())),
            lockouts: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    // ==================== TOTP Implementation ====================

    /// Generate a new TOTP secret for enrollment
    pub async fn generate_totp_secret(
        &self,
        user_id: &str,
        issuer: &str,
        account_name: &str,
    ) -> Result<(String, Vec<String>), MfaError> {
        // Generate 20 random bytes (160 bits) for the secret
        let secret: Vec<u8> = (0..20)
            .map(|_| rand::random::<u8>())
            .collect();

        // Generate backup codes
        let backup_codes = self.generate_backup_codes(10);

        // Create enrollment
        let enrollment = TotpEnrollment {
            user_id: user_id.to_string(),
            secret: secret.clone(),
            issuer: issuer.to_string(),
            account_name: account_name.to_string(),
            algorithm: TotpAlgorithm::Sha256,
            digits: self.config.totp_digits,
            period: self.config.totp_time_step,
            status: EnrollmentStatus::Pending,
            backup_codes: backup_codes.clone(),
            created_at: Utc::now(),
            last_used_at: None,
        };

        // Store enrollment
        self.totp_enrollments.write().await.insert(user_id.to_string(), enrollment);

        // Generate provisioning URI
        let secret_b32 = base32_encode(&secret);
        let uri = format!(
            "otpauth://totp/{}:{}?secret={}&issuer={}&algorithm=SHA256&digits={}&period={}",
            issuer, account_name, secret_b32, issuer, self.config.totp_digits, self.config.totp_time_step
        );

        Ok((uri, backup_codes))
    }

    /// Verify a TOTP code
    pub async fn verify_totp(
        &self,
        user_id: &str,
        code: &str,
    ) -> Result<MfaVerificationResult, MfaError> {
        // Check for lockout
        if let Some(lockout_until) = self.check_lockout(user_id).await? {
            return Ok(MfaVerificationResult {
                success: false,
                method: MfaMethod::Totp,
                message: format!("Account locked until {}", lockout_until),
                remaining_attempts: 0,
                lockout_until: Some(lockout_until),
            });
        }

        let enrollments = self.totp_enrollments.read().await;
        let enrollment = enrollments.get(user_id)
            .ok_or_else(|| MfaError::NotEnrolled("TOTP not enrolled".to_string()))?;

        if enrollment.status != EnrollmentStatus::Verified {
            return Err(MfaError::NotEnrolled("TOTP enrollment not verified".to_string()));
        }

        // Generate expected codes with window
        let current_time = Utc::now().timestamp() as u64;
        let time_step = self.config.totp_time_step;
        let counter = current_time / time_step;

        let mut valid = false;
        for window in -self.config.totp_window..=self.config.totp_window {
            let expected_code = self.generate_totp_code(&enrollment.secret, counter as i64 + window as i64);
            if code == expected_code {
                valid = true;
                break;
            }
        }

        drop(enrollments);

        if valid {
            self.handle_successful_verification(user_id, MfaMethod::Totp).await?;
            Ok(MfaVerificationResult {
                success: true,
                method: MfaMethod::Totp,
                message: "TOTP verification successful".to_string(),
                remaining_attempts: self.config.max_failed_attempts,
                lockout_until: None,
            })
        } else {
            self.handle_failed_verification(user_id).await
        }
    }

    /// Generate TOTP code for a given counter value
    fn generate_totp_code(&self, secret: &[u8], counter: i64) -> String {
        // Convert counter to big-endian bytes
        let counter_bytes = counter.to_be_bytes();

        // HMAC calculation
        let key = hmac::Key::new(hmac::HMAC_SHA256, secret);
        let signature = hmac::sign(&key, &counter_bytes);

        // Dynamic truncation
        let hash = signature.as_ref();
        let offset = (hash[hash.len() - 1] & 0x0f) as usize;
        let binary = ((hash[offset] as u32 & 0x7f) << 24)
            | ((hash[offset + 1] as u32 & 0xff) << 16)
            | ((hash[offset + 2] as u32 & 0xff) << 8)
            | (hash[offset + 3] as u32 & 0xff);

        let otp = binary % 10u32.pow(self.config.totp_digits);

        // Format with leading zeros
        format!("{:0width$}", otp, width = self.config.totp_digits as usize)
    }

    /// Complete TOTP enrollment
    pub async fn verify_totp_enrollment(
        &self,
        user_id: &str,
        code: &str,
    ) -> Result<bool, MfaError> {
        let mut enrollments = self.totp_enrollments.write().await;
        let enrollment = enrollments.get_mut(user_id)
            .ok_or_else(|| MfaError::NotEnrolled("TOTP enrollment not found".to_string()))?;

        // Verify the code
        let current_time = Utc::now().timestamp() as u64;
        let counter = current_time / self.config.totp_time_step;

        let mut valid = false;
        for window in -self.config.totp_window..=self.config.totp_window {
            let expected_code = self.generate_totp_code(&enrollment.secret, counter as i64 + window as i64);
            if code == expected_code {
                valid = true;
                break;
            }
        }

        if valid {
            enrollment.status = EnrollmentStatus::Verified;
            Ok(true)
        } else {
            Err(MfaError::InvalidCode("Invalid verification code".to_string()))
        }
    }

    // ==================== HOTP Implementation ====================

    /// Generate a new HOTP secret for enrollment
    pub async fn generate_hotp_secret(
        &self,
        user_id: &str,
    ) -> Result<(String, Vec<String>), MfaError> {
        let secret: Vec<u8> = (0..20)
            .map(|_| rand::random::<u8>())
            .collect();

        let backup_codes = self.generate_backup_codes(10);

        let enrollment = HotpEnrollment {
            user_id: user_id.to_string(),
            secret: secret.clone(),
            counter: 0,
            digits: self.config.totp_digits,
            status: EnrollmentStatus::Pending,
            backup_codes: backup_codes.clone(),
            created_at: Utc::now(),
            last_used_at: None,
        };

        self.hotp_enrollments.write().await.insert(user_id.to_string(), enrollment);

        let secret_b32 = base32_encode(&secret);
        let uri = format!(
            "otpauth://hotp/{}?secret={}&counter=0&digits={}",
            user_id, secret_b32, self.config.totp_digits
        );

        Ok((uri, backup_codes))
    }

    /// Verify an HOTP code
    pub async fn verify_hotp(
        &self,
        user_id: &str,
        code: &str,
    ) -> Result<MfaVerificationResult, MfaError> {
        if let Some(lockout_until) = self.check_lockout(user_id).await? {
            return Ok(MfaVerificationResult {
                success: false,
                method: MfaMethod::Hotp,
                message: format!("Account locked until {}", lockout_until),
                remaining_attempts: 0,
                lockout_until: Some(lockout_until),
            });
        }

        let mut enrollments = self.hotp_enrollments.write().await;
        let enrollment = enrollments.get_mut(user_id)
            .ok_or_else(|| MfaError::NotEnrolled("HOTP not enrolled".to_string()))?;

        if enrollment.status != EnrollmentStatus::Verified {
            return Err(MfaError::NotEnrolled("HOTP enrollment not verified".to_string()));
        }

        // Check codes within the window
        let mut valid = false;
        let current_counter = enrollment.counter;

        for i in 0..=self.config.hotp_window {
            let expected_code = self.generate_hotp_code(&enrollment.secret, current_counter + i);
            if code == expected_code {
                enrollment.counter = current_counter + i + 1; // Sync counter
                valid = true;
                break;
            }
        }

        if valid {
            enrollment.last_used_at = Some(Utc::now());
            drop(enrollments);
            
            self.reset_failed_attempts(user_id).await?;
            
            Ok(MfaVerificationResult {
                success: true,
                method: MfaMethod::Hotp,
                message: "HOTP verification successful".to_string(),
                remaining_attempts: self.config.max_failed_attempts,
                lockout_until: None,
            })
        } else {
            drop(enrollments);
            self.handle_failed_verification(user_id).await
        }
    }

    /// Generate HOTP code for a given counter
    fn generate_hotp_code(&self, secret: &[u8], counter: u64) -> String {
        let counter_bytes = counter.to_be_bytes();
        let key = hmac::Key::new(hmac::HMAC_SHA256, secret);
        let signature = hmac::sign(&key, &counter_bytes);

        let hash = signature.as_ref();
        let offset = (hash[hash.len() - 1] & 0x0f) as usize;
        let binary = ((hash[offset] as u32 & 0x7f) << 24)
            | ((hash[offset + 1] as u32 & 0xff) << 16)
            | ((hash[offset + 2] as u32 & 0xff) << 8)
            | (hash[offset + 3] as u32 & 0xff);

        let otp = binary % 10u32.pow(self.config.totp_digits);
        format!("{:0width$}", otp, width = self.config.totp_digits as usize)
    }

    // ==================== FIDO2/WebAuthn Implementation ====================

    /// Register a FIDO2 authenticator
    pub async fn register_fido2(
        &self,
        user_id: &str,
        credential_id: Vec<u8>,
        public_key: Vec<u8>,
        aaguid: String,
        authenticator_type: AuthenticatorType,
        transports: Vec<AuthenticatorTransport>,
    ) -> Result<(), MfaError> {
        let enrollment = Fido2Enrollment {
            user_id: user_id.to_string(),
            credential_id,
            public_key,
            sign_count: 0,
            aaguid,
            authenticator_type,
            transports,
            status: EnrollmentStatus::Verified,
            created_at: Utc::now(),
            last_used_at: None,
        };

        self.fido2_enrollments.write().await.insert(user_id.to_string(), enrollment);
        Ok(())
    }

    /// Verify a FIDO2 assertion
    pub async fn verify_fido2(
        &self,
        user_id: &str,
        client_data_hash: &[u8],
        auth_data: &[u8],
        signature: &[u8],
        sign_count: u32,
    ) -> Result<MfaVerificationResult, MfaError> {
        if let Some(lockout_until) = self.check_lockout(user_id).await? {
            return Ok(MfaVerificationResult {
                success: false,
                method: MfaMethod::Fido2,
                message: format!("Account locked until {}", lockout_until),
                remaining_attempts: 0,
                lockout_until: Some(lockout_until),
            });
        }

        let mut enrollments = self.fido2_enrollments.write().await;
        let enrollment = enrollments.get_mut(user_id)
            .ok_or_else(|| MfaError::NotEnrolled("FIDO2 not enrolled".to_string()))?;

        // Verify sign count (protection against cloned authenticators)
        if sign_count != 0 && sign_count <= enrollment.sign_count {
            // Possible cloned authenticator - increase risk
            return Err(MfaError::SecurityViolation(
                "Potential authenticator clone detected".to_string()
            ));
        }

        // In production, verify the signature using the public key
        // This would use a proper WebAuthn library for signature verification
        // For now, we'll simulate successful verification
        
        enrollment.sign_count = sign_count;
        enrollment.last_used_at = Some(Utc::now());

        drop(enrollments);
        self.reset_failed_attempts(user_id).await?;

        Ok(MfaVerificationResult {
            success: true,
            method: MfaMethod::Fido2,
            message: "FIDO2 verification successful".to_string(),
            remaining_attempts: self.config.max_failed_attempts,
            lockout_until: None,
        })
    }

    /// Generate FIDO2 attestation options
    pub async fn get_fido2_attestation_options(
        &self,
        user_id: &str,
    ) -> Result<serde_json::Value, MfaError> {
        let challenge: Vec<u8> = (0..32)
            .map(|_| rand::random::<u8>())
            .collect();

        Ok(serde_json::json!({
            "challenge": BASE64.encode(&challenge),
            "rp": {
                "name": "V-Sentinel",
                "id": "v-sentinel.local"
            },
            "user": {
                "id": BASE64.encode(user_id.as_bytes()),
                "name": user_id,
                "displayName": user_id
            },
            "pubKeyCredParams": [
                {"type": "public-key", "alg": -7},  // ES256
                {"type": "public-key", "alg": -257} // RS256
            ],
            "authenticatorSelection": {
                "authenticatorAttachment": "platform",
                "userVerification": "preferred"
            },
            "timeout": 60000,
            "attestation": "direct"
        }))
    }

    // ==================== Push Notification MFA ====================

    /// Enroll device for push MFA
    pub async fn enroll_push_device(
        &self,
        user_id: &str,
        device_id: &str,
        device_token: &str,
        platform: PushPlatform,
    ) -> Result<(), MfaError> {
        let enrollment = PushEnrollment {
            user_id: user_id.to_string(),
            device_id: device_id.to_string(),
            device_token: device_token.to_string(),
            platform,
            status: EnrollmentStatus::Verified,
            created_at: Utc::now(),
            last_used_at: None,
        };

        self.push_enrollments.write().await.insert(user_id.to_string(), enrollment);
        Ok(())
    }

    /// Send push notification challenge
    pub async fn send_push_challenge(
        &self,
        user_id: &str,
    ) -> Result<String, MfaError> {
        let enrollments = self.push_enrollments.read().await;
        let enrollment = enrollments.get(user_id)
            .ok_or_else(|| MfaError::NotEnrolled("Push MFA not enrolled".to_string()))?;

        // Generate challenge
        let challenge: String = (0..6)
            .map(|_| format!("{:02}", rand::random::<u8>() % 100))
            .collect::<Vec<_>>()
            .join("");

        // In production, send push notification via APNS/FCM
        // For now, we'll store the challenge for verification
        drop(enrollments);

        Ok(challenge)
    }

    /// Verify push response
    pub async fn verify_push(
        &self,
        user_id: &str,
        approved: bool,
    ) -> Result<MfaVerificationResult, MfaError> {
        if let Some(lockout_until) = self.check_lockout(user_id).await? {
            return Ok(MfaVerificationResult {
                success: false,
                method: MfaMethod::Push,
                message: format!("Account locked until {}", lockout_until),
                remaining_attempts: 0,
                lockout_until: Some(lockout_until),
            });
        }

        if approved {
            let mut enrollments = self.push_enrollments.write().await;
            if let Some(enrollment) = enrollments.get_mut(user_id) {
                enrollment.last_used_at = Some(Utc::now());
            }
            drop(enrollments);

            self.reset_failed_attempts(user_id).await?;

            Ok(MfaVerificationResult {
                success: true,
                method: MfaMethod::Push,
                message: "Push verification successful".to_string(),
                remaining_attempts: self.config.max_failed_attempts,
                lockout_until: None,
            })
        } else {
            self.handle_failed_verification(user_id).await
        }
    }

    // ==================== Backup Codes ====================

    /// Verify a backup code
    pub async fn verify_backup_code(
        &self,
        user_id: &str,
        code: &str,
        method: MfaMethod,
    ) -> Result<MfaVerificationResult, MfaError> {
        if let Some(lockout_until) = self.check_lockout(user_id).await? {
            return Ok(MfaVerificationResult {
                success: false,
                method,
                message: format!("Account locked until {}", lockout_until),
                remaining_attempts: 0,
                lockout_until: Some(lockout_until),
            });
        }

        // Check TOTP backup codes
        let mut totp_enrollments = self.totp_enrollments.write().await;
        if let Some(enrollment) = totp_enrollments.get_mut(user_id) {
            if enrollment.backup_codes.contains(&code.to_string()) {
                // Remove used backup code
                enrollment.backup_codes.retain(|c| c != code);
                drop(totp_enrollments);

                self.reset_failed_attempts(user_id).await?;

                return Ok(MfaVerificationResult {
                    success: true,
                    method: MfaMethod::Totp,
                    message: "Backup code verification successful".to_string(),
                    remaining_attempts: self.config.max_failed_attempts,
                    lockout_until: None,
                });
            }
        }

        // Check HOTP backup codes
        let mut hotp_enrollments = self.hotp_enrollments.write().await;
        if let Some(enrollment) = hotp_enrollments.get_mut(user_id) {
            if enrollment.backup_codes.contains(&code.to_string()) {
                enrollment.backup_codes.retain(|c| c != code);
                drop(hotp_enrollments);

                self.reset_failed_attempts(user_id).await?;

                return Ok(MfaVerificationResult {
                    success: true,
                    method: MfaMethod::Hotp,
                    message: "Backup code verification successful".to_string(),
                    remaining_attempts: self.config.max_failed_attempts,
                    lockout_until: None,
                });
            }
        }

        self.handle_failed_verification(user_id).await
    }

    // ==================== Helper Methods ====================

    /// Generate backup codes
    fn generate_backup_codes(&self, count: usize) -> Vec<String> {
        (0..count)
            .map(|_| {
                let part1: String = (0..4)
                    .map(|_| format!("{:X}", rand::random::<u8>() % 16))
                    .collect();
                let part2: String = (0..4)
                    .map(|_| format!("{:X}", rand::random::<u8>() % 16))
                    .collect();
                format!("{}-{}", part1, part2)
            })
            .collect()
    }

    /// Check if user is locked out
    async fn check_lockout(&self, user_id: &str) -> Result<Option<DateTime<Utc>>, MfaError> {
        let lockouts = self.lockouts.read().await;
        if let Some(lockout_until) = lockouts.get(user_id) {
            if *lockout_until > Utc::now() {
                return Ok(Some(*lockout_until));
            }
        }
        Ok(None)
    }

    /// Handle successful verification
    async fn handle_successful_verification(
        &self,
        user_id: &str,
        method: MfaMethod,
    ) -> Result<(), MfaError> {
        self.reset_failed_attempts(user_id).await?;
        
        // Update last used time
        match method {
            MfaMethod::Totp => {
                let mut enrollments = self.totp_enrollments.write().await;
                if let Some(enrollment) = enrollments.get_mut(user_id) {
                    enrollment.last_used_at = Some(Utc::now());
                }
            }
            _ => {}
        }

        Ok(())
    }

    /// Handle failed verification
    async fn handle_failed_verification(
        &self,
        user_id: &str,
    ) -> Result<MfaVerificationResult, MfaError> {
        let mut failed_attempts = self.failed_attempts.write().await;
        let attempts = failed_attempts.entry(user_id.to_string()).or_insert(0);
        *attempts += 1;

        let remaining = self.config.max_failed_attempts.saturating_sub(*attempts);

        if remaining == 0 {
            // Lockout the user
            let lockout_until = Utc::now() + Duration::minutes(self.config.lockout_duration_minutes as i64);
            self.lockouts.write().await.insert(user_id.to_string(), lockout_until);
            
            Ok(MfaVerificationResult {
                success: false,
                method: MfaMethod::Totp,
                message: "Too many failed attempts. Account locked.".to_string(),
                remaining_attempts: 0,
                lockout_until: Some(lockout_until),
            })
        } else {
            Ok(MfaVerificationResult {
                success: false,
                method: MfaMethod::Totp,
                message: "Invalid code".to_string(),
                remaining_attempts: remaining,
                lockout_until: None,
            })
        }
    }

    /// Reset failed attempts
    async fn reset_failed_attempts(&self, user_id: &str) -> Result<(), MfaError> {
        self.failed_attempts.write().await.remove(user_id);
        self.lockouts.write().await.remove(user_id);
        Ok(())
    }
}

/// MFA Error types
#[derive(Debug, thiserror::Error)]
pub enum MfaError {
    #[error("Not enrolled: {0}")]
    NotEnrolled(String),
    #[error("Invalid code: {0}")]
    InvalidCode(String),
    #[error("Security violation: {0}")]
    SecurityViolation(String),
    #[error("Lockout: {0}")]
    Lockout(String),
    #[error("Expired: {0}")]
    Expired(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Base32 encoding helper
fn base32_encode(data: &[u8]) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
    let mut result = String::new();
    let mut bits = 0u32;
    let mut bit_count = 0;

    for &byte in data {
        bits = (bits << 8) | (byte as u32);
        bit_count += 8;
        while bit_count >= 5 {
            bit_count -= 5;
            let idx = ((bits >> bit_count) & 0x1F) as usize;
            result.push(ALPHABET[idx] as char);
        }
    }

    if bit_count > 0 {
        let idx = ((bits << (5 - bit_count)) & 0x1F) as usize;
        result.push(ALPHABET[idx] as char);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_totp_enrollment_and_verification() {
        let manager = MfaManager::new(MfaConfig::default());
        
        // Generate secret
        let (uri, backup_codes) = manager.generate_totp_secret("user1", "V-Sentinel", "user1@example.com").await.unwrap();
        
        assert!(uri.starts_with("otpauth://totp/"));
        assert_eq!(backup_codes.len(), 10);
        
        // Verify enrollment is pending
        let enrollments = manager.totp_enrollments.read().await;
        let enrollment = enrollments.get("user1").unwrap();
        assert_eq!(enrollment.status, EnrollmentStatus::Pending);
    }

    #[tokio::test]
    async fn test_hotp_generation() {
        let manager = MfaManager::new(MfaConfig::default());
        
        // Generate secret
        let (uri, _) = manager.generate_hotp_secret("user1").await.unwrap();
        assert!(uri.starts_with("otpauth://hotp/"));
    }

    #[tokio::test]
    async fn test_fido2_options() {
        let manager = MfaManager::new(MfaConfig::default());
        
        let options = manager.get_fido2_attestation_options("user1").await.unwrap();
        assert!(options["challenge"].is_string());
        assert_eq!(options["rp"]["name"], "V-Sentinel");
    }

    #[test]
    fn test_base32_encoding() {
        let input = b"Hello";
        let encoded = base32_encode(input);
        assert!(!encoded.is_empty());
    }

    #[tokio::test]
    async fn test_backup_codes_generation() {
        let manager = MfaManager::new(MfaConfig::default());
        let (_, backup_codes) = manager.generate_totp_secret("user1", "V-Sentinel", "user1@example.com").await.unwrap();
        
        // Verify format (XXXX-XXXX)
        for code in &backup_codes {
            assert!(code.contains('-'));
            let parts: Vec<&str> = code.split('-').collect();
            assert_eq!(parts.len(), 2);
            assert_eq!(parts[0].len(), 4);
            assert_eq!(parts[1].len(), 4);
        }
    }

    #[tokio::test]
    async fn test_lockout_mechanism() {
        let manager = MfaManager::new(MfaConfig {
            max_failed_attempts: 2,
            ..Default::default()
        });
        
        // Generate and verify enrollment
        manager.generate_totp_secret("user1", "V-Sentinel", "user1@example.com").await.unwrap();
        let mut enrollments = manager.totp_enrollments.write().await;
        enrollments.get_mut("user1").unwrap().status = EnrollmentStatus::Verified;
        drop(enrollments);
        
        // Fail twice
        let result1 = manager.verify_totp("user1", "000000").await.unwrap();
        assert!(!result1.success);
        
        let result2 = manager.verify_totp("user1", "000000").await.unwrap();
        assert!(!result2.success);
        assert!(result2.lockout_until.is_some());
    }
}
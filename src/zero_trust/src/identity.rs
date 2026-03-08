//! Identity Fabric Module
//! 
//! Implements unified identity management following Zero Trust principles.
//! Provides single source of truth for all identity-related operations.

use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use tracing::{info, debug, warn};
use chrono::{DateTime, Utc, Duration};
use async_trait::async_trait;

use super::Subject;

/// Identity Fabric
/// 
/// Central identity management system that integrates multiple identity providers
/// and provides unified identity services
pub struct IdentityFabric {
    identities: HashMap<String, Identity>,
    providers: HashMap<String, Box<dyn IdentityProvider + Send + Sync>>,
    identity_links: HashMap<String, Vec<String>>, // Links between identities
    sessions: HashMap<String, IdentitySession>,
    config: IdentityConfig,
}

/// Identity configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityConfig {
    /// Session timeout in seconds
    pub session_timeout_secs: u64,
    /// Enable identity synchronization
    pub sync_enabled: bool,
    /// Sync interval in seconds
    pub sync_interval_secs: u64,
    /// Enable identity proofing
    pub proofing_enabled: bool,
    /// Minimum identity assurance level
    pub min_assurance_level: AssuranceLevel,
    /// Enable continuous identity verification
    pub continuous_verification: bool,
}

impl Default for IdentityConfig {
    fn default() -> Self {
        Self {
            session_timeout_secs: 28800,
            sync_enabled: true,
            sync_interval_secs: 300,
            proofing_enabled: true,
            min_assurance_level: AssuranceLevel::Low,
            continuous_verification: true,
        }
    }
}

/// Identity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    /// Unique identity identifier
    pub id: String,
    /// Primary identifier (email, username, etc.)
    pub primary_identifier: String,
    /// Identity type
    pub identity_type: IdentityType,
    /// Display name
    pub display_name: String,
    /// Email addresses
    pub emails: Vec<EmailAddress>,
    /// Phone numbers
    pub phones: Vec<PhoneNumber>,
    /// Attributes
    pub attributes: HashMap<String, serde_json::Value>,
    /// Credentials
    pub credentials: Vec<Credential>,
    /// Roles
    pub roles: Vec<String>,
    /// Groups
    pub groups: Vec<String>,
    /// Assurance level
    pub assurance_level: AssuranceLevel,
    /// Identity provider
    pub provider: String,
    /// Status
    pub status: IdentityStatus,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Updated at
    pub updated_at: DateTime<Utc>,
    /// Last authenticated
    pub last_authenticated: Option<DateTime<Utc>>,
    /// Risk score
    pub risk_score: f64,
}

/// Identity types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IdentityType {
    Human,
    Service,
    Device,
    Application,
    Workload,
}

/// Email address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailAddress {
    pub address: String,
    pub is_primary: bool,
    pub is_verified: bool,
    pub verified_at: Option<DateTime<Utc>>,
}

/// Phone number
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhoneNumber {
    pub number: String,
    pub is_primary: bool,
    pub is_verified: bool,
    pub verified_at: Option<DateTime<Utc>>,
    pub phone_type: PhoneType,
}

/// Phone types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PhoneType {
    Mobile,
    Home,
    Work,
    Other,
}

/// Credential
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credential {
    pub id: String,
    pub credential_type: CredentialType,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub last_used: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub auth_methods: Vec<super::AuthMethod>,
}

/// Credential types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CredentialType {
    Password,
    Certificate,
    FIDO2,
    TOTP,
    WebAuthn,
    APIKey,
    OAuth2Token,
    SamlAssertion,
    Biometric,
}

/// Assurance levels (NIST SP 800-63)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum AssuranceLevel {
    None = 0,
    Low = 1,
    Substantial = 2,
    High = 3,
}

/// Identity status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IdentityStatus {
    Active,
    Inactive,
    Suspended,
    Locked,
    Pending,
    Deleted,
}

/// Identity session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentitySession {
    pub id: String,
    pub identity_id: String,
    pub provider: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub status: SessionStatus,
    pub auth_context: AuthContext,
    pub device_id: Option<String>,
    pub ip_address: String,
}

/// Session status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionStatus {
    Active,
    Expired,
    Terminated,
    Suspended,
}

/// Authentication context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthContext {
    pub auth_time: DateTime<Utc>,
    pub auth_methods: Vec<super::AuthMethod>,
    pub assurance_level: AssuranceLevel,
    pub acr_values: Vec<String>,
    pub amr_values: Vec<String>,
}

/// Identity provider trait
#[async_trait::async_trait]
pub trait IdentityProvider {
    /// Provider identifier
    fn id(&self) -> &str;
    /// Provider name
    fn name(&self) -> &str;
    /// Provider type
    fn provider_type(&self) -> ProviderType;
    /// Authenticate identity
    async fn authenticate(&self, credentials: &ProviderCredentials) -> Result<ProviderIdentity>;
    /// Get identity by ID
    async fn get_identity(&self, id: &str) -> Result<Option<ProviderIdentity>>;
    /// Search identities
    async fn search(&self, query: &str) -> Result<Vec<ProviderIdentity>>;
    /// Validate token
    async fn validate_token(&self, token: &str) -> Result<TokenInfo>;
    /// Sync identities
    async fn sync(&self) -> Result<Vec<ProviderIdentity>>;
}

/// Provider types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProviderType {
    Internal,
    LDAP,
    ActiveDirectory,
    Okta,
    Auth0,
    AzureAD,
    GoogleWorkspace,
    PingIdentity,
    Keycloak,
    Custom,
}

/// Provider credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderCredentials {
    pub username: Option<String>,
    pub password: Option<String>,
    pub token: Option<String>,
    pub certificate: Option<Vec<u8>>,
    pub additional: HashMap<String, serde_json::Value>,
}

/// Provider identity information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderIdentity {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
    pub display_name: Option<String>,
    pub groups: Vec<String>,
    pub roles: Vec<String>,
    pub attributes: HashMap<String, serde_json::Value>,
}

/// Token information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    pub subject: String,
    pub issuer: String,
    pub audience: Vec<String>,
    pub expires_at: DateTime<Utc>,
    pub issued_at: DateTime<Utc>,
    pub scopes: Vec<String>,
    pub claims: HashMap<String, serde_json::Value>,
}

impl IdentityFabric {
    /// Create a new Identity Fabric
    pub fn new() -> Self {
        Self {
            identities: HashMap::new(),
            providers: HashMap::new(),
            identity_links: HashMap::new(),
            sessions: HashMap::new(),
            config: IdentityConfig::default(),
        }
    }

    /// Create with custom configuration
    pub fn with_config(config: IdentityConfig) -> Self {
        Self {
            identities: HashMap::new(),
            providers: HashMap::new(),
            identity_links: HashMap::new(),
            sessions: HashMap::new(),
            config,
        }
    }

    /// Register an identity provider
    pub fn register_provider(&mut self, provider: Box<dyn IdentityProvider + Send + Sync>) {
        info!("Registering identity provider: {}", provider.id());
        self.providers.insert(provider.id().to_string(), provider);
    }

    /// Create a new identity
    pub fn create_identity(&mut self, identity: Identity) -> Result<()> {
        if self.identities.contains_key(&identity.id) {
            return Err(anyhow!("Identity with ID {} already exists", identity.id));
        }
        
        info!("Creating identity: {} ({})", identity.display_name, identity.id);
        self.identities.insert(identity.id.clone(), identity);
        Ok(())
    }

    /// Get identity by ID
    pub fn get_identity(&self, id: &str) -> Option<&Identity> {
        self.identities.get(id)
    }

    /// Get identity by email
    pub fn get_identity_by_email(&self, email: &str) -> Option<&Identity> {
        self.identities.values()
            .find(|i| i.emails.iter().any(|e| e.address == email))
    }

    /// Update identity
    pub fn update_identity(&mut self, identity: Identity) -> Result<()> {
        if !self.identities.contains_key(&identity.id) {
            return Err(anyhow!("Identity {} not found", identity.id));
        }
        
        info!("Updating identity: {}", identity.id);
        self.identities.insert(identity.id.clone(), identity);
        Ok(())
    }

    /// Delete identity
    pub fn delete_identity(&mut self, id: &str) -> Result<()> {
        if self.identities.remove(id).is_none() {
            return Err(anyhow!("Identity {} not found", id));
        }
        
        // Clean up sessions
        self.sessions.retain(|_, s| s.identity_id != id);
        
        // Clean up links
        self.identity_links.remove(id);
        for links in self.identity_links.values_mut() {
            links.retain(|l| l != id);
        }
        
        info!("Deleted identity: {}", id);
        Ok(())
    }

    /// Verify identity (for Zero Trust)
    pub async fn verify(&self, subject: &Subject) -> Result<bool> {
        // Check if identity exists and is active
        let identity = match self.identities.get(&subject.id) {
            Some(id) => id,
            None => {
                debug!("Identity {} not found", subject.id);
                return Ok(false);
            }
        };
        
        if identity.status != IdentityStatus::Active {
            warn!("Identity {} is not active: {:?}", subject.id, identity.status);
            return Ok(false);
        }
        
        // Check assurance level
        if identity.assurance_level < self.config.min_assurance_level {
            warn!("Identity {} assurance level {:?} below minimum {:?}",
                  subject.id, identity.assurance_level, self.config.min_assurance_level);
            return Ok(false);
        }
        
        // Check if there's an active session
        let has_active_session = self.sessions.values()
            .any(|s| s.identity_id == subject.id && s.status == SessionStatus::Active && s.expires_at > Utc::now());
        
        if !has_active_session {
            debug!("No active session for identity {}", subject.id);
            // Still allow if identity is valid - session may be established later
        }
        
        Ok(true)
    }

    /// Authenticate identity
    pub async fn authenticate(
        &mut self,
        provider_id: &str,
        credentials: &ProviderCredentials,
    ) -> Result<IdentitySession> {
        let provider = self.providers.get(provider_id)
            .ok_or_else(|| anyhow!("Provider {} not found", provider_id))?;
        
        // Authenticate with provider
        let provider_identity = provider.authenticate(credentials).await?;
        
        // Get or create local identity
        let identity = self.get_or_create_identity(&provider_identity, provider_id).await?;
        
        // Create session
        let session = IdentitySession {
            id: uuid::Uuid::new_v4().to_string(),
            identity_id: identity.id.clone(),
            provider: provider_id.to_string(),
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::seconds(self.config.session_timeout_secs as i64),
            last_activity: Utc::now(),
            status: SessionStatus::Active,
            auth_context: AuthContext {
                auth_time: Utc::now(),
                auth_methods: vec![super::AuthMethod::Password],
                assurance_level: identity.assurance_level,
                acr_values: vec![],
                amr_values: vec![],
            },
            device_id: None,
            ip_address: String::new(),
        };
        
        // Update identity last authenticated
        if let Some(id) = self.identities.get_mut(&identity.id) {
            id.last_authenticated = Some(Utc::now());
        }
        
        self.sessions.insert(session.id.clone(), session.clone());
        
        info!("Created session {} for identity {}", session.id, identity.id);
        Ok(session)
    }

    /// Get or create identity from provider identity
    async fn get_or_create_identity(
        &mut self,
        provider_identity: &ProviderIdentity,
        provider_id: &str,
    ) -> Result<Identity> {
        // Try to find existing identity
        if let Some(existing) = self.identities.get(&provider_identity.id) {
            return Ok(existing.clone());
        }
        
        // Create new identity
        let identity = Identity {
            id: provider_identity.id.clone(),
            primary_identifier: provider_identity.username.clone(),
            identity_type: IdentityType::Human,
            display_name: provider_identity.display_name.clone().unwrap_or_else(|| provider_identity.username.clone()),
            emails: provider_identity.email.as_ref()
                .map(|e| vec![EmailAddress {
                    address: e.clone(),
                    is_primary: true,
                    is_verified: false,
                    verified_at: None,
                }])
                .unwrap_or_default(),
            phones: vec![],
            attributes: provider_identity.attributes.clone(),
            credentials: vec![],
            roles: provider_identity.roles.clone(),
            groups: provider_identity.groups.clone(),
            assurance_level: AssuranceLevel::Low,
            provider: provider_id.to_string(),
            status: IdentityStatus::Active,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_authenticated: Some(Utc::now()),
            risk_score: 0.0,
        };
        
        self.identities.insert(identity.id.clone(), identity.clone());
        Ok(identity)
    }

    /// Validate session
    pub fn validate_session(&mut self, session_id: &str) -> Result<bool> {
        let session = self.sessions.get_mut(session_id)
            .ok_or_else(|| anyhow!("Session {} not found", session_id))?;
        
        if session.status != SessionStatus::Active {
            return Ok(false);
        }
        
        if session.expires_at < Utc::now() {
            session.status = SessionStatus::Expired;
            return Ok(false);
        }
        
        session.last_activity = Utc::now();
        Ok(true)
    }

    /// Terminate session
    pub fn terminate_session(&mut self, session_id: &str) -> Result<()> {
        if let Some(session) = self.sessions.get_mut(session_id) {
            session.status = SessionStatus::Terminated;
            info!("Terminated session {}", session_id);
        }
        Ok(())
    }

    /// Link identities
    pub fn link_identities(&mut self, identity1: &str, identity2: &str) -> Result<()> {
        if !self.identities.contains_key(identity1) {
            return Err(anyhow!("Identity {} not found", identity1));
        }
        if !self.identities.contains_key(identity2) {
            return Err(anyhow!("Identity {} not found", identity2));
        }
        
        self.identity_links
            .entry(identity1.to_string())
            .or_insert_with(Vec::new)
            .push(identity2.to_string());
        
        self.identity_links
            .entry(identity2.to_string())
            .or_insert_with(Vec::new)
            .push(identity1.to_string());
        
        info!("Linked identities {} and {}", identity1, identity2);
        Ok(())
    }

    /// Get linked identities
    pub fn get_linked_identities(&self, identity_id: &str) -> Vec<&Identity> {
        self.identity_links
            .get(identity_id)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.identities.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Sync identities from all providers
    pub async fn sync_all(&mut self) -> Result<usize> {
        if !self.config.sync_enabled {
            return Ok(0);
        }
        
        let mut total_synced = 0;
        
        // Collect provider IDs first to avoid borrow conflicts
        let provider_ids: Vec<String> = self.providers.keys().cloned().collect();
        
        for provider_id in provider_ids {
            let identities = match self.providers.get(&provider_id) {
                Some(provider) => provider.sync().await,
                None => continue,
            };
            
            match identities {
                Ok(identities) => {
                    for pi in identities {
                        if let Ok(_identity) = self.get_or_create_identity(&pi, &provider_id).await {
                            total_synced += 1;
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to sync from provider {}: {}", provider_id, e);
                }
            }
        }
        
        info!("Synced {} identities from all providers", total_synced);
        Ok(total_synced)
    }

    /// Get identity count
    pub fn identity_count(&self) -> usize {
        self.identities.len()
    }

    /// Get active session count
    pub fn active_session_count(&self) -> usize {
        self.sessions.values()
            .filter(|s| s.status == SessionStatus::Active && s.expires_at > Utc::now())
            .count()
    }

    /// List all identities
    pub fn list_identities(&self) -> Vec<&Identity> {
        self.identities.values().collect()
    }
}

impl Default for IdentityFabric {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_fabric_creation() {
        let fabric = IdentityFabric::new();
        assert!(fabric.identities.is_empty());
    }

    #[test]
    fn test_create_identity() {
        let mut fabric = IdentityFabric::new();
        let identity = Identity {
            id: "user-123".to_string(),
            primary_identifier: "testuser".to_string(),
            identity_type: IdentityType::Human,
            display_name: "Test User".to_string(),
            emails: vec![EmailAddress {
                address: "test@example.com".to_string(),
                is_primary: true,
                is_verified: true,
                verified_at: Some(Utc::now()),
            }],
            phones: vec![],
            attributes: HashMap::new(),
            credentials: vec![],
            roles: vec!["user".to_string()],
            groups: vec![],
            assurance_level: AssuranceLevel::Substantial,
            provider: "internal".to_string(),
            status: IdentityStatus::Active,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_authenticated: None,
            risk_score: 0.0,
        };
        
        fabric.create_identity(identity).unwrap();
        assert_eq!(fabric.identity_count(), 1);
    }
}
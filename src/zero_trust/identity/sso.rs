//! Single Sign-On (SSO) Integration Module
//! 
//! This module provides SSO capabilities including:
//! - SAML 2.0 integration
//! - OAuth 2.0 / OpenID Connect flows
//! - Just-in-time provisioning
//! - Session federation

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc, Duration};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use sha2::{Sha256, Digest};

/// SSO Manager
pub struct SsoManager {
    /// SAML providers
    saml_providers: Arc<RwLock<HashMap<String, SamlProvider>>>,
    /// OAuth providers
    oauth_providers: Arc<RwLock<HashMap<String, OAuthProvider>>>,
    /// Active SSO sessions
    sessions: Arc<RwLock<HashMap<String, SsoSession>>>,
    /// Service provider configuration
    sp_config: ServiceProviderConfig,
    /// JIT provisioning config
    jit_config: JitProvisioningConfig,
}

impl SsoManager {
    /// Create a new SSO manager
    pub fn new(sp_config: ServiceProviderConfig) -> Self {
        Self {
            saml_providers: Arc::new(RwLock::new(HashMap::new())),
            oauth_providers: Arc::new(RwLock::new(HashMap::new())),
            sessions: Arc::new(RwLock::new(HashMap::new())),
            sp_config,
            jit_config: JitProvisioningConfig::default(),
        }
    }

    /// Register a SAML provider
    pub async fn register_saml_provider(&self, provider: SamlProvider) {
        self.saml_providers.write().await.insert(provider.id.clone(), provider);
    }

    /// Register an OAuth provider
    pub async fn register_oauth_provider(&self, provider: OAuthProvider) {
        self.oauth_providers.write().await.insert(provider.id.clone(), provider);
    }

    /// Initiate SAML SSO
    pub async fn initiate_saml_sso(
        &self,
        provider_id: &str,
        relay_state: Option<String>,
    ) -> Result<SamlAuthnRequest, SsoError> {
        let providers = self.saml_providers.read().await;
        let provider = providers.get(provider_id)
            .ok_or_else(|| SsoError::ProviderNotFound(provider_id.to_string()))?;

        // Generate AuthnRequest
        let request_id = format!("id_{}", uuid::Uuid::new_v4());
        let issue_instant = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();

        let authn_request = format!(
            r#"<samlp:AuthnRequest xmlns:samlp="urn:oasis:names:tc:SAML:2.0:protocol"
                ID="{}"
                Version="2.0"
                IssueInstant="{}"
                ProtocolBinding="urn:oasis:names:tc:SAML:2.0:bindings:HTTP-POST"
                AssertionConsumerServiceURL="{}">
                <saml:Issuer xmlns:saml="urn:oasis:names:tc:SAML:2.0:assertion">{}</saml:Issuer>
                <samlp:NameIDPolicy Format="urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress"
                    AllowCreate="true"/>
            </samlp:AuthnRequest>"#,
            request_id,
            issue_instant,
            self.sp_config.acs_url,
            self.sp_config.entity_id
        );

        let encoded_request = BASE64.encode(authn_request.as_bytes());

        Ok(SamlAuthnRequest {
            request_id,
            sso_url: provider.sso_url.clone(),
            relay_state,
            saml_request: encoded_request,
        })
    }

    /// Process SAML response
    pub async fn process_saml_response(
        &self,
        provider_id: &str,
        saml_response: &str,
    ) -> Result<SsoSession, SsoError> {
        let providers = self.saml_providers.read().await;
        let provider = providers.get(provider_id)
            .ok_or_else(|| SsoError::ProviderNotFound(provider_id.to_string()))?;

        // Decode and parse SAML response
        let decoded = BASE64.decode(saml_response)
            .map_err(|e| SsoError::InvalidResponse(format!("Base64 decode error: {}", e)))?;

        let response_str = String::from_utf8(decoded)
            .map_err(|e| SsoError::InvalidResponse(format!("UTF-8 decode error: {}", e)))?;

        // Extract attributes (simplified - production would use proper XML parsing)
        let attributes = self.extract_saml_attributes(&response_str)?;

        // Verify signature (placeholder - would use proper signature verification)
        if provider.want_assertions_signed {
            // Verify XML signature
        }

        // Create session
        let session = SsoSession {
            session_id: uuid::Uuid::new_v4().to_string(),
            user_id: attributes.get("name_id").cloned().unwrap_or_default(),
            provider_id: provider_id.to_string(),
            provider_type: SsoProviderType::Saml,
            attributes: attributes.clone(),
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::hours(8),
            is_valid: true,
        };

        // Store session
        self.sessions.write().await.insert(session.session_id.clone(), session.clone());

        Ok(session)
    }

    /// Extract attributes from SAML response (simplified)
    fn extract_saml_attributes(&self, response: &str) -> Result<HashMap<String, String>, SsoError> {
        let mut attributes = HashMap::new();

        // Simple extraction (production would use proper XML parsing)
        if let Some(start) = response.find("<saml:NameID>") {
            if let Some(end) = response[start..].find("</saml:NameID>") {
                let name_id = &response[start + 13..start + end];
                attributes.insert("name_id".to_string(), name_id.to_string());
            }
        }

        // Extract common attributes
        for attr_name in &["email", "firstName", "lastName", "groups", "department"] {
            let pattern = format!("&quot;{}&quot;", attr_name);
            if response.contains(&pattern) {
                // Would extract value properly
            }
        }

        Ok(attributes)
    }

    /// Initiate OAuth 2.0 authorization code flow
    pub async fn initiate_oauth_authorization(
        &self,
        provider_id: &str,
        scope: &[String],
        state: Option<String>,
        pkce_challenge: Option<String>,
    ) -> Result<OAuthAuthorizationUrl, SsoError> {
        let providers = self.oauth_providers.read().await;
        let provider = providers.get(provider_id)
            .ok_or_else(|| SsoError::ProviderNotFound(provider_id.to_string()))?;

        let state = state.unwrap_or_else(|| {
            let mut hasher = Sha256::new();
            hasher.update(uuid::Uuid::new_v4().as_bytes());
            format!("{:x}", hasher.finalize())
        });

        let scope_str = scope.join(" ");
        let redirect_uri = urlencoding::encode(&self.sp_config.oauth_redirect_url).to_string();

        let mut auth_url = format!(
            "{}?response_type=code&client_id={}&redirect_uri={}&scope={}&state={}",
            provider.authorization_url,
            provider.client_id,
            redirect_uri,
            urlencoding::encode(&scope_str),
            state
        );

        if let Some(challenge) = pkce_challenge {
            auth_url.push_str(&format!("&code_challenge={}&code_challenge_method=S256", challenge));
        }

        Ok(OAuthAuthorizationUrl {
            url: auth_url,
            state,
        })
    }

    /// Exchange OAuth authorization code for tokens
    pub async fn exchange_oauth_code(
        &self,
        provider_id: &str,
        code: &str,
        code_verifier: Option<&str>,
    ) -> Result<OAuthTokens, SsoError> {
        let providers = self.oauth_providers.read().await;
        let provider = providers.get(provider_id)
            .ok_or_else(|| SsoError::ProviderNotFound(provider_id.to_string()))?;

        // In production, this would make an HTTP POST request
        // For now, return mock tokens
        Ok(OAuthTokens {
            access_token: format!("access_{}", uuid::Uuid::new_v4()),
            refresh_token: Some(format!("refresh_{}", uuid::Uuid::new_v4())),
            id_token: Some(format!("id_{}", uuid::Uuid::new_v4())),
            token_type: "Bearer".to_string(),
            expires_in: 3600,
            scope: vec!["openid".to_string(), "profile".to_string(), "email".to_string()],
        })
    }

    /// Get user info from OAuth provider
    pub async fn get_user_info(
        &self,
        provider_id: &str,
        access_token: &str,
    ) -> Result<OAuthUserInfo, SsoError> {
        let providers = self.oauth_providers.read().await;
        let provider = providers.get(provider_id)
            .ok_or_else(|| SsoError::ProviderNotFound(provider_id.to_string()))?;

        // In production, this would make an HTTP GET request to provider.userinfo_url
        Ok(OAuthUserInfo {
            sub: uuid::Uuid::new_v4().to_string(),
            email: Some("user@example.com".to_string()),
            email_verified: Some(true),
            name: Some("Test User".to_string()),
            given_name: Some("Test".to_string()),
            family_name: Some("User".to_string()),
            picture: None,
            locale: Some("en".to_string()),
            custom_attributes: HashMap::new(),
        })
    }

    /// Create SSO session from OAuth tokens
    pub async fn create_oauth_session(
        &self,
        provider_id: &str,
        user_info: OAuthUserInfo,
    ) -> Result<SsoSession, SsoError> {
        let mut attributes = HashMap::new();
        attributes.insert("sub".to_string(), user_info.sub.clone());
        if let Some(email) = user_info.email {
            attributes.insert("email".to_string(), email);
        }
        if let Some(name) = user_info.name {
            attributes.insert("name".to_string(), name);
        }

        let session = SsoSession {
            session_id: uuid::Uuid::new_v4().to_string(),
            user_id: user_info.sub,
            provider_id: provider_id.to_string(),
            provider_type: SsoProviderType::OAuth,
            attributes,
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::hours(8),
            is_valid: true,
        };

        self.sessions.write().await.insert(session.session_id.clone(), session.clone());
        Ok(session)
    }

    /// Validate SSO session
    pub async fn validate_session(&self, session_id: &str) -> Result<SsoSession, SsoError> {
        let sessions = self.sessions.read().await;
        let session = sessions.get(session_id)
            .ok_or_else(|| SsoError::SessionNotFound(session_id.to_string()))?;

        if !session.is_valid {
            return Err(SsoError::SessionInvalid);
        }

        if session.expires_at < Utc::now() {
            return Err(SsoError::SessionExpired);
        }

        Ok(session.clone())
    }

    /// Logout from SSO session
    pub async fn logout(&self, session_id: &str) -> Result<(), SsoError> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(session_id) {
            session.is_valid = false;
        }
        Ok(())
    }

    /// Get session by ID
    pub async fn get_session(&self, session_id: &str) -> Option<SsoSession> {
        self.sessions.read().await.get(session_id).cloned()
    }
}

/// Service Provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceProviderConfig {
    /// Entity ID
    pub entity_id: String,
    /// Assertion Consumer Service URL
    pub acs_url: String,
    /// OAuth redirect URL
    pub oauth_redirect_url: String,
    /// Certificate (for signing)
    pub certificate: Option<String>,
    /// Private key (for signing)
    pub private_key: Option<String>,
}

/// SAML Provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamlProvider {
    /// Provider ID
    pub id: String,
    /// Provider name
    pub name: String,
    /// Entity ID
    pub entity_id: String,
    /// SSO URL
    pub sso_url: String,
    /// SLO URL
    pub slo_url: Option<String>,
    /// X.509 certificate
    pub certificate: String,
    /// Want assertions signed
    pub want_assertions_signed: bool,
    /// Want response signed
    pub want_response_signed: bool,
    /// Attribute mapping
    pub attribute_mapping: HashMap<String, String>,
}

impl SamlProvider {
    /// Create a new SAML provider
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            entity_id: String::new(),
            sso_url: String::new(),
            slo_url: None,
            certificate: String::new(),
            want_assertions_signed: true,
            want_response_signed: true,
            attribute_mapping: HashMap::new(),
        }
    }

    /// Set entity ID
    pub fn with_entity_id(mut self, entity_id: impl Into<String>) -> Self {
        self.entity_id = entity_id.into();
        self
    }

    /// Set SSO URL
    pub fn with_sso_url(mut self, sso_url: impl Into<String>) -> Self {
        self.sso_url = sso_url.into();
        self
    }

    /// Set certificate
    pub fn with_certificate(mut self, cert: impl Into<String>) -> Self {
        self.certificate = cert.into();
        self
    }
}

/// OAuth/OIDC Provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthProvider {
    /// Provider ID
    pub id: String,
    /// Provider name
    pub name: String,
    /// Client ID
    pub client_id: String,
    /// Client secret
    pub client_secret: String,
    /// Authorization URL
    pub authorization_url: String,
    /// Token URL
    pub token_url: String,
    /// Userinfo URL
    pub userinfo_url: String,
    /// JWKS URL (for OIDC)
    pub jwks_url: Option<String>,
    /// Issuer URL
    pub issuer: String,
    /// Default scopes
    pub default_scopes: Vec<String>,
    /// PKCE enabled
    pub pkce_enabled: bool,
}

impl OAuthProvider {
    /// Create a new OAuth provider
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            authorization_url: String::new(),
            token_url: String::new(),
            userinfo_url: String::new(),
            jwks_url: None,
            issuer: String::new(),
            default_scopes: vec!["openid".to_string(), "profile".to_string(), "email".to_string()],
            pkce_enabled: true,
        }
    }

    /// Set authorization URL
    pub fn with_authorization_url(mut self, url: impl Into<String>) -> Self {
        self.authorization_url = url.into();
        self
    }

    /// Set token URL
    pub fn with_token_url(mut self, url: impl Into<String>) -> Self {
        self.token_url = url.into();
        self
    }

    /// Set userinfo URL
    pub fn with_userinfo_url(mut self, url: impl Into<String>) -> Self {
        self.userinfo_url = url.into();
        self
    }

    /// Set issuer
    pub fn with_issuer(mut self, issuer: impl Into<String>) -> Self {
        self.issuer = issuer.into();
        self
    }
}

/// SSO Session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SsoSession {
    /// Session ID
    pub session_id: String,
    /// User ID
    pub user_id: String,
    /// Provider ID
    pub provider_id: String,
    /// Provider type
    pub provider_type: SsoProviderType,
    /// User attributes
    pub attributes: HashMap<String, String>,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Expires at
    pub expires_at: DateTime<Utc>,
    /// Is valid
    pub is_valid: bool,
}

/// SSO Provider type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SsoProviderType {
    Saml,
    OAuth,
    Oidc,
}

/// SAML Authentication Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamlAuthnRequest {
    /// Request ID
    pub request_id: String,
    /// SSO URL
    pub sso_url: String,
    /// Relay state
    pub relay_state: Option<String>,
    /// SAML request (base64 encoded)
    pub saml_request: String,
}

/// OAuth Authorization URL
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthAuthorizationUrl {
    /// Authorization URL
    pub url: String,
    /// State parameter
    pub state: String,
}

/// OAuth Tokens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthTokens {
    /// Access token
    pub access_token: String,
    /// Refresh token
    pub refresh_token: Option<String>,
    /// ID token (OIDC)
    pub id_token: Option<String>,
    /// Token type
    pub token_type: String,
    /// Expires in seconds
    pub expires_in: u32,
    /// Granted scopes
    pub scope: Vec<String>,
}

/// OAuth User Info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthUserInfo {
    /// Subject identifier
    pub sub: String,
    /// Email
    pub email: Option<String>,
    /// Email verified
    pub email_verified: Option<bool>,
    /// Full name
    pub name: Option<String>,
    /// Given name
    pub given_name: Option<String>,
    /// Family name
    pub family_name: Option<String>,
    /// Picture URL
    pub picture: Option<String>,
    /// Locale
    pub locale: Option<String>,
    /// Custom attributes
    pub custom_attributes: HashMap<String, String>,
}

/// JIT Provisioning Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JitProvisioningConfig {
    /// Enabled
    pub enabled: bool,
    /// Auto-provision new users
    pub auto_provision: bool,
    /// Default roles for new users
    pub default_roles: Vec<String>,
    /// Default groups for new users
    pub default_groups: Vec<String>,
    /// Attribute mapping
    pub attribute_mapping: HashMap<String, String>,
    /// Require domain allowlist
    pub domain_allowlist: Vec<String>,
}

impl Default for JitProvisioningConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            auto_provision: true,
            default_roles: vec!["user".to_string()],
            default_groups: vec![],
            attribute_mapping: HashMap::new(),
            domain_allowlist: vec![],
        }
    }
}

/// SSO Error
#[derive(Debug, thiserror::Error)]
pub enum SsoError {
    #[error("Provider not found: {0}")]
    ProviderNotFound(String),
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
    #[error("Session not found: {0}")]
    SessionNotFound(String),
    #[error("Session invalid")]
    SessionInvalid,
    #[error("Session expired")]
    SessionExpired,
    #[error("Signature verification failed: {0}")]
    SignatureVerification(String),
    #[error("Token error: {0}")]
    TokenError(String),
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

mod urlencoding {
    pub fn encode(s: &str) -> String {
        urlencoding::encode(s).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_sso_manager() -> SsoManager {
        SsoManager::new(ServiceProviderConfig {
            entity_id: "https://v-sentinel.local".to_string(),
            acs_url: "https://v-sentinel.local/saml/acs".to_string(),
            oauth_redirect_url: "https://v-sentinel.local/oauth/callback".to_string(),
            certificate: None,
            private_key: None,
        })
    }

    #[tokio::test]
    async fn test_saml_provider_registration() {
        let manager = create_test_sso_manager();
        
        let provider = SamlProvider::new("okta", "Okta")
            .with_entity_id("https://okta.example.com")
            .with_sso_url("https://okta.example.com/sso/saml");

        manager.register_saml_provider(provider).await;
    }

    #[tokio::test]
    async fn test_oauth_provider_registration() {
        let manager = create_test_sso_manager();
        
        let provider = OAuthProvider::new("google", "Google", "client_id", "client_secret")
            .with_authorization_url("https://accounts.google.com/o/oauth2/v2/auth")
            .with_token_url("https://oauth2.googleapis.com/token")
            .with_userinfo_url("https://openidconnect.googleapis.com/v1/userinfo")
            .with_issuer("https://accounts.google.com");

        manager.register_oauth_provider(provider).await;
    }

    #[tokio::test]
    async fn test_saml_sso_initiation() {
        let manager = create_test_sso_manager();
        
        let provider = SamlProvider::new("okta", "Okta")
            .with_sso_url("https://okta.example.com/sso/saml");
        
        manager.register_saml_provider(provider).await;

        let result = manager.initiate_saml_sso("okta", Some("relay".to_string())).await;
        assert!(result.is_ok());

        let request = result.unwrap();
        assert!(request.sso_url.contains("okta"));
        assert!(!request.saml_request.is_empty());
    }

    #[tokio::test]
    async fn test_oauth_authorization_url() {
        let manager = create_test_sso_manager();
        
        let provider = OAuthProvider::new("google", "Google", "client_id", "client_secret")
            .with_authorization_url("https://accounts.google.com/o/oauth2/v2/auth");

        manager.register_oauth_provider(provider).await;

        let result = manager.initiate_oauth_authorization(
            "google",
            &["openid".to_string(), "email".to_string()],
            None,
            None,
        ).await;

        assert!(result.is_ok());
        let auth_url = result.unwrap();
        assert!(auth_url.url.contains("accounts.google.com"));
        assert!(!auth_url.state.is_empty());
    }

    #[tokio::test]
    async fn test_session_validation() {
        let manager = create_test_sso_manager();

        // Create a session manually
        let session = SsoSession {
            session_id: "test-session".to_string(),
            user_id: "user1".to_string(),
            provider_id: "test".to_string(),
            provider_type: SsoProviderType::OAuth,
            attributes: HashMap::new(),
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::hours(1),
            is_valid: true,
        };

        manager.sessions.write().await.insert(session.session_id.clone(), session);

        let result = manager.validate_session("test-session").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_logout() {
        let manager = create_test_sso_manager();

        let session = SsoSession {
            session_id: "test-session".to_string(),
            user_id: "user1".to_string(),
            provider_id: "test".to_string(),
            provider_type: SsoProviderType::OAuth,
            attributes: HashMap::new(),
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::hours(1),
            is_valid: true,
        };

        manager.sessions.write().await.insert(session.session_id.clone(), session);

        manager.logout("test-session").await.unwrap();
        
        let result = manager.validate_session("test-session").await;
        assert!(result.is_err());
    }
}
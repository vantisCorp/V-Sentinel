//! Identity management module for Zero Trust Architecture
//!
//! This module provides comprehensive identity management capabilities including:
//! - Identity Fabric: Unified identity management layer
//! - SSO: Single Sign-On integration (SAML 2.0, OAuth 2.0/OIDC)
//! - Sync: Identity synchronization across systems
//! - Analytics: Identity analytics and reporting

pub mod fabric;
pub mod sso;
pub mod sync;
pub mod analytics;

// Re-export key types from identity fabric
pub use fabric::{IdentityFabric, IdentityProvider, UserInfo};

// Re-export SSO types
pub use sso::{
    SsoManager, SsoConfig, SsoSession, SsoSessionState,
    SamlProvider, SamlConfig, OAuthProvider, OAuthConfig,
    JitProvisioningConfig, ServiceProviderConfig,
};

// Re-export Identity Sync types
pub use sync::{
    IdentitySyncManager, SyncConfig, SyncStatus, IdentityMapping,
    ConflictResolution, ConflictRecord, SyncRecord, SyncDirection,
};

// Re-export Identity Analytics types
pub use analytics::{
    IdentityAnalyticsManager, AnalyticsConfig, AccessEvent,
    UserProfile, RiskScore, AnomalyType, AnomalyRecord,
    AccessPatternReport, ComplianceReport, ComplianceFramework,
};
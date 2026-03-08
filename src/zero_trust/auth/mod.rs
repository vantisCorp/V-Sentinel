//! Authentication Module for Zero Trust
//! 
//! This module provides comprehensive authentication capabilities:
//! - Continuous authentication with session management
//! - Multi-factor authentication (MFA)
//! - Behavioral biometrics
//! - Adaptive authentication based on risk

pub mod continuous;
pub mod mfa;
pub mod biometrics;
pub mod adaptive;

pub use continuous::{ContinuousAuthManager, SessionInfo, AuthState};
pub use mfa::{MfaManager, MfaConfig, MfaMethod, MfaVerificationResult};
pub use biometrics::{BiometricsManager, BiometricsConfig, BiometricAnalysisResult, RiskLevel};
pub use adaptive::{AdaptiveAuthManager, AdaptiveAuthConfig, AdaptiveAuthDecision, AuthRequirement};
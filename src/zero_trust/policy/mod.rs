//! Policy engine module for Zero Trust Architecture
//!
//! This module provides comprehensive policy capabilities including:
//! - Policy Engine: Core policy evaluation and decision making
//! - Policy Language: Enhanced DSL for policy definition
//! - Enforcement Points: API gateway, service mesh, and database enforcement
//! - Validation: Policy testing, simulation, and impact analysis
//! - Audit: Comprehensive audit logging and compliance reporting

pub mod engine;
pub mod language;
pub mod enforcement;
pub mod validation;
pub mod audit;

// Re-export core policy types
pub use engine::{
    PolicyEngine, Policy, PolicyType, PolicyCondition, PolicyAction,
    ConditionType, Operator,
};

// Re-export policy language types
pub use language::{
    PolicyLanguageManager, PolicyLanguageConfig, PolicyExpression,
    PolicyTemplate, PolicyVersion, PolicyFunction, PolicyLanguageError,
};

// Re-export enforcement types
pub use enforcement::{
    EnforcementPointManager, EnforcementConfig, EnforcementStats,
    ApiGatewayEnforcer, ApiGatewayConfig, ApiRequest, ApiEnforcementDecision,
    ServiceMeshEnforcer, ServiceMeshConfig, MeshRequest, MeshEnforcementDecision,
    DatabaseEnforcer, DatabaseConfig, DatabaseRequest, QueryType, DatabaseEnforcementDecision,
    EnforcementError, EnforcementResult,
};

// Re-export validation types
pub use validation::{
    PolicyValidator, ValidationConfig, ValidationResult, ValidationError,
    PolicySimulator, TestScenario, SimulationResults, SimulationSummary,
    ImpactAnalysis, ImpactType, ImpactChange,
    PolicyRegressionTester, RegressionTestResult,
};

// Re-export audit types
pub use audit::{
    PolicyAuditManager, AuditConfig, AuditLogEntry, AuditEventType,
    PolicyChangeRecord, ChangeType,
    ComplianceFramework, ComplianceRequirement, ComplianceRequirementType,
    AuditReport, ReportPeriod, AuditStatistics,
    ComplianceReport, ComplianceStatus, ComplianceItem,
    AuditFilters, AuditQuery, AuditError,
};
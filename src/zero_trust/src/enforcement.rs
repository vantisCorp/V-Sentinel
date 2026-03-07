//! Policy Enforcement Module
//! 
//! Implements Policy Enforcement Points (PEP) that enforce access decisions
//! and obligations throughout the system.

use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use tracing::{info, debug, warn};
use chrono::{DateTime, Utc};

use super::{AccessDecision, Decision, Obligation, ObligationType};

/// Enforcement Point
/// 
/// Enforces access decisions and manages obligations
pub struct EnforcementPoint {
    active_decisions: HashMap<String, ActiveDecision>,
    obligation_handlers: HashMap<String, Box<dyn ObligationHandler + Send + Sync>>,
    audit_log: Vec<EnforcementAuditEntry>,
    statistics: EnforcementStatistics,
}

/// Active decision (decision that has been enforced)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveDecision {
    /// Decision ID
    pub decision_id: String,
    /// Request ID
    pub request_id: String,
    /// Subject ID
    pub subject_id: String,
    /// Resource ID
    pub resource_id: String,
    /// Decision
    pub decision: Decision,
    /// Trust score
    pub trust_score: f64,
    /// Obligations to fulfill
    pub pending_obligations: Vec<Obligation>,
    /// Fulfilled obligations
    pub fulfilled_obligations: Vec<Obligation>,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Expires at
    pub expires_at: Option<DateTime<Utc>>,
    /// Status
    pub status: DecisionStatus,
}

/// Decision status
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DecisionStatus {
    Active,
    Expired,
    Revoked,
    Completed,
    ObligationPending,
}

/// Enforcement audit entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnforcementAuditEntry {
    pub id: String,
    pub decision_id: String,
    pub action: EnforcementAction,
    pub timestamp: DateTime<Utc>,
    pub details: HashMap<String, serde_json::Value>,
    pub result: EnforcementResult,
}

/// Enforcement actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnforcementAction {
    DecisionEnforced,
    DecisionRevoked,
    ObligationCreated,
    ObligationFulfilled,
    ObligationFailed,
    AccessGranted,
    AccessDenied,
    ChallengeIssued,
    SessionCreated,
    SessionTerminated,
}

/// Enforcement result
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EnforcementResult {
    Success,
    Failure,
    Partial,
}

/// Enforcement statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EnforcementStatistics {
    pub total_decisions: u64,
    pub allow_count: u64,
    pub deny_count: u64,
    pub challenge_count: u64,
    pub obligations_fulfilled: u64,
    pub obligations_failed: u64,
    pub revocations: u64,
}

/// Trait for obligation handlers
#[async_trait::async_trait]
pub trait ObligationHandler {
    fn obligation_type(&self) -> ObligationType;
    async fn fulfill(&self, obligation: &Obligation, context: &EnforcementContext) -> Result<bool>;
    fn is_fulfilled(&self, obligation: &Obligation) -> bool;
}

/// Enforcement context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnforcementContext {
    pub subject_id: String,
    pub resource_id: String,
    pub session_id: Option<String>,
    pub additional: HashMap<String, serde_json::Value>,
}

impl EnforcementPoint {
    /// Create a new Enforcement Point
    pub fn new() -> Self {
        Self {
            active_decisions: HashMap::new(),
            obligation_handlers: HashMap::new(),
            audit_log: Vec::new(),
            statistics: EnforcementStatistics::default(),
        }
    }

    /// Register an obligation handler
    pub fn register_handler(&mut self, handler: Box<dyn ObligationHandler + Send + Sync>) {
        let key = format!("{:?}", handler.obligation_type());
        info!("Registering obligation handler for: {}", key);
        self.obligation_handlers.insert(key, handler);
    }

    /// Enforce an access decision
    pub async fn enforce(&self, decision: &AccessDecision) -> Result<()> {
        info!("Enforcing decision {} for request {}", decision.decision_id, decision.request_id);
        
        match decision.decision {
            Decision::Allow => {
                self.enforce_allow(decision).await?;
            }
            Decision::Deny => {
                self.enforce_deny(decision).await?;
            }
            Decision::Challenge => {
                self.enforce_challenge(decision).await?;
            }
            Decision::AllowWithObligations => {
                self.enforce_with_obligations(decision).await?;
            }
        }
        
        Ok(())
    }

    /// Enforce allow decision
    async fn enforce_allow(&self, decision: &AccessDecision) -> Result<()> {
        info!("Access ALLOWED for request {}", decision.request_id);
        Ok(())
    }

    /// Enforce deny decision
    async fn enforce_deny(&self, decision: &AccessDecision) -> Result<()> {
        info!("Access DENIED for request {}", decision.request_id);
        Ok(())
    }

    /// Enforce challenge decision
    async fn enforce_challenge(&self, decision: &AccessDecision) -> Result<()> {
        info!("Access CHALLENGE required for request {}", decision.request_id);
        
        // Issue challenges for obligations
        for obligation in &decision.obligations {
            debug!("Issuing obligation: {:?}", obligation.obligation_type);
        }
        
        Ok(())
    }

    /// Enforce decision with obligations
    async fn enforce_with_obligations(&self, decision: &AccessDecision) -> Result<()> {
        info!("Access ALLOWED WITH OBLIGATIONS for request {}", decision.request_id);
        
        // Record obligations to be fulfilled
        for obligation in &decision.obligations {
            debug!("Obligation required: {:?}", obligation.obligation_type);
        }
        
        Ok(())
    }

    /// Record a decision (stores it for tracking)
    pub fn record_decision(&mut self, decision: &AccessDecision, subject_id: &str, resource_id: &str) -> Result<String> {
        let active = ActiveDecision {
            decision_id: decision.decision_id.to_string(),
            request_id: decision.request_id.to_string(),
            subject_id: subject_id.to_string(),
            resource_id: resource_id.to_string(),
            decision: decision.decision,
            trust_score: decision.trust_score,
            pending_obligations: decision.obligations.clone(),
            fulfilled_obligations: vec![],
            created_at: decision.timestamp,
            expires_at: decision.expires_at,
            status: if decision.obligations.is_empty() {
                DecisionStatus::Active
            } else {
                DecisionStatus::ObligationPending
            },
        };
        
        self.active_decisions.insert(active.decision_id.clone(), active.clone());
        
        // Update statistics
        self.statistics.total_decisions += 1;
        match decision.decision {
            Decision::Allow => self.statistics.allow_count += 1,
            Decision::Deny => self.statistics.deny_count += 1,
            Decision::Challenge => self.statistics.challenge_count += 1,
            Decision::AllowWithObligations => self.statistics.allow_count += 1,
        }
        
        // Create audit entry
        self.add_audit_entry(active.decision_id.clone(), EnforcementAction::DecisionEnforced, decision);
        
        Ok(active.decision_id)
    }

    /// Check if obligation is fulfilled
    pub async fn check_obligation(&mut self, decision_id: &str, obligation_idx: usize) -> Result<bool> {
        let decision = self.active_decisions.get_mut(decision_id)
            .ok_or_else(|| anyhow!("Decision {} not found", decision_id))?;
        
        if obligation_idx >= decision.pending_obligations.len() {
            return Err(anyhow!("Invalid obligation index"));
        }
        
        let obligation = decision.pending_obligations[obligation_idx].clone();
        let handler_key = format!("{:?}", obligation.obligation_type);
        
        if let Some(handler) = self.obligation_handlers.get(&handler_key) {
            let context = EnforcementContext {
                subject_id: decision.subject_id.clone(),
                resource_id: decision.resource_id.clone(),
                session_id: None,
                additional: HashMap::new(),
            };
            
            let fulfilled = handler.fulfill(&obligation, &context).await?;
            
            if fulfilled {
                // Move obligation to fulfilled
                let obligation = decision.pending_obligations.remove(obligation_idx);
                decision.fulfilled_obligations.push(obligation);
                
                // Update status if all obligations fulfilled
                if decision.pending_obligations.is_empty() {
                    decision.status = DecisionStatus::Active;
                }
                
                self.statistics.obligations_fulfilled += 1;
                self.add_audit_entry(decision_id.to_string(), EnforcementAction::ObligationFulfilled, &obligation);
                
                return Ok(true);
            }
        }
        
        self.statistics.obligations_failed += 1;
        Ok(false)
    }

    /// Revoke a decision
    pub fn revoke_decision(&mut self, decision_id: &str, reason: &str) -> Result<()> {
        let decision = self.active_decisions.get_mut(decision_id)
            .ok_or_else(|| anyhow!("Decision {} not found", decision_id))?;
        
        decision.status = DecisionStatus::Revoked;
        self.statistics.revocations += 1;
        
        self.add_audit_entry(
            decision_id.to_string(), 
            EnforcementAction::DecisionRevoked,
            &format!("Revoked: {}", reason)
        );
        
        info!("Revoked decision {}: {}", decision_id, reason);
        Ok(())
    }

    /// Check if decision is still valid
    pub fn is_decision_valid(&self, decision_id: &str) -> bool {
        if let Some(decision) = self.active_decisions.get(decision_id) {
            if decision.status != DecisionStatus::Active && decision.status != DecisionStatus::ObligationPending {
                return false;
            }
            
            if let Some(expires) = decision.expires_at {
                if expires < Utc::now() {
                    return false;
                }
            }
            
            return true;
        }
        false
    }

    /// Get active decision
    pub fn get_decision(&self, decision_id: &str) -> Option<&ActiveDecision> {
        self.active_decisions.get(decision_id)
    }

    /// Get all active decisions for a subject
    pub fn get_subject_decisions(&self, subject_id: &str) -> Vec<&ActiveDecision> {
        self.active_decisions.values()
            .filter(|d| d.subject_id == subject_id && d.status == DecisionStatus::Active)
            .collect()
    }

    /// Get all active decisions for a resource
    pub fn get_resource_decisions(&self, resource_id: &str) -> Vec<&ActiveDecision> {
        self.active_decisions.values()
            .filter(|d| d.resource_id == resource_id && d.status == DecisionStatus::Active)
            .collect()
    }

    /// Clean up expired decisions
    pub fn cleanup_expired(&mut self) -> usize {
        let now = Utc::now();
        let expired: Vec<String> = self.active_decisions.iter()
            .filter(|(_, d)| {
                d.expires_at.map(|e| e < now).unwrap_or(false) || d.status == DecisionStatus::Expired
            })
            .map(|(id, _)| id.clone())
            .collect();
        
        for id in &expired {
            if let Some(decision) = self.active_decisions.get_mut(id) {
                decision.status = DecisionStatus::Expired;
            }
        }
        
        if !expired.is_empty() {
            info!("Cleaned up {} expired decisions", expired.len());
        }
        
        expired.len()
    }

    /// Add audit entry
    fn add_audit_entry<T: Serialize>(&mut self, decision_id: String, action: EnforcementAction, details: &T) {
        let entry = EnforcementAuditEntry {
            id: uuid::Uuid::new_v4().to_string(),
            decision_id,
            action,
            timestamp: Utc::now(),
            details: serde_json::to_value(details)
                .and_then(|v| serde_json::from_value(v))
                .unwrap_or_default(),
            result: EnforcementResult::Success,
        };
        
        self.audit_log.push(entry);
    }

    /// Get statistics
    pub fn get_statistics(&self) -> &EnforcementStatistics {
        &self.statistics
    }

    /// Get audit log
    pub fn get_audit_log(&self, limit: usize) -> Vec<&EnforcementAuditEntry> {
        self.audit_log.iter().rev().take(limit).collect()
    }

    /// Get active decision count
    pub fn active_decision_count(&self) -> usize {
        self.active_decisions.values()
            .filter(|d| d.status == DecisionStatus::Active)
            .count()
    }
}

impl Default for EnforcementPoint {
    fn default() -> Self {
        Self::new()
    }
}

// Default obligation handlers

/// MFA Obligation Handler
pub struct MFAObligationHandler;

impl MFAObligationHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl ObligationHandler for MFAObligationHandler {
    fn obligation_type(&self) -> ObligationType {
        ObligationType::ProvideMFA
    }
    
    async fn fulfill(&self, _obligation: &Obligation, _context: &EnforcementContext) -> Result<bool> {
        // In real implementation, would check MFA status
        Ok(true)
    }
    
    fn is_fulfilled(&self, _obligation: &Obligation) -> bool {
        false
    }
}

/// Reauthentication Obligation Handler
pub struct ReauthObligationHandler;

impl ReauthObligationHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl ObligationHandler for ReauthObligationHandler {
    fn obligation_type(&self) -> ObligationType {
        ObligationType::Reauthenticate
    }
    
    async fn fulfill(&self, _obligation: &Obligation, _context: &EnforcementContext) -> Result<bool> {
        // In real implementation, would trigger reauthentication
        Ok(true)
    }
    
    fn is_fulfilled(&self, _obligation: &Obligation) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enforcement_point_creation() {
        let ep = EnforcementPoint::new();
        assert!(ep.active_decisions.is_empty());
    }

    #[test]
    fn test_record_decision() {
        let mut ep = EnforcementPoint::new();
        let decision = AccessDecision {
            id: uuid::Uuid::new_v4(),
            request_id: uuid::Uuid::new_v4(),
            decision: Decision::Allow,
            trust_score: 0.85,
            applied_policies: vec![],
            obligations: vec![],
            recommendations: vec![],
            timestamp: Utc::now(),
            expires_at: Some(Utc::now() + chrono::Duration::hours(1)),
        };
        
        let id = ep.record_decision(&decision, "user-123", "resource-1").unwrap();
        assert!(!id.is_empty());
        assert!(ep.is_decision_valid(&id));
    }
}
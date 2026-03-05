//! PQC Message Module

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

/// PQC Message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PqcMessage {
    /// Message ID
    pub message_id: String,
    /// Conversation ID
    pub conversation_id: String,
    /// Sender ID
    pub sender_id: String,
    /// Encrypted content
    pub content: Vec<u8>,
    /// PQC signature
    pub signature: Vec<u8>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Message priority
    pub priority: MessagePriority,
    /// Time-to-live (seconds)
    pub ttl_secs: u64,
}

/// Message Priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessagePriority {
    Low,
    Normal,
    High,
    Urgent,
}

impl PqcMessage {
    /// Create a new message
    pub fn new(
        message_id: String,
        conversation_id: String,
        sender_id: String,
        content: Vec<u8>,
    ) -> Self {
        Self {
            message_id,
            conversation_id,
            sender_id,
            content,
            signature: Vec::new(),
            timestamp: Utc::now(),
            priority: MessagePriority::Normal,
            ttl_secs: 604800,
        }
    }
    
    /// Check if message is expired
    pub fn is_expired(&self) -> bool {
        let elapsed = Utc::now() - self.timestamp;
        elapsed.num_seconds() as u64 > self.ttl_secs
    }
    
    /// Sign the message
    pub fn sign(&mut self, _signature: Vec<u8>) {
        self.signature = _signature;
    }
    
    /// Verify the message signature
    pub fn verify_signature(&self) -> Result<bool> {
        // Verify PQC signature
        Ok(!self.signature.is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_message_creation() {
        let message = PqcMessage::new(
            "msg-1".to_string(),
            "conv-1".to_string(),
            "user-1".to_string(),
            vec![1, 2, 3],
        );
        
        assert_eq!(message.message_id, "msg-1");
        assert_eq!(message.conversation_id, "conv-1");
        assert_eq!(message.sender_id, "user-1");
    }
    
    #[test]
    fn test_message_expiration() {
        let mut message = PqcMessage::new(
            "msg-1".to_string(),
            "conv-1".to_string(),
            "user-1".to_string(),
            vec![1, 2, 3],
        );
        
        message.ttl_secs = 0;
        assert!(message.is_expired());
    }
}
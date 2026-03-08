//! SENTINEL Blockchain Module
//!
//! This module provides blockchain-based threat reputation system,
//! immutable audit logs, smart contracts for security automation,
//! and decentralized consensus for threat intelligence sharing.

use anyhow::{anyhow, Result};
use chrono::Utc;
use rand::{rngs::OsRng, RngCore};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256, Sha512};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

/// Blockchain Manager
pub struct BlockchainManager {
    chain: Arc<RwLock<Vec<Block>>>,
    mempool: Arc<RwLock<Vec<Transaction>>>,
    consensus: Arc<RwLock<ConsensusEngine>>,
    smart_contracts: Arc<RwLock<HashMap<String, SmartContract>>>,
    threat_registry: Arc<RwLock<ThreatRegistry>>,
    wallet: Arc<RwLock<Wallet>>,
    statistics: Arc<RwLock<BlockchainStatistics>>,
}

/// Block in the blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
    pub hash: String,
    pub signature: String,
}

/// Block header
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub version: u32,
    pub height: u64,
    pub timestamp: i64,
    pub previous_hash: String,
    pub merkle_root: String,
    pub nonce: u64,
    pub difficulty: u32,
    pub validator: String,
}

/// Transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub transaction_type: TransactionType,
    pub sender: String,
    pub recipient: String,
    pub payload: Vec<u8>,
    pub timestamp: i64,
    pub signature: String,
    pub gas: u64,
    pub gas_price: u64,
}

/// Transaction Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionType {
    /// Threat intelligence submission
    ThreatSubmission,
    /// Threat reputation update
    ThreatReputation,
    /// Smart contract deployment
    ContractDeploy,
    /// Smart contract execution
    ContractExecute,
    /// Reputation attestation
    Attestation,
    /// Peer registration
    PeerRegister,
    /// Audit log entry
    AuditLog,
}

/// Consensus Engine
pub struct ConsensusEngine {
    algorithm: ConsensusAlgorithm,
    validators: Vec<Validator>,
    current_height: u64,
    current_round: u64,
    votes: HashMap<String, Vec<Vote>>,
}

/// Consensus Algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsensusAlgorithm {
    /// Proof of Authority
    PoA,
    /// Delegated Proof of Stake
    DPoS,
    /// Practical Byzantine Fault Tolerance
    PBFT,
    /// HoneyBadgerBFT
    HoneyBadger,
    /// Tendermint
    Tendermint,
}

/// Validator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Validator {
    pub address: String,
    pub public_key: Vec<u8>,
    pub stake: u64,
    pub reputation: f64,
    pub last_seen: i64,
    pub is_active: bool,
}

/// Vote in consensus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    pub validator: String,
    pub block_hash: String,
    pub round: u64,
    pub signature: String,
    pub decision: bool,
}

/// Smart Contract
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartContract {
    pub address: String,
    pub code: Vec<u8>,
    pub state: HashMap<String, Vec<u8>>,
    pub owner: String,
    pub created_at: i64,
    pub gas_limit: u64,
    pub abi: ContractABI,
}

/// Contract ABI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractABI {
    pub functions: Vec<ContractFunction>,
    pub events: Vec<ContractEvent>,
}

/// Contract Function
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractFunction {
    pub name: String,
    pub inputs: Vec<Parameter>,
    pub outputs: Vec<Parameter>,
    pub payable: bool,
}

/// Contract Event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractEvent {
    pub name: String,
    pub inputs: Vec<Parameter>,
    pub anonymous: bool,
}

/// Parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub param_type: String,
    pub indexed: bool,
}

/// Threat Registry
pub struct ThreatRegistry {
    threats: HashMap<String, ThreatEntry>,
    reputation_scores: HashMap<String, f64>,
    attestations: HashMap<String, Vec<Attestation>>,
}

/// Threat Entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatEntry {
    pub threat_id: String,
    pub threat_type: ThreatType,
    pub severity: Severity,
    pub description: String,
    pub indicators: Vec<Indicator>,
    pub first_seen: i64,
    pub last_seen: i64,
    pub confidence: f64,
    pub submitter: String,
    pub endorsements: u64,
    pub challenges: u64,
}

/// Threat Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ThreatType {
    Malware,
    Ransomware,
    Phishing,
    DDoS,
    Botnet,
    ZeroDay,
    APT,
    CryptoJacking,
    Spyware,
    Trojan,
    Other,
}

/// Severity Levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Informational,
}

/// Indicator of Compromise
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Indicator {
    pub indicator_type: IndicatorType,
    pub value: String,
    pub context: String,
}

/// Indicator Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IndicatorType {
    IPv4,
    IPv6,
    Domain,
    URL,
    HashMD5,
    HashSHA256,
    Email,
    FilePath,
    Registry,
    Mutex,
    Certificate,
}

/// Attestation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attestation {
    pub attestation_id: String,
    pub threat_id: String,
    pub attester: String,
    pub decision: bool,
    pub confidence: f64,
    pub timestamp: i64,
    pub signature: String,
}

/// Wallet for signing transactions
pub struct Wallet {
    private_key: Vec<u8>,
    public_key: Vec<u8>,
    address: String,
}

/// Blockchain Statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainStatistics {
    pub total_blocks: u64,
    pub total_transactions: u64,
    pub total_threats: u64,
    pub total_validators: u64,
    pub average_block_time_ms: f64,
    pub total_gas_used: u64,
    pub chain_health: f64,
}

impl Default for BlockchainStatistics {
    fn default() -> Self {
        Self {
            total_blocks: 0,
            total_transactions: 0,
            total_threats: 0,
            total_validators: 0,
            average_block_time_ms: 0.0,
            total_gas_used: 0,
            chain_health: 100.0,
        }
    }
}

impl BlockchainManager {
    /// Create a new blockchain manager
    pub fn new() -> Result<Self> {
        info!("Creating Blockchain Manager...");

        Ok(Self {
            chain: Arc::new(RwLock::new(Vec::new())),
            mempool: Arc::new(RwLock::new(Vec::new())),
            consensus: Arc::new(RwLock::new(ConsensusEngine::new(ConsensusAlgorithm::PBFT))),
            smart_contracts: Arc::new(RwLock::new(HashMap::new())),
            threat_registry: Arc::new(RwLock::new(ThreatRegistry::new())),
            wallet: Arc::new(RwLock::new(Wallet::new())),
            statistics: Arc::new(RwLock::new(BlockchainStatistics::default())),
        })
    }

    /// Initialize blockchain with genesis block
    pub async fn initialize(&self) -> Result<()> {
        info!("Initializing Blockchain...");

        // Create genesis block
        let genesis = self.create_genesis_block().await?;

        {
            let mut chain = self.chain.write().await;
            chain.push(genesis);
        }

        // Initialize wallet
        {
            let mut wallet = self.wallet.write().await;
            wallet.generate_keys()?;
        }

        info!("Blockchain initialized with genesis block");
        Ok(())
    }

    /// Submit threat intelligence
    pub async fn submit_threat(&self, threat: ThreatEntry) -> Result<String> {
        let wallet = self.wallet.read().await;
        let sender = wallet.address.clone();

        // Create transaction
        let tx = Transaction {
            id: self.generate_tx_id().await,
            transaction_type: TransactionType::ThreatSubmission,
            sender: sender.clone(),
            recipient: "THREAT_REGISTRY".to_string(),
            payload: serde_json::to_vec(&threat)?,
            timestamp: Utc::now().timestamp(),
            signature: wallet.sign(threat.threat_id.as_bytes())?,
            gas: 100_000,
            gas_price: 1,
        };

        // Add to mempool
        {
            let mut mempool = self.mempool.write().await;
            mempool.push(tx);
        }

        info!("Submitted threat: {}", threat.threat_id);

        Ok(threat.threat_id)
    }

    /// Get threat by ID
    pub async fn get_threat(&self, threat_id: &str) -> Result<Option<ThreatEntry>> {
        let registry = self.threat_registry.read().await;
        Ok(registry.threats.get(threat_id).cloned())
    }

    /// Attest to threat
    pub async fn attest_threat(
        &self,
        threat_id: &str,
        is_valid: bool,
        confidence: f64,
    ) -> Result<String> {
        let wallet = self.wallet.read().await;

        let attestation = Attestation {
            attestation_id: self.generate_id().await,
            threat_id: threat_id.to_string(),
            attester: wallet.address.clone(),
            decision: is_valid,
            confidence,
            timestamp: Utc::now().timestamp(),
            signature: wallet.sign(threat_id.as_bytes())?,
        };

        // Update threat registry
        {
            let mut registry = self.threat_registry.write().await;
            if let Some(threat) = registry.threats.get_mut(threat_id) {
                if is_valid {
                    threat.endorsements += 1;
                } else {
                    threat.challenges += 1;
                }
            }
            registry
                .attestations
                .entry(threat_id.to_string())
                .or_default()
                .push(attestation.clone());
        }

        // Update reputation
        self.update_reputation(&wallet.address, is_valid).await?;

        Ok(attestation.attestation_id)
    }

    /// Deploy smart contract
    pub async fn deploy_contract(&self, code: Vec<u8>, abi: ContractABI) -> Result<String> {
        let wallet = self.wallet.read().await;
        let address = self.generate_contract_address().await;

        let contract = SmartContract {
            address: address.clone(),
            code,
            state: HashMap::new(),
            owner: wallet.address.clone(),
            created_at: Utc::now().timestamp(),
            gas_limit: 10_000_000,
            abi,
        };

        {
            let mut contracts = self.smart_contracts.write().await;
            contracts.insert(address.clone(), contract);
        }

        info!("Deployed contract at: {}", address);
        Ok(address)
    }

    /// Execute smart contract
    pub async fn execute_contract(
        &self,
        address: &str,
        function: &str,
        args: Vec<Vec<u8>>,
    ) -> Result<Vec<u8>> {
        let mut contracts = self.smart_contracts.write().await;
        let contract = contracts
            .get_mut(address)
            .ok_or_else(|| anyhow!("Contract not found"))?;

        // Find function in ABI
        let _func = contract
            .abi
            .functions
            .iter()
            .find(|f| f.name == function)
            .ok_or_else(|| anyhow!("Function not found: {}", function))?;

        // Execute contract (simplified)
        let result = self
            .execute_contract_function(contract, function, args)
            .await?;

        Ok(result)
    }

    /// Add block to chain
    pub async fn add_block(&self, transactions: Vec<Transaction>) -> Result<String> {
        let (height, previous_hash) = {
            let chain = self.chain.read().await;
            let previous_block = chain.last().ok_or_else(|| anyhow!("No genesis block"))?;
            (
                previous_block.header.height + 1,
                previous_block.hash.clone(),
            )
        };

        let wallet = self.wallet.read().await;

        let header = BlockHeader {
            version: 1,
            height,
            timestamp: Utc::now().timestamp(),
            previous_hash,
            merkle_root: self.calculate_merkle_root(&transactions).await,
            nonce: 0,
            difficulty: 1,
            validator: wallet.address.clone(),
        };

        let block_hash = self.calculate_block_hash(&header).await;
        let signature = wallet.sign(block_hash.as_bytes())?;

        let block = Block {
            header,
            transactions: transactions.clone(),
            hash: block_hash.clone(),
            signature,
        };

        {
            let mut chain = self.chain.write().await;
            chain.push(block);
        }

        // Process transactions
        self.process_transactions(&transactions).await?;

        {
            let mut stats = self.statistics.write().await;
            stats.total_blocks += 1;
            stats.total_transactions += transactions.len() as u64;
        }

        info!("Added block at height: {}", height);

        Ok(block_hash)
    }

    /// Get block by height
    pub async fn get_block(&self, height: u64) -> Result<Option<Block>> {
        let chain = self.chain.read().await;
        Ok(chain.get(height as usize).cloned())
    }

    /// Get current chain height
    pub async fn get_height(&self) -> u64 {
        let chain = self.chain.read().await;
        chain.len() as u64 - 1
    }

    /// Get threat reputation score
    pub async fn get_reputation(&self, address: &str) -> f64 {
        let registry = self.threat_registry.read().await;
        registry
            .reputation_scores
            .get(address)
            .copied()
            .unwrap_or(50.0)
    }

    /// Get blockchain statistics
    pub async fn get_statistics(&self) -> BlockchainStatistics {
        self.statistics.read().await.clone()
    }

    /// Get threat statistics
    pub async fn get_threat_statistics(&self) -> Result<ThreatStatistics> {
        let registry = self.threat_registry.read().await;

        let mut by_type: HashMap<ThreatType, u64> = HashMap::new();
        let mut by_severity: HashMap<Severity, u64> = HashMap::new();

        for threat in registry.threats.values() {
            *by_type.entry(threat.threat_type).or_insert(0) += 1;
            *by_severity.entry(threat.severity).or_insert(0) += 1;
        }

        Ok(ThreatStatistics {
            total_threats: registry.threats.len() as u64,
            total_attestations: registry
                .attestations
                .values()
                .map(|v| v.len() as u64)
                .sum::<u64>(),
            by_type,
            by_severity,
            average_confidence: registry.threats.values().map(|t| t.confidence).sum::<f64>()
                / registry.threats.len().max(1) as f64,
        })
    }

    // Private helper methods

    async fn create_genesis_block(&self) -> Result<Block> {
        let header = BlockHeader {
            version: 1,
            height: 0,
            timestamp: 1700000000,
            previous_hash: "0".repeat(64),
            merkle_root: "0".repeat(64),
            nonce: 0,
            difficulty: 1,
            validator: "GENESIS".to_string(),
        };

        let hash = self.calculate_block_hash(&header).await;

        Ok(Block {
            header,
            transactions: Vec::new(),
            hash,
            signature: String::new(),
        })
    }

    async fn calculate_block_hash(&self, header: &BlockHeader) -> String {
        let mut hasher = Sha256::new();
        hasher.update(header.version.to_be_bytes());
        hasher.update(header.height.to_be_bytes());
        hasher.update(header.timestamp.to_be_bytes());
        hasher.update(header.previous_hash.as_bytes());
        hasher.update(header.merkle_root.as_bytes());
        hasher.update(header.nonce.to_be_bytes());
        format!("{:x}", hasher.finalize())
    }

    async fn calculate_merkle_root(&self, transactions: &[Transaction]) -> String {
        if transactions.is_empty() {
            return "0".repeat(64);
        }

        let mut hashes: Vec<String> = transactions.iter().map(|tx| tx.id.clone()).collect();

        while hashes.len() > 1 {
            let mut new_hashes = Vec::new();
            for chunk in hashes.chunks(2) {
                let combined = if chunk.len() == 2 {
                    format!("{}{}", chunk[0], chunk[1])
                } else {
                    format!("{}{}", chunk[0], chunk[0])
                };
                let mut hasher = Sha256::new();
                hasher.update(combined);
                new_hashes.push(format!("{:x}", hasher.finalize()));
            }
            hashes = new_hashes;
        }

        hashes.into_iter().next().unwrap_or_else(|| "0".repeat(64))
    }

    async fn generate_tx_id(&self) -> String {
        self.generate_id().await
    }

    async fn generate_id(&self) -> String {
        let mut bytes = [0u8; 32];
        OsRng.fill_bytes(&mut bytes);
        format!("{:x}", Sha256::digest(bytes))
    }

    async fn generate_contract_address(&self) -> String {
        self.generate_id().await
    }

    async fn process_transactions(&self, transactions: &[Transaction]) -> Result<()> {
        for tx in transactions {
            if tx.transaction_type == TransactionType::ThreatSubmission {
                let threat: ThreatEntry = serde_json::from_slice(&tx.payload)?;
                let mut registry = self.threat_registry.write().await;
                registry.threats.insert(threat.threat_id.clone(), threat);
                registry
                    .reputation_scores
                    .entry(tx.sender.clone())
                    .or_insert(50.0);
            }
        }
        Ok(())
    }

    async fn update_reputation(&self, address: &str, correct: bool) -> Result<()> {
        let mut registry = self.threat_registry.write().await;
        let score = registry
            .reputation_scores
            .entry(address.to_string())
            .or_insert(50.0);

        if correct {
            *score = (*score + 5.0).min(100.0);
        } else {
            *score = (*score - 10.0).max(0.0);
        }

        Ok(())
    }

    async fn execute_contract_function(
        &self,
        contract: &mut SmartContract,
        function: &str,
        args: Vec<Vec<u8>>,
    ) -> Result<Vec<u8>> {
        // Simplified contract execution
        match function {
            "get" => {
                if !args.is_empty() {
                    let key = String::from_utf8_lossy(&args[0]).to_string();
                    return Ok(contract.state.get(&key).cloned().unwrap_or_default());
                }
            }
            "set" => {
                if args.len() >= 2 {
                    let key = String::from_utf8_lossy(&args[0]).to_string();
                    contract.state.insert(key, args[1].clone());
                    return Ok(vec![1]);
                }
            }
            _ => {}
        }

        Ok(vec![])
    }
}

impl ConsensusEngine {
    pub fn new(algorithm: ConsensusAlgorithm) -> Self {
        Self {
            algorithm,
            validators: Vec::new(),
            current_height: 0,
            current_round: 0,
            votes: HashMap::new(),
        }
    }

    pub fn add_validator(&mut self, validator: Validator) {
        self.validators.push(validator);
    }

    pub fn get_validators(&self) -> &[Validator] {
        &self.validators
    }
}

impl Default for Wallet {
    fn default() -> Self {
        Self::new()
    }
}

impl Wallet {
    pub fn new() -> Self {
        Self {
            private_key: vec![0u8; 32],
            public_key: vec![0u8; 32],
            address: String::new(),
        }
    }

    pub fn generate_keys(&mut self) -> Result<()> {
        OsRng.fill_bytes(&mut self.private_key);

        // Derive public key (simplified)
        let mut hasher = Sha256::new();
        hasher.update(&self.private_key);
        self.public_key = hasher.finalize().to_vec();

        // Derive address
        self.address = format!("{:x}", Sha256::digest(&self.public_key));

        Ok(())
    }

    pub fn sign(&self, message: &[u8]) -> Result<String> {
        let mut hasher = Sha512::new();
        hasher.update(&self.private_key);
        hasher.update(message);
        Ok(format!("{:x}", hasher.finalize()))
    }

    pub fn verify(&self, message: &[u8], signature: &str) -> bool {
        let expected = self.sign(message).unwrap_or_default();
        signature == expected
    }
}

impl Default for ThreatRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ThreatRegistry {
    pub fn new() -> Self {
        Self {
            threats: HashMap::new(),
            reputation_scores: HashMap::new(),
            attestations: HashMap::new(),
        }
    }
}

/// Threat Statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatStatistics {
    pub total_threats: u64,
    pub total_attestations: u64,
    pub by_type: HashMap<ThreatType, u64>,
    pub by_severity: HashMap<Severity, u64>,
    pub average_confidence: f64,
}

/// Initialize blockchain module
pub fn init() -> Result<()> {
    info!("Blockchain Module initialized");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_blockchain_initialization() {
        let manager = BlockchainManager::new().unwrap();
        assert!(manager.initialize().await.is_ok());
        assert_eq!(manager.get_height().await, 0);
    }

    #[tokio::test]
    async fn test_threat_submission() {
        let manager = BlockchainManager::new().unwrap();
        manager.initialize().await.unwrap();

        let threat = ThreatEntry {
            threat_id: "THREAT-001".to_string(),
            threat_type: ThreatType::Malware,
            severity: Severity::High,
            description: "Test malware".to_string(),
            indicators: vec![],
            first_seen: Utc::now().timestamp(),
            last_seen: Utc::now().timestamp(),
            confidence: 0.95,
            submitter: "test".to_string(),
            endorsements: 0,
            challenges: 0,
        };

        let id = manager.submit_threat(threat).await.unwrap();
        assert_eq!(id, "THREAT-001");
    }

    #[tokio::test]
    async fn test_block_addition() {
        let manager = BlockchainManager::new().unwrap();
        manager.initialize().await.unwrap();

        let block_hash = manager.add_block(vec![]).await.unwrap();
        assert!(!block_hash.is_empty());
        assert_eq!(manager.get_height().await, 1);
    }

    #[tokio::test]
    async fn test_contract_deployment() {
        let manager = BlockchainManager::new().unwrap();
        manager.initialize().await.unwrap();

        let abi = ContractABI {
            functions: vec![ContractFunction {
                name: "get".to_string(),
                inputs: vec![],
                outputs: vec![],
                payable: false,
            }],
            events: vec![],
        };

        let address = manager.deploy_contract(vec![1, 2, 3], abi).await.unwrap();
        assert!(!address.is_empty());
    }

    #[tokio::test]
    async fn test_reputation_system() {
        let manager = BlockchainManager::new().unwrap();
        manager.initialize().await.unwrap();

        let threat = ThreatEntry {
            threat_id: "THREAT-002".to_string(),
            threat_type: ThreatType::Phishing,
            severity: Severity::Medium,
            description: "Test phishing".to_string(),
            indicators: vec![],
            first_seen: Utc::now().timestamp(),
            last_seen: Utc::now().timestamp(),
            confidence: 0.85,
            submitter: "test".to_string(),
            endorsements: 0,
            challenges: 0,
        };

        manager.submit_threat(threat).await.unwrap();
        manager
            .attest_threat("THREAT-002", true, 0.9)
            .await
            .unwrap();

        let retrieved = manager.get_threat("THREAT-002").await.unwrap().unwrap();
        assert_eq!(retrieved.endorsements, 1);
    }
}

//! Cryptographic Inventory Tool
//! 
//! This tool scans the V-Sentinel codebase to identify all cryptographic operations,
//! algorithms, and dependencies. It helps assess quantum vulnerability exposure.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use regex::Regex;
use serde::{Serialize, Deserialize};

/// Cryptographic usage entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoUsage {
    pub file_path: String,
    pub line_number: usize,
    pub operation_type: CryptoOperation,
    pub algorithm: String,
    pub description: String,
    pub quantum_vulnerable: bool,
}

/// Types of cryptographic operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CryptoOperation {
    Encryption,
    Decryption,
    KeyGeneration,
    KeyExchange,
    DigitalSignature,
    SignatureVerification,
    Hashing,
    KeyDerivation,
    RandomNumberGeneration,
    Unknown,
}

/// Quantum vulnerability assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumVulnerabilityReport {
    pub total_crypto_operations: usize,
    pub quantum_vulnerable_operations: usize,
    pub quantum_safe_operations: usize,
    pub vulnerable_operations: Vec<CryptoUsage>,
    pub safe_operations: Vec<CryptoUsage>,
    pub algorithm_distribution: HashMap<String, usize>,
    pub risk_level: RiskLevel,
}

/// Risk level assessment
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RiskLevel {
    Critical,
    High,
    Medium,
    Low,
}

/// Cryptographic Inventory Tool
pub struct CryptoInventoryTool {
    crypto_patterns: HashMap<CryptoOperation, Vec<Regex>>,
    vulnerable_algorithms: HashSet<String>,
    safe_algorithms: HashSet<String>,
}

impl CryptoInventoryTool {
    /// Create a new cryptographic inventory tool
    pub fn new() -> Self {
        let mut tool = Self {
            crypto_patterns: HashMap::new(),
            vulnerable_algorithms: HashSet::new(),
            safe_algorithms: HashSet::new(),
        };
        
        tool.initialize_patterns();
        tool.initialize_algorithm_lists();
        
        tool
    }
    
    fn initialize_patterns(&mut self) {
        // Encryption patterns
        let encryption_patterns = vec![
            Regex::new(r"(?i)(encrypt|aes|des|blowfish|twofish|rc4|rc5|rsa|ecdh|ecies)").unwrap(),
            Regex::new(r"(?i)(openssl_encrypt|crypto_box|sealedbox)").unwrap(),
            Regex::new(r"(?i)(rustls|ring::aead|aes_gcm)").unwrap(),
        ];
        self.crypto_patterns.insert(CryptoOperation::Encryption, encryption_patterns);
        
        // Decryption patterns
        let decryption_patterns = vec![
            Regex::new(r"(?i)(decrypt|openssl_decrypt|crypto_box_open)").unwrap(),
            Regex::new(r"(?i)(open_in_place|unseal)").unwrap(),
        ];
        self.crypto_patterns.insert(CryptoOperation::Decryption, decryption_patterns);
        
        // Key generation patterns
        let keygen_patterns = vec![
            Regex::new(r"(?i)(keypair|generate_key|keygen|new_key|private_key|public_key)").unwrap(),
            Regex::new(r"(?i)(keypair\(\)|key_from_seed|keypair_from_seed)").unwrap(),
        ];
        self.crypto_patterns.insert(CryptoOperation::KeyGeneration, keygen_patterns);
        
        // Key exchange patterns
        let keyexchange_patterns = vec![
            Regex::new(r"(?i)(diffie_hellman|key_exchange|derive_shared|encapsulate|decapsulate)").unwrap(),
            Regex::new(r"(?i)(dh_compute_shared|ecdh)").unwrap(),
        ];
        self.crypto_patterns.insert(CryptoOperation::KeyExchange, keyexchange_patterns);
        
        // Digital signature patterns
        let signature_patterns = vec![
            Regex::new(r"(?i)(sign|digital_signature|ed25519|ecdsa|rsa_sign)").unwrap(),
            Regex::new(r"(?i)(signing_key|sign_with)").unwrap(),
        ];
        self.crypto_patterns.insert(CryptoOperation::DigitalSignature, signature_patterns);
        
        // Signature verification patterns
        let verify_patterns = vec![
            Regex::new(r"(?i)(verify|signature_verify|verify_sig)").unwrap(),
            Regex::new(r"(?i)(verify_with|is_valid_signature)").unwrap(),
        ];
        self.crypto_patterns.insert(CryptoOperation::SignatureVerification, verify_patterns);
        
        // Hashing patterns
        let hash_patterns = vec![
            Regex::new(r"(?i)(hash|sha256|sha512|sha3|blake2|md5|keccak)").unwrap(),
            Regex::new(r"(?i)(digest|hasher|finalize\(\))").unwrap(),
        ];
        self.crypto_patterns.insert(CryptoOperation::Hashing, hash_patterns);
        
        // Key derivation patterns
        let kdf_patterns = vec![
            Regex::new(r"(?i)(derive_key|kdf|hkdf|pbkdf2|scrypt|argon2)").unwrap(),
            Regex::new(r"(?i)(stretch|derive_from)").unwrap(),
        ];
        self.crypto_patterns.insert(CryptoOperation::KeyDerivation, kdf_patterns);
        
        // Random number generation patterns
        let rng_patterns = vec![
            Regex::new(r"(?i)(random|rng|osrng|thread_rng|random_bytes)").unwrap(),
            Regex::new(r"(?i)(fill_bytes|gen_random)").unwrap(),
        ];
        self.crypto_patterns.insert(CryptoOperation::RandomNumberGeneration, rng_patterns);
    }
    
    fn initialize_algorithm_lists(&mut self) {
        // Quantum-vulnerable algorithms (RSA, ECC, DH)
        self.vulnerable_algorithms.insert("rsa".to_string());
        self.vulnerable_algorithms.insert("ecdh".to_string());
        self.vulnerable_algorithms.insert("ecdsa".to_string());
        self.vulnerable_algorithms.insert("ed25519".to_string());
        self.vulnerable_algorithms.insert("ed448".to_string());
        self.vulnerable_algorithms.insert("diffie_hellman".to_string());
        self.vulnerable_algorithms.insert("dh_compute_shared".to_string());
        self.vulnerable_algorithms.insert("x25519".to_string());
        self.vulnerable_algorithms.insert("x448".to_string());
        
        // Quantum-safe algorithms (PQC, symmetric crypto, hash-based)
        self.safe_algorithms.insert("crystals_kyber".to_string());
        self.safe_algorithms.insert("crystals_dilithium".to_string());
        self.safe_algorithms.insert("sphincs".to_string());
        self.safe_algorithms.insert("falcon".to_string());
        self.safe_algorithms.insert("aes".to_string());
        self.safe_algorithms.insert("chacha20".to_string());
        self.safe_algorithms.insert("sha256".to_string());
        self.safe_algorithms.insert("sha512".to_string());
        self.safe_algorithms.insert("sha3".to_string());
        self.safe_algorithms.insert("blake2".to_string());
        self.safe_algorithms.insert("hkdf".to_string());
        self.safe_algorithms.insert("pbkdf2".to_string());
        self.safe_algorithms.insert("scrypt".to_string());
        self.safe_algorithms.insert("argon2".to_string());
    }
    
    /// Scan a directory for cryptographic usage
    pub fn scan_directory(&self, directory: &Path) -> Result<Vec<CryptoUsage>, String> {
        let mut all_usage = Vec::new();
        
        let entries = fs::read_dir(directory)
            .map_err(|e| format!("Failed to read directory: {}", e))?;
        
        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            let path = entry.path();
            
            if path.is_dir() {
                // Recursively scan subdirectories
                if let Ok(sub_usage) = self.scan_directory(&path) {
                    all_usage.extend(sub_usage);
                }
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                // Scan Rust source files
                if let Ok(file_usage) = self.scan_file(&path) {
                    all_usage.extend(file_usage);
                }
            }
        }
        
        Ok(all_usage)
    }
    
    /// Scan a single file for cryptographic usage
    fn scan_file(&self, file_path: &Path) -> Result<Vec<CryptoUsage>, String> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read file {}: {}", file_path.display(), e))?;
        
        let mut usage_list = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        
        for (line_num, line) in lines.iter().enumerate() {
            for (op_type, patterns) in &self.crypto_patterns {
                for pattern in patterns {
                    if let Some(captures) = pattern.find(line) {
                        let algorithm = self.extract_algorithm(&captures.as_str());
                        let quantum_vulnerable = self.is_quantum_vulnerable(&algorithm);
                        
                        usage_list.push(CryptoUsage {
                            file_path: file_path.to_string_lossy().to_string(),
                            line_number: line_num + 1,
                            operation_type: op_type.clone(),
                            algorithm,
                            description: captures.as_str().to_string(),
                            quantum_vulnerable,
                        });
                    }
                }
            }
        }
        
        Ok(usage_list)
    }
    
    /// Extract algorithm name from matched text
    fn extract_algorithm(&self, text: &str) -> String {
        let lower = text.to_lowercase();
        
        // Check for known algorithms
        for algo in &self.vulnerable_algorithms {
            if lower.contains(algo) {
                return algo.clone();
            }
        }
        
        for algo in &self.safe_algorithms {
            if lower.contains(algo) {
                return algo.clone();
            }
        }
        
        // Default: extract first word
        text.split_whitespace()
            .next()
            .unwrap_or("unknown")
            .to_lowercase()
    }
    
    /// Check if an algorithm is quantum-vulnerable
    fn is_quantum_vulnerable(&self, algorithm: &str) -> bool {
        self.vulnerable_algorithms.contains(&algorithm.to_lowercase())
    }
    
    /// Generate quantum vulnerability report
    pub fn generate_vulnerability_report(&self, usage: &[CryptoUsage]) -> QuantumVulnerabilityReport {
        let vulnerable_operations: Vec<CryptoUsage> = usage
            .iter()
            .filter(|u| u.quantum_vulnerable)
            .cloned()
            .collect();
        
        let safe_operations: Vec<CryptoUsage> = usage
            .iter()
            .filter(|u| !u.quantum_vulnerable)
            .cloned()
            .collect();
        
        let mut algorithm_distribution: HashMap<String, usize> = HashMap::new();
        for usage in usage {
            *algorithm_distribution.entry(usage.algorithm.clone()).or_insert(0) += 1;
        }
        
        let risk_level = self.calculate_risk_level(&vulnerable_operations, usage.len());
        
        QuantumVulnerabilityReport {
            total_crypto_operations: usage.len(),
            quantum_vulnerable_operations: vulnerable_operations.len(),
            quantum_safe_operations: safe_operations.len(),
            vulnerable_operations,
            safe_operations,
            algorithm_distribution,
            risk_level,
        }
    }
    
    /// Calculate overall risk level
    fn calculate_risk_level(&self, vulnerable: &[CryptoUsage], total: usize) -> RiskLevel {
        if total == 0 {
            return RiskLevel::Low;
        }
        
        let vulnerable_ratio = vulnerable.len() as f64 / total as f64;
        
        // Check for critical vulnerabilities
        let has_rsa = vulnerable.iter().any(|u| u.algorithm == "rsa");
        let has_ecc = vulnerable.iter().any(|u| 
            u.algorithm == "ecdh" || u.algorithm == "ecdsa" || 
            u.algorithm == "ed25519" || u.algorithm == "x25519"
        );
        
        if (has_rsa || has_ecc) && vulnerable_ratio > 0.5 {
            RiskLevel::Critical
        } else if vulnerable_ratio > 0.7 {
            RiskLevel::Critical
        } else if vulnerable_ratio > 0.5 {
            RiskLevel::High
        } else if vulnerable_ratio > 0.3 {
            RiskLevel::Medium
        } else {
            RiskLevel::Low
        }
    }
    
    /// Generate markdown report
    pub fn generate_markdown_report(&self, report: &QuantumVulnerabilityReport) -> String {
        let mut md = String::new();
        
        md.push_str("# Cryptographic Inventory & Quantum Vulnerability Report\n\n");
        md.push_str("## Executive Summary\n\n");
        md.push_str(&format!("### Overall Risk Level: **{:?}**\n\n", report.risk_level));
        md.push_str(&format!("- **Total Crypto Operations**: {}\n", report.total_crypto_operations));
        md.push_str(&format!("- **Quantum-Vulnerable Operations**: {}\n", report.quantum_vulnerable_operations));
        md.push_str(&format!("- **Quantum-Safe Operations**: {}\n", report.quantum_safe_operations));
        md.push_str(&format!("- **Vulnerability Ratio**: {:.1}%\n\n", 
            if report.total_crypto_operations > 0 {
                (report.quantum_vulnerable_operations as f64 / report.total_crypto_operations as f64) * 100.0
            } else {
                0.0
            }
        ));
        
        md.push_str("### Algorithm Distribution\n\n");
        let mut algos: Vec<_> = report.algorithm_distribution.iter().collect();
        algos.sort_by(|a, b| b.1.cmp(a.1));
        
        for (algorithm, count) in algos {
            md.push_str(&format!("- **{}**: {} uses\n", algorithm, count));
        }
        md.push_str("\n");
        
        // Vulnerable operations section
        if !report.vulnerable_operations.is_empty() {
            md.push_str("## Quantum-Vulnerable Operations\n\n");
            md.push_str("The following operations use algorithms that are vulnerable to quantum attacks:\n\n");
            
            for (i, usage) in report.vulnerable_operations.iter().enumerate() {
                md.push_str(&format!("### {}. {} Operation\n", i + 1, usage.operation_type));
                md.push_str(&format!("- **File**: `{}` (line {})\n", usage.file_path, usage.line_number));
                md.push_str(&format!("- **Algorithm**: `{}`\n", usage.algorithm));
                md.push_str(&format!("- **Pattern**: `{}`\n", usage.description));
                md.push_str("\n");
            }
        }
        
        // Safe operations section
        if !report.safe_operations.is_empty() {
            md.push_str("## Quantum-Safe Operations\n\n");
            md.push_str("The following operations use quantum-safe algorithms:\n\n");
            
            for (i, usage) in report.safe_operations.iter().enumerate().take(20) {
                md.push_str(&format!("{}. **{}** - `{}` (line {})\n", 
                    i + 1, usage.algorithm, usage.file_path, usage.line_number));
            }
            
            if report.safe_operations.len() > 20 {
                md.push_str(&format!("*... and {} more*\n", report.safe_operations.len() - 20));
            }
            md.push_str("\n");
        }
        
        // Recommendations section
        md.push_str("## Recommendations\n\n");
        match report.risk_level {
            RiskLevel::Critical => {
                md.push_str("⚠️ **CRITICAL**: Immediate action required. Your system has significant quantum vulnerabilities.\n\n");
                md.push_str("### Immediate Actions:\n");
                md.push_str("1. Prioritize migration to post-quantum cryptography (PQC)\n");
                md.push_str("2. Replace RSA and ECC operations with CRYSTALS-Kyber and CRYSTALS-Dilithium\n");
                md.push_str("3. Begin &quot;harvest now, decrypt later&quot; risk assessment\n");
                md.push_str("4. Implement crypto-agility framework\n\n");
            }
            RiskLevel::High => {
                md.push_str("⚠️ **HIGH**: Significant vulnerabilities detected. PQC migration is urgent.\n\n");
                md.push_str("### Recommended Actions:\n");
                md.push_str("1. Create PQC migration roadmap\n");
                md.push_str("2. Start with highest-risk operations\n");
                md.push_str("3. Implement hybrid crypto (classical + PQC) during transition\n");
                md.push_str("4. Update security policies for quantum readiness\n\n");
            }
            RiskLevel::Medium => {
                md.push_str("⚠️ **MEDIUM**: Moderate quantum vulnerabilities detected. Begin planning PQC migration.\n\n");
                md.push_str("### Recommended Actions:\n");
                md.push_str("1. Assess critical path operations\n");
                md.push_str("2. Develop PQC implementation plan\n");
                md.push_str("3. Test PQC algorithms in non-critical systems\n");
                md.push_str("4. Train team on PQC migration\n\n");
            }
            RiskLevel::Low => {
                md.push_str("✅ **LOW**: Good quantum readiness posture. Continue monitoring.\n\n");
                md.push_str("### Recommended Actions:\n");
                md.push_str("1. Continue using quantum-safe algorithms\n");
                md.push_str("2. Monitor for new quantum computing developments\n");
                md.push_str("3. Plan for future PQC adoption\n");
                md.push_str("4. Maintain crypto-agility\n\n");
            }
        }
        
        md.push_str("### Next Steps for PQC Migration:\n\n");
        md.push_str("1. **Phase 1**: Cryptographic discovery (current)\n");
        md.push_str("2. **Phase 2**: Add PQC algorithms (CRYSTALS-Kyber, CRYSTALS-Dilithium)\n");
        md.push_str("3. **Phase 3**: Integrate PQC with existing systems\n");
        md.push_str("4. **Phase 4**: Implement crypto-agility framework\n");
        md.push_str("5. **Phase 5**: Migration and validation\n\n");
        
        md
/// Main entry point for the cryptographic inventory tool
fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║     V-Sentinel Cryptographic Inventory Tool                    ║");
    println!("║     Post-Quantum Cryptography Assessment                     ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();
    
    let tool = CryptoInventoryTool::new();
    
    // Scan the source directory
    if let Err(e) = tool.run("src/") {
        eprintln!("❌ Error: {}", e);
        std::process::exit(1);
    }
    
    println!("\n✅ Cryptographic inventory complete!");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_algorithm_classification() {
        let tool = CryptoInventoryTool::new();
        
        assert!(tool.is_quantum_vulnerable("rsa"));
        assert!(tool.is_quantum_vulnerable("ecdh"));
        assert!(tool.is_quantum_vulnerable("ecdsa"));
        
        assert!(!tool.is_quantum_vulnerable("aes"));
        assert!(!tool.is_quantum_vulnerable("sha256"));
        assert!(!tool.is_quantum_vulnerable("crystals_kyber"));
    }
    
    #[test]
    fn test_risk_level_calculation() {
        let tool = CryptoInventoryTool::new();
        
        // Test critical risk
        let vulnerable = vec![
            CryptoUsage {
                file_path: "test.rs".to_string(),
                line_number: 1,
                operation_type: CryptoOperation::KeyGeneration,
                algorithm: "rsa".to_string(),
                description: "rsa".to_string(),
                quantum_vulnerable: true,
            }
        ];
        let report = tool.generate_vulnerability_report(&vulnerable);
        assert!(report.risk_level == RiskLevel::Critical || report.risk_level == RiskLevel::High);
    }
}

impl Default for CryptoInventoryTool {
    fn default() -> Self {
        Self::new()
    }
}

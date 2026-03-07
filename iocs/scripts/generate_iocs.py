#!/usr/bin/env python3
"""
V-Sentinel IOC Generator
Generates Indicators of Compromise from threat intelligence data
"""

import json
import os
import sys
import argparse
import hashlib
from datetime import datetime
from typing import Dict, List, Any, Optional
import random
import string


class IOCGenerator:
    """Generates IOCs from threat intelligence data"""
    
    def __init__(self, config_path: Optional[str] = None):
        self.config = self._load_config(config_path)
        self.iocs_generated = 0
        self.iocs_failed = 0
        
    def _load_config(self, config_path: Optional[str]) -> Dict[str, Any]:
        """Load configuration from file or use defaults"""
        default_config = {
            "output_format": "json",
            "include_metadata": True,
            "validate_output": True,
            "confidence_level": "medium",
            "ioc_types": ["ip", "domain", "url", "hash", "email", "certificate"],
            "output_dir": "iocs/generated"
        }
        
        if config_path and os.path.exists(config_path):
            with open(config_path, 'r') as f:
                config = json.load(f)
                default_config.update(config)
        
        return default_config
    
    def generate_ip_iocs(self, count: int, threat_actor: str, campaign: str) -> List[Dict[str, Any]]:
        """Generate IP address IOCs"""
        iocs = []
        
        for i in range(count):
            ip = self._generate_random_ip()
            ioc = {
                "id": f"{threat_actor.lower().replace(' ', '-')}-ip-{i+1:04d}",
                "type": "ip",
                "value": ip,
                "first_seen": self._random_date(days_ago=random.randint(30, 365)).isoformat() + "Z",
                "last_seen": self._random_date(days_ago=random.randint(1, 30)).isoformat() + "Z",
                "confidence": self._random_confidence(),
                "description": f"Command and control server for {campaign}",
                "campaign": campaign,
                "threat_actor": threat_actor,
                "tags": ["c2", "botnet", "malware"],
                "port": random.choice([80, 443, 8080, 8443]),
                "protocol": random.choice(["http", "https"])
            }
            iocs.append(ioc)
        
        return iocs
    
    def generate_domain_iocs(self, count: int, threat_actor: str, campaign: str) -> List[Dict[str, Any]]:
        """Generate domain name IOCs"""
        iocs = []
        domains = self._generate_random_domains(count)
        
        for i, domain in enumerate(domains):
            ioc = {
                "id": f"{threat_actor.lower().replace(' ', '-')}-domain-{i+1:04d}",
                "type": "domain",
                "value": domain,
                "first_seen": self._random_date(days_ago=random.randint(30, 365)).isoformat() + "Z",
                "last_seen": self._random_date(days_ago=random.randint(1, 30)).isoformat() + "Z",
                "confidence": self._random_confidence(),
                "description": f"C2 domain used in {campaign}",
                "campaign": campaign,
                "threat_actor": threat_actor,
                "tags": ["c2", "domain", "botnet"],
                "dns_info": {
                    "registrar": random.choice(["GoDaddy.com, LLC", "NameCheap, Inc.", "Cloudflare, Inc."]),
                    "registered": self._random_date(days_ago=random.randint(60, 365)).strftime("%Y-%m-%d"),
                    "status": random.choice(["clientTransferProhibited", "active"])
                }
            }
            iocs.append(ioc)
        
        return iocs
    
    def generate_hash_iocs(self, count: int, threat_actor: str, campaign: str, malware_family: str) -> List[Dict[str, Any]]:
        """Generate file hash IOCs"""
        iocs = []
        
        for i in range(count):
            sha256 = self._generate_random_sha256()
            file_size = random.randint(102400, 10485760)
            compile_time = self._random_date(days_ago=random.randint(30, 365))
            
            ioc = {
                "id": f"{threat_actor.lower().replace(' ', '-')}-hash-{i+1:04d}",
                "type": "hash",
                "value": sha256,
                "hash_type": "sha256",
                "first_seen": self._random_date(days_ago=random.randint(30, 365)).isoformat() + "Z",
                "last_seen": self._random_date(days_ago=random.randint(1, 30)).isoformat() + "Z",
                "confidence": self._random_confidence(),
                "description": f"{malware_family} malware variant",
                "campaign": campaign,
                "threat_actor": threat_actor,
                "malware_family": malware_family,
                "tags": ["malware", "trojan", "windows"],
                "file_info": {
                    "name": f"{malware_family.lower().replace(' ', '_')}_{i+1}.exe",
                    "size": file_size,
                    "type": random.choice([
                        "PE32 executable (GUI) Intel 80386",
                        "PE32+ executable (DLL) x86-64",
                        "ELF 64-bit LSB executable"
                    ]),
                    "compile_time": compile_time.strftime("%Y-%m-%dT%H:%M:%SZ")
                }
            }
            iocs.append(ioc)
        
        return iocs
    
    def generate_url_iocs(self, count: int, threat_actor: str, campaign: str) -> List[Dict[str, Any]]:
        """Generate URL IOCs"""
        iocs = []
        domains = self._generate_random_domains(count)
        
        for i, domain in enumerate(domains):
            paths = [
                "/api/v1/update",
                "/download/payload",
                "/c2/beacon",
                "/status/check",
                "/config/retrieve"
            ]
            
            ioc = {
                "id": f"{threat_actor.lower().replace(' ', '-')}-url-{i+1:04d}",
                "type": "url",
                "value": f"https://{domain}{random.choice(paths)}",
                "first_seen": self._random_date(days_ago=random.randint(30, 365)).isoformat() + "Z",
                "last_seen": self._random_date(days_ago=random.randint(1, 30)).isoformat() + "Z",
                "confidence": self._random_confidence(),
                "description": f"C2 endpoint for {campaign}",
                "campaign": campaign,
                "threat_actor": threat_actor,
                "tags": ["c2", "api", "botnet"],
                "http_method": random.choice(["GET", "POST"])
            }
            iocs.append(ioc)
        
        return iocs
    
    def generate_email_iocs(self, count: int, threat_actor: str, campaign: str) -> List[Dict[str, Any]]:
        """Generate email address IOCs"""
        iocs = []
        
        for i in range(count):
            username = self._generate_random_username()
            domain = self._generate_random_domains(1)[0]
            
            ioc = {
                "id": f"{threat_actor.lower().replace(' ', '-')}-email-{i+1:04d}",
                "type": "email",
                "value": f"{username}@{domain}",
                "first_seen": self._random_date(days_ago=random.randint(30, 365)).isoformat() + "Z",
                "last_seen": self._random_date(days_ago=random.randint(1, 30)).isoformat() + "Z",
                "confidence": self._random_confidence(),
                "description": f"Phishing email for {campaign}",
                "campaign": campaign,
                "threat_actor": threat_actor,
                "tags": ["phishing", "email", "botnet"]
            }
            iocs.append(ioc)
        
        return iocs
    
    def _generate_random_ip(self) -> str:
        """Generate random IP address"""
        return f"{random.randint(1, 255)}.{random.randint(0, 255)}.{random.randint(0, 255)}.{random.randint(1, 254)}"
    
    def _generate_random_domains(self, count: int) -> List[str]:
        """Generate random domain names"""
        domains = []
        tlds = [".com", ".net", ".org", ".info", ".biz", ".co"]
        
        for _ in range(count):
            prefix = ''.join(random.choices(string.ascii_lowercase + string.digits, k=random.randint(8, 12)))
            domain = f"{prefix}{random.choice(tlds)}"
            domains.append(domain)
        
        return domains
    
    def _generate_random_sha256(self) -> str:
        """Generate random SHA256 hash"""
        return ''.join(random.choices(string.hexdigits.lower(), k=64))
    
    def _generate_random_username(self) -> str:
        """Generate random username"""
        prefixes = ["info", "support", "admin", "service", "billing", "notifications"]
        return random.choice(prefixes)
    
    def _random_date(self, days_ago: int) -> datetime:
        """Generate random date"""
        return datetime.now() - timedelta(days=random.randint(0, days_ago))
    
    def _random_confidence(self) -> str:
        """Generate random confidence level"""
        return random.choice(["low", "medium", "high"])
    
    def save_iocs(self, iocs: List[Dict[str, Any]], output_file: str) -> bool:
        """Save IOCs to file"""
        try:
            os.makedirs(os.path.dirname(output_file), exist_ok=True)
            
            with open(output_file, 'w') as f:
                json.dump(iocs, f, indent=2)
            
            self.iocs_generated += len(iocs)
            return True
        except Exception as e:
            print(f"Error saving IOCs: {e}", file=sys.stderr)
            self.iocs_failed += len(iocs)
            return False
    
    def generate_all(self, threat_actor: str, campaign: str, counts: Dict[str, int]) -> Dict[str, List[Dict[str, Any]]]:
        """Generate all types of IOCs"""
        all_iocs = {}
        
        if "ip" in counts and counts["ip"] > 0:
            all_iocs["ips"] = self.generate_ip_iocs(counts["ip"], threat_actor, campaign)
        
        if "domain" in counts and counts["domain"] > 0:
            all_iocs["domains"] = self.generate_domain_iocs(counts["domain"], threat_actor, campaign)
        
        if "hash" in counts and counts["hash"] > 0:
            malware_family = campaign.replace(" ", "")
            all_iocs["hashes"] = self.generate_hash_iocs(counts["hash"], threat_actor, campaign, malware_family)
        
        if "url" in counts and counts["url"] > 0:
            all_iocs["urls"] = self.generate_url_iocs(counts["url"], threat_actor, campaign)
        
        if "email" in counts and counts["email"] > 0:
            all_iocs["emails"] = self.generate_email_iocs(counts["email"], threat_actor, campaign)
        
        return all_iocs


def main():
    parser = argparse.ArgumentParser(description="Generate IOCs for threat intelligence")
    parser.add_argument("--threat-actor", required=True, help="Threat actor name")
    parser.add_argument("--campaign", required=True, help="Campaign name")
    parser.add_argument("--ip-count", type=int, default=10, help="Number of IP IOCs to generate")
    parser.add_argument("--domain-count", type=int, default=10, help="Number of domain IOCs to generate")
    parser.add_argument("--hash-count", type=int, default=10, help="Number of hash IOCs to generate")
    parser.add_argument("--url-count", type=int, default=10, help="Number of URL IOCs to generate")
    parser.add_argument("--email-count", type=int, default=5, help="Number of email IOCs to generate")
    parser.add_argument("--output", default="iocs/generated/threat_iocs.json", help="Output file path")
    parser.add_argument("--format", choices=["json", "csv", "stix"], default="json", help="Output format")
    parser.add_argument("--config", help="Configuration file path")
    
    args = parser.parse_args()
    
    generator = IOCGenerator(args.config)
    
    counts = {
        "ip": args.ip_count,
        "domain": args.domain_count,
        "hash": args.hash_count,
        "url": args.url_count,
        "email": args.email_count
    }
    
    print(f"Generating IOCs for {args.threat_actor} - {args.campaign}...")
    
    all_iocs = generator.generate_all(args.threat_actor, args.campaign, counts)
    
    # Flatten all IOCs into single list
    flat_iocs = []
    for ioc_type, ioc_list in all_iocs.items():
        flat_iocs.extend(ioc_list)
    
    # Add metadata
    metadata = {
        "threat_actor": args.threat_actor,
        "campaign": args.campaign,
        "generated_at": datetime.now().isoformat() + "Z",
        "total_iocs": len(flat_iocs),
        "ioc_types": {
            "ip": len(all_iocs.get("ips", [])),
            "domain": len(all_iocs.get("domains", [])),
            "hash": len(all_iocs.get("hashes", [])),
            "url": len(all_iocs.get("urls", [])),
            "email": len(all_iocs.get("emails", []))
        },
        "indicators": flat_iocs
    }
    
    if generator.save_iocs(metadata, args.output):
        print(f"✓ Successfully generated {len(flat_iocs)} IOCs")
        print(f"✓ Saved to {args.output}")
        print(f"\nIOC breakdown:")
        for ioc_type, count in metadata["ioc_types"].items():
            if count > 0:
                print(f"  - {ioc_type}: {count}")
    else:
        print("✗ Failed to save IOCs", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
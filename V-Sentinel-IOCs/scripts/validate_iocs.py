#!/usr/bin/env python3
"""
V-Sentinel IOC Validator
Validates Indicators of Compromise for quality and integrity
"""

import json
import os
import sys
import argparse
import re
from datetime import datetime
from typing import Dict, List, Any, Set, Tuple
import ipaddress


class IOCValidator:
    """Validates IOCs for data quality and integrity"""
    
    def __init__(self, strict_mode: bool = False):
        self.strict_mode = strict_mode
        self.errors = []
        self.warnings = []
        self.validated_iocs = 0
        self.failed_iocs = 0
        self.duplicate_iocs = 0
        self.seen_iocs = set()
        
    def validate_ioc_file(self, file_path: str) -> bool:
        """Validate a single IOC file"""
        if not os.path.exists(file_path):
            self.errors.append(f"File not found: {file_path}")
            return False
        
        try:
            with open(file_path, 'r') as f:
                data = json.load(f)
            
            # Check if it's a single IOC or collection
            if isinstance(data, dict):
                if "indicators" in data:
                    # It's a threat actor file with metadata
                    return self.validate_ioc_collection(data, file_path)
                elif "type" in data and "value" in data:
                    # It's a single IOC
                    return self.validate_single_ioc(data, file_path)
                else:
                    self.errors.append(f"Invalid IOC format in {file_path}")
                    return False
            elif isinstance(data, list):
                # It's an array of IOCs
                return self.validate_ioc_array(data, file_path)
            else:
                self.errors.append(f"Invalid data structure in {file_path}")
                return False
                
        except json.JSONDecodeError as e:
            self.errors.append(f"Invalid JSON in {file_path}: {e}")
            return False
        except Exception as e:
            self.errors.append(f"Error processing {file_path}: {e}")
            return False
    
    def validate_ioc_collection(self, data: Dict[str, Any], file_path: str) -> bool:
        """Validate a threat actor IOC collection"""
        if "threat_actor" not in data:
            self.errors.append(f"Missing threat_actor field in {file_path}")
            return False
        
        if "indicators" not in data:
            self.errors.append(f"Missing indicators field in {file_path}")
            return False
        
        if not isinstance(data["indicators"], list):
            self.errors.append(f"indicators must be an array in {file_path}")
            return False
        
        # Validate metadata
        if self.strict_mode:
            required_fields = ["threat_actor", "campaigns", "indicators", "ttps"]
            for field in required_fields:
                if field not in data:
                    self.warnings.append(f"Missing recommended field: {field}")
        
        # Validate each IOC
        all_valid = True
        for ioc in data["indicators"]:
            if not self.validate_single_ioc(ioc, file_path):
                all_valid = False
        
        return all_valid
    
    def validate_ioc_array(self, iocs: List[Dict[str, Any]], file_path: str) -> bool:
        """Validate an array of IOCs"""
        all_valid = True
        for ioc in iocs:
            if not self.validate_single_ioc(ioc, file_path):
                all_valid = False
        return all_valid
    
    def validate_single_ioc(self, ioc: Dict[str, Any], file_path: str) -> bool:
        """Validate a single IOC"""
        required_fields = ["id", "type", "value"]
        
        # Check required fields
        for field in required_fields:
            if field not in ioc:
                self.errors.append(f"Missing required field '{field}' in IOC from {file_path}")
                self.failed_iocs += 1
                return False
        
        # Validate IOC type
        if ioc["type"] not in ["ip", "domain", "url", "hash", "email", "certificate"]:
            self.errors.append(f"Invalid IOC type '{ioc['type']}' in {ioc['id']}")
            self.failed_iocs += 1
            return False
        
        # Validate based on type
        validator_method = f"_validate_{ioc['type']}"
        if hasattr(self, validator_method):
            if not getattr(self, validator_method)(ioc, file_path):
                self.failed_iocs += 1
                return False
        
        # Check for duplicates
        ioc_key = f"{ioc['type']}:{ioc['value']}"
        if ioc_key in self.seen_iocs:
            self.warnings.append(f"Duplicate IOC found: {ioc_key}")
            self.duplicate_iocs += 1
        else:
            self.seen_iocs.add(ioc_key)
        
        # Validate confidence level
        if "confidence" in ioc:
            if ioc["confidence"] not in ["low", "medium", "high"]:
                self.warnings.append(f"Invalid confidence level '{ioc['confidence']}' in {ioc['id']}")
        
        # Validate dates
        if "first_seen" in ioc:
            if not self._validate_date(ioc["first_seen"]):
                self.warnings.append(f"Invalid first_seen date in {ioc['id']}")
        
        if "last_seen" in ioc:
            if not self._validate_date(ioc["last_seen"]):
                self.warnings.append(f"Invalid last_seen date in {ioc['id']}")
        
        # Check date consistency
        if "first_seen" in ioc and "last_seen" in ioc:
            first_seen = datetime.fromisoformat(ioc["first_seen"].replace("Z", "+00:00"))
            last_seen = datetime.fromisoformat(ioc["last_seen"].replace("Z", "+00:00"))
            if first_seen > last_seen:
                self.errors.append(f"first_seen is after last_seen in {ioc['id']}")
                self.failed_iocs += 1
                return False
        
        self.validated_iocs += 1
        return True
    
    def _validate_ip(self, ioc: Dict[str, Any], file_path: str) -> bool:
        """Validate IP address IOC"""
        try:
            ipaddress.ip_address(ioc["value"])
            return True
        except ValueError:
            self.errors.append(f"Invalid IP address '{ioc['value']}' in {ioc['id']}")
            return False
    
    def _validate_domain(self, ioc: Dict[str, Any], file_path: str) -> bool:
        """Validate domain name IOC"""
        domain_pattern = r'^[a-zA-Z0-9][a-zA-Z0-9-]{0,61}[a-zA-Z0-9]\.[a-zA-Z]{2,}$'
        if not re.match(domain_pattern, ioc["value"]):
            self.errors.append(f"Invalid domain '{ioc['value']}' in {ioc['id']}")
            return False
        return True
    
    def _validate_url(self, ioc: Dict[str, Any], file_path: str) -> bool:
        """Validate URL IOC"""
        url_pattern = r'^https?://[^\s/$.?#].[^\s]*$'
        if not re.match(url_pattern, ioc["value"]):
            self.errors.append(f"Invalid URL '{ioc['value']}' in {ioc['id']}")
            return False
        return True
    
    def _validate_hash(self, ioc: Dict[str, Any], file_path: str) -> bool:
        """Validate hash IOC"""
        hash_type = ioc.get("hash_type", "sha256")
        hash_value = ioc["value"]
        
        if hash_type == "md5":
            if not re.match(r'^[a-fA-F0-9]{32}$', hash_value):
                self.errors.append(f"Invalid MD5 hash '{hash_value}' in {ioc['id']}")
                return False
        elif hash_type == "sha1":
            if not re.match(r'^[a-fA-F0-9]{40}$', hash_value):
                self.errors.append(f"Invalid SHA1 hash '{hash_value}' in {ioc['id']}")
                return False
        elif hash_type == "sha256":
            if not re.match(r'^[a-fA-F0-9]{64}$', hash_value):
                self.errors.append(f"Invalid SHA256 hash '{hash_value}' in {ioc['id']}")
                return False
        else:
            self.errors.append(f"Invalid hash type '{hash_type}' in {ioc['id']}")
            return False
        
        return True
    
    def _validate_email(self, ioc: Dict[str, Any], file_path: str) -> bool:
        """Validate email address IOC"""
        email_pattern = r'^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$'
        if not re.match(email_pattern, ioc["value"]):
            self.errors.append(f"Invalid email address '{ioc['value']}' in {ioc['id']}")
            return False
        return True
    
    def _validate_certificate(self, ioc: Dict[str, Any], file_path: str) -> bool:
        """Validate certificate IOC"""
        cert_pattern = r'^[a-fA-F0-9]{2}(:[a-fA-F0-9]{2}){15}$'
        if not re.match(cert_pattern, ioc["value"]):
            self.errors.append(f"Invalid certificate fingerprint '{ioc['value']}' in {ioc['id']}")
            return False
        return True
    
    def _validate_date(self, date_str: str) -> bool:
        """Validate ISO 8601 date string"""
        try:
            datetime.fromisoformat(date_str.replace("Z", "+00:00"))
            return True
        except ValueError:
            return False
    
    def validate_directory(self, directory: str) -> bool:
        """Validate all IOC files in a directory"""
        if not os.path.exists(directory):
            self.errors.append(f"Directory not found: {directory}")
            return False
        
        all_valid = True
        valid_files = 0
        
        for root, dirs, files in os.walk(directory):
            for file in files:
                if file.endswith('.json'):
                    file_path = os.path.join(root, file)
                    if self.validate_ioc_file(file_path):
                        valid_files += 1
                    else:
                        all_valid = False
        
        print(f"Validated {valid_files} files in {directory}")
        return all_valid
    
    def generate_report(self, output_file: Optional[str] = None) -> str:
        """Generate validation report"""
        report = []
        report.append("=" * 80)
        report.append("V-Sentinel IOC Validation Report")
        report.append("=" * 80)
        report.append(f"Generated: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
        report.append("")
        
        report.append("Summary:")
        report.append(f"  Validated IOCs: {self.validated_iocs}")
        report.append(f"  Failed IOCs: {self.failed_iocs}")
        report.append(f"  Duplicate IOCs: {self.duplicate_iocs}")
        report.append(f"  Total Errors: {len(self.errors)}")
        report.append(f"  Total Warnings: {len(self.warnings)}")
        report.append("")
        
        if self.errors:
            report.append("Errors:")
            for error in self.errors:
                report.append(f"  ✗ {error}")
            report.append("")
        
        if self.warnings:
            report.append("Warnings:")
            for warning in self.warnings:
                report.append(f"  ⚠ {warning}")
            report.append("")
        
        report.append("=" * 80)
        
        report_text = "\n".join(report)
        
        if output_file:
            with open(output_file, 'w') as f:
                f.write(report_text)
            print(f"Report saved to {output_file}")
        
        return report_text
    
    def is_valid(self) -> bool:
        """Check if validation passed"""
        if self.strict_mode:
            return self.failed_iocs == 0 and len(self.errors) == 0
        else:
            return self.failed_iocs == 0


def main():
    parser = argparse.ArgumentParser(description="Validate IOCs for data quality")
    parser.add_argument("--path", required=True, help="IOC file or directory to validate")
    parser.add_argument("--report", help="Output report file")
    parser.add_argument("--strict", action="store_true", help="Strict validation mode")
    parser.add_argument("--test", action="store_true", help="Run test validation")
    
    args = parser.parse_args()
    
    validator = IOCValidator(strict_mode=args.strict)
    
    if args.test:
        print("Running test validation...")
        # Create test IOC data
        test_ioc = {
            "id": "test-001",
            "type": "ip",
            "value": "192.168.1.1",
            "confidence": "high",
            "first_seen": "2026-01-01T00:00:00Z",
            "last_seen": "2026-01-15T00:00:00Z"
        }
        validator.validate_single_ioc(test_ioc, "test")
        print("✓ Test validation passed")
        return
    
    if os.path.isfile(args.path):
        validator.validate_ioc_file(args.path)
    elif os.path.isdir(args.path):
        validator.validate_directory(args.path)
    else:
        print(f"Error: {args.path} is not a valid file or directory", file=sys.stderr)
        sys.exit(1)
    
    # Generate and display report
    report = validator.generate_report(args.report)
    print(report)
    
    # Exit with appropriate code
    if not validator.is_valid():
        sys.exit(1)


if __name__ == "__main__":
    main()
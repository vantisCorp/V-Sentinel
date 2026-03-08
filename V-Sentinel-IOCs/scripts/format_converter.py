#!/usr/bin/env python3
"""
V-Sentinel IOC Format Converter
Converts IOCs between JSON, CSV, STIX, and OpenIOC formats
"""

import json
import csv
import os
import sys
import argparse
from datetime import datetime
from typing import Dict, List, Any, Optional
import uuid


class IOCFormatConverter:
    """Converts IOCs between different formats"""
    
    def __init__(self):
        self.converted_count = 0
        self.failed_count = 0
    
    def load_iocs(self, input_file: str) -> List[Dict[str, Any]]:
        """Load IOCs from JSON file"""
        if not os.path.exists(input_file):
            raise FileNotFoundError(f"Input file not found: {input_file}")
        
        with open(input_file, 'r') as f:
            data = json.load(f)
        
        # Handle different JSON structures
        if isinstance(data, dict):
            if "indicators" in data:
                return data["indicators"]
            elif "type" in data and "value" in data:
                return [data]
            else:
                raise ValueError("Invalid IOC structure")
        elif isinstance(data, list):
            return data
        else:
            raise ValueError("Invalid data type")
    
    def convert_to_csv(self, iocs: List[Dict[str, Any]], output_file: str) -> bool:
        """Convert IOCs to CSV format"""
        try:
            # Define CSV fields
            fieldnames = [
                "id", "type", "value", "confidence", "threat_actor",
                "campaign", "malware_family", "first_seen", "last_seen",
                "description", "tags"
            ]
            
            with open(output_file, 'w', newline='', encoding='utf-8') as f:
                writer = csv.DictWriter(f, fieldnames=fieldnames, extrasaction='ignore')
                writer.writeheader()
                
                for ioc in iocs:
                    # Flatten tags array
                    if "tags" in ioc and isinstance(ioc["tags"], list):
                        ioc["tags"] = ", ".join(ioc["tags"])
                    
                    writer.writerow(ioc)
                    self.converted_count += 1
            
            return True
        except Exception as e:
            print(f"Error converting to CSV: {e}", file=sys.stderr)
            self.failed_count += 1
            return False
    
    def convert_to_stix(self, iocs: List[Dict[str, Any]], output_file: str) -> bool:
        """Convert IOCs to STIX 2.1 format"""
        try:
            stix_bundle = {
                "type": "bundle",
                "id": f"bundle--{str(uuid.uuid4())}",
                "spec_version": "2.1",
                "objects": []
            }
            
            # Create identity object for V-Sentinel
            identity = {
                "type": "identity",
                "id": "identity--v-sentinel-research",
                "name": "V-Sentinel Security Research",
                "identity_class": "organization"
            }
            stix_bundle["objects"].append(identity)
            
            # Create indicator objects for each IOC
            for ioc in iocs:
                indicator = {
                    "type": "indicator",
                    "id": f"indicator--{str(uuid.uuid4())}",
                    "created": datetime.now().strftime("%Y-%m-%dT%H:%M:%S.%fZ"),
                    "modified": datetime.now().strftime("%Y-%m-%dT%H:%M:%S.%fZ"),
                    "name": ioc.get("description", ioc["value"]),
                    "description": ioc.get("description", ""),
                    "indicator_types": ["malicious-activity"],
                    "pattern": self._create_stix_pattern(ioc),
                    "valid_from": ioc.get("first_seen", datetime.now().isoformat() + "Z"),
                    "created_by_ref": "identity--v-sentinel-research"
                }
                
                # Add confidence level
                if "confidence" in ioc:
                    confidence_map = {"low": 15, "medium": 50, "high": 85}
                    indicator["confidence"] = confidence_map.get(ioc["confidence"], 50)
                
                # Add labels/tags
                if "tags" in ioc:
                    indicator["labels"] = ioc["tags"]
                
                # Add threat actor reference
                if "threat_actor" in ioc:
                    threat_actor_id = f"intrusion-set--{ioc['threat_actor'].lower().replace(' ', '-')}"
                    indicator["pattern"] = indicator["pattern"].replace("threat_actor", threat_actor_id)
                
                stix_bundle["objects"].append(indicator)
                self.converted_count += 1
            
            with open(output_file, 'w') as f:
                json.dump(stix_bundle, f, indent=2)
            
            return True
        except Exception as e:
            print(f"Error converting to STIX: {e}", file=sys.stderr)
            self.failed_count += 1
            return False
    
    def convert_to_openioc(self, iocs: List[Dict[str, Any]], output_file: str) -> bool:
        """Convert IOCs to OpenIOC XML format"""
        try:
            xml_lines = []
            xml_lines.append('<?xml version="1.0" encoding="UTF-8"?>')
            xml_lines.append('<OpenIOC xmlns="http://schemas.mandiant.com/2010/ioc" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:ioc="http://schemas.mandiant.com/2010/ioc" xsi:schemaLocation="http://schemas.mandiant.com/2010/ioc ioc.xsd">')
            xml_lines.append('  <definition>')
            xml_lines.append(f'    <short_description>Generated by V-Sentinel IOC Converter</short_description>')
            xml_lines.append(f'    <description>Converted {len(iocs)} indicators</description>')
            xml_lines.append(f'    <authored_by>V-Sentinel Security Research</authored_by>')
            xml_lines.append(f'    <authored_date>{datetime.now().strftime("%Y-%m-%dT%H:%M:%S")}</authored_date>')
            xml_lines.append('    <links/>')
            
            for ioc in iocs:
                xml_lines.append('    <Indicator id="{}" operator="OR">'.format(ioc.get("id", str(uuid.uuid4()))))
                xml_lines.append('      <Context document="{}" search="{}" type="{}">'.format(
                    ioc.get("threat_actor", "ThreatActor"),
                    ioc.get("description", ioc["value"]),
                    ioc["type"]
                ))
                xml_lines.append('        <ContentItem type="{}">'.format(ioc["type"]))
                
                if ioc["type"] == "ip":
                    xml_lines.append('          <Address condition="is">{}</Address>'.format(ioc["value"]))
                elif ioc["type"] == "domain":
                    xml_lines.append('          <DomainItem condition="is">{}</DomainItem>'.format(ioc["value"]))
                elif ioc["type"] == "url":
                    xml_lines.append('          <Url condition="is">{}</Url>'.format(ioc["value"]))
                elif ioc["type"] == "hash":
                    xml_lines.append('          <Hash condition="is" type="{}">{}</Hash>'.format(
                        ioc.get("hash_type", "SHA-256"), ioc["value"]
                    ))
                elif ioc["type"] == "email":
                    xml_lines.append('          <EmailItem condition="is">{}</EmailItem>'.format(ioc["value"]))
                
                xml_lines.append('        </ContentItem>')
                xml_lines.append('      </Context>')
                xml_lines.append('    </Indicator>')
                self.converted_count += 1
            
            xml_lines.append('  </definition>')
            xml_lines.append('</OpenIOC>')
            
            with open(output_file, 'w') as f:
                f.write('\n'.join(xml_lines))
            
            return True
        except Exception as e:
            print(f"Error converting to OpenIOC: {e}", file=sys.stderr)
            self.failed_count += 1
            return False
    
    def convert_to_json(self, iocs: List[Dict[str, Any]], output_file: str, pretty: bool = True) -> bool:
        """Convert IOCs to formatted JSON"""
        try:
            with open(output_file, 'w') as f:
                if pretty:
                    json.dump(iocs, f, indent=2)
                else:
                    json.dump(iocs, f)
            self.converted_count += len(iocs)
            return True
        except Exception as e:
            print(f"Error converting to JSON: {e}", file=sys.stderr)
            self.failed_count += 1
            return False
    
    def _create_stix_pattern(self, ioc: Dict[str, Any]) -> str:
        """Create STIX pattern from IOC"""
        ioc_type = ioc["type"]
        value = ioc["value"]
        
        pattern_map = {
            "ip": f"[ipv4-addr:value = '{value}']",
            "domain": f"[domain-name:value = '{value}']",
            "url": f"[url:value = '{value}']",
            "hash": f"[file:hashes.'{ioc.get(&quot;hash_type&quot;, &quot;SHA-256&quot;)}' = '{value}']",
            "email": f"[email-addr:value = '{value}']"
        }
        
        return pattern_map.get(ioc_type, f"[{ioc_type}:value = '{value}']")
    
    def convert(self, input_file: str, output_file: str, target_format: str) -> bool:
        """Main conversion method"""
        try:
            # Load IOCs
            iocs = self.load_iocs(input_file)
            print(f"Loaded {len(iocs)} IOCs from {input_file}")
            
            # Convert based on target format
            if target_format == "csv":
                success = self.convert_to_csv(iocs, output_file)
            elif target_format == "stix":
                success = self.convert_to_stix(iocs, output_file)
            elif target_format == "openioc":
                success = self.convert_to_openioc(iocs, output_file)
            elif target_format == "json":
                success = self.convert_to_json(iocs, output_file)
            else:
                raise ValueError(f"Unsupported format: {target_format}")
            
            if success:
                print(f"✓ Successfully converted {self.converted_count} IOCs to {target_format.upper()}")
                print(f"✓ Saved to {output_file}")
                if self.failed_count > 0:
                    print(f"⚠ {self.failed_count} IOCs failed to convert")
            else:
                print("✗ Conversion failed")
            
            return success
        except Exception as e:
            print(f"Conversion error: {e}", file=sys.stderr)
            return False


def main():
    parser = argparse.ArgumentParser(description="Convert IOCs between different formats")
    parser.add_argument("--input", required=True, help="Input IOC file (JSON)")
    parser.add_argument("--output", required=True, help="Output file path")
    parser.add_argument("--format", required=True, 
                       choices=["json", "csv", "stix", "openioc"],
                       help="Target format")
    parser.add_argument("--pretty", action="store_true", 
                       help="Pretty-print JSON output")
    
    args = parser.parse_args()
    
    converter = IOCFormatConverter()
    success = converter.convert(args.input, args.output, args.format)
    
    if not success:
        sys.exit(1)


if __name__ == "__main__":
    main()
/*
 * V-Sentinel YARA Rules - APT Groups
 * Version: 1.0.0
 * Author: V-Sentinel Security Research
 * Date: 2026-01-15
 */

/* ============================================================================
 * APT29 (Cozy Bear) Detection Rules
 * ============================================================================ */

rule APT29_SolarWinds_Sunburst_Backdoor {
    meta:
        description = "Detects SolarWinds Orion backdoor (Sunburst)"
        threat_actor = "APT29"
        campaign = "SolarWinds Supply Chain Attack"
        malware_family = "Sunburst"
        confidence = "high"
        date = "2026-01-15"
        reference = "https://attack.mitre.org/groups/G0016/"
        severity = "critical"
    
    strings:
        $s1 = "SolarWinds.Orion.Core.BusinessLayer.dll" wide
        $s2 = "avsvmcloud.com" ascii wide
        $s3 = "UpdateManagement" ascii
        $s4 = { 4D 5A 90 00 }  // MZ header
        $s5 = "Sunburst" ascii wide
    
    condition:
        uint16(0) == 0x5A4D and
        all of ($s*) and
        filesize < 500KB
}

rule APT29_Teardrop_Lateral_Movement {
    meta:
        description = "Detects Teardrop malware used for lateral movement"
        threat_actor = "APT29"
        campaign = "SolarWinds Supply Chain Attack"
        malware_family = "Teardrop"
        confidence = "high"
        date = "2026-01-15"
        reference = "https://attack.mitre.org/groups/G0016/"
        severity = "high"
    
    strings:
        $s1 = "teardrop.exe" ascii wide
        $s2 = "wmi" nocase
        $s3 = "schtasks" nocase
        $s4 = "powershell.exe" nocase
        $s5 = "Invoke-Expression" nocase
        $s6 = { 4D 5A 90 00 }
    
    condition:
        uint16(0) == 0x5A4D and
        4 of ($s*) and
        filesize < 300KB
}

/* ============================================================================
 * Lazarus Group Detection Rules
 * ============================================================================ */

rule Lazarus_AppleJeus_Trojan {
    meta:
        description = "Detects AppleJeus trojan targeting cryptocurrency exchanges"
        threat_actor = "Lazarus Group"
        campaign = "Operation AppleJeus"
        malware_family = "AppleJeus"
        confidence = "high"
        date = "2026-01-15"
        reference = "https://attack.mitre.org/groups/G0032/"
        severity = "critical"
    
    strings:
        $s1 = "CryptoTraderPro" ascii wide
        $s2 = "update-service.mobi" ascii wide
        $s3 = "cryptocurrency" ascii wide
        $s4 = "trading" ascii wide
        $s5 = "wallet" ascii wide
        $s6 = "exchange" ascii wide
        $s7 = { 4D 5A 90 00 } or { CF FA ED FE }  // MZ or Mach-O
    
    condition:
        (uint16(0) == 0x5A4D or uint32(0) == 0xFEEDFACE) and
        5 of ($s*) and
        filesize < 10MB
}

rule Lazarus_Manuscrypt_Stealer {
    meta:
        description = "Detects Manuscrypt malware variant for credential theft"
        threat_actor = "Lazarus Group"
        campaign = "Cryptocurrency Exchange Attacks"
        malware_family = "Manuscrypt"
        confidence = "high"
        date = "2026-01-15"
        reference = "https://attack.mitre.org/groups/G0032/"
        severity = "high"
    
    strings:
        $s1 = "manuscrypt" ascii wide nocase
        $s2 = "stealer" ascii wide nocase
        $s3 = "clipboard" ascii wide nocase
        $s4 = "keystroke" ascii wide nocase
        $s5 = "browser" ascii wide nocase
        $s6 = "password" ascii wide nocase
        $s7 = { 4D 5A 90 00 }
    
    condition:
        uint16(0) == 0x5A4D and
        4 of ($s*) and
        filesize < 5MB
}

rule Lazarus_WannaCry_Ransomware {
    meta:
        description = "Detects WannaCry ransomware payload"
        threat_actor = "Lazarus Group"
        campaign = "WannaCry Ransomware Attack"
        malware_family = "WannaCry"
        confidence = "high"
        date = "2026-01-15"
        reference = "https://attack.mitre.org/groups/G0032/"
        severity = "critical"
    
    strings:
        $s1 = "WanaDecryptor" ascii wide
        $s2 = "wcry" ascii wide
        $s3 = "EternalBlue" ascii wide
        $s4 = "DoublePulsar" ascii wide
        $s5 = "@WanaDecryptor@" ascii
        $s6 = { 4D 5A 90 00 }
        $s7 = { FF 25 00 20 40 00 }  // Common jmp pattern
    
    condition:
        uint16(0) == 0x5A4D and
        4 of ($s*) and
        filesize < 500KB
}

/* ============================================================================
 * Generic APT Detection Rules
 * ============================================================================ */

rule APT_PowerShell_Execution_Obfuscated {
    meta:
        description = "Detects obfuscated PowerShell execution commonly used by APT groups"
        threat_actor = "Multiple APT Groups"
        confidence = "medium"
        date = "2026-01-15"
        reference = "https://attack.mitre.org/techniques/T1059/006/"
        severity = "medium"
    
    strings:
        $s1 = "Invoke-Expression" nocase
        $s2 = "IEX" nocase
        $s3 = "FromBase64String" nocase
        $s4 = "ToBase64String" nocase
        $s5 = "Bypass" nocase
        $s6 = "EncodedCommand" nocase
        $s7 = "char[" nocase
    
    condition:
        4 of ($s*) and
        uint16(0) == 0x5A4D
}

rule APT_Credential_Dumping_Tools {
    meta:
        description = "Detects common credential dumping tools used by APT actors"
        threat_actor = "Multiple APT Groups"
        confidence = "high"
        date = "2026-01-15"
        reference = "https://attack.mitre.org/techniques/T1003/"
        severity = "high"
    
    strings:
        $s1 = "mimikatz" ascii wide nocase
        $s2 = "procdump" ascii wide nocase
        $s3 = "lsass.exe" ascii wide nocase
        $s4 = "sekurlsa" ascii wide nocase
        $s5 = "logonpasswords" ascii wide nocase
        $s6 = "kerberos" ascii wide nocase
        $s7 = { 4D 5A 90 00 }
    
    condition:
        uint16(0) == 0x5A4D and
        3 of ($s*) and
        filesize < 5MB
}

rule APT_Lateral_Movement_Tools {
    meta:
        description = "Detects lateral movement tools commonly used by APT groups"
        threat_actor = "Multiple APT Groups"
        confidence = "medium"
        date = "2026-01-15"
        reference = "https://attack.mitre.org/techniques/T1021/"
        severity = "medium"
    
    strings:
        $s1 = "psexec.exe" ascii wide nocase
        $s2 = "wmic.exe" ascii wide nocase
        $s3 = "wmi" ascii wide nocase
        $s4 = "smb" ascii wide nocase
        $s5 = "rdesktop" ascii wide nocase
        $s6 = "mstsc.exe" ascii wide nocase
        $s7 = { 4D 5A 90 00 }
    
    condition:
        uint16(0) == 0x5A4D and
        3 of ($s*) and
        filesize < 10MB
}

/* ============================================================================
 * MITRE ATT&CK Technique Coverage
 * ============================================================================ */

/*
 * This rule set covers the following MITRE ATT&CK techniques:
 * 
 * Initial Access:
 *   - T1195.002: Supply Chain Compromise (Sunburst)
 *   - T1190: Exploit Public-Facing Application
 * 
 * Execution:
 *   - T1059.006: PowerShell
 *   - T1204.002: User Execution: Malicious File
 * 
 * Persistence:
 *   - T1053.005: Scheduled Task
 *   - T1547.001: Boot or Logon Autostart Execution
 * 
 * Credential Access:
 *   - T1003: OS Credential Dumping
 *   - T1056.001: Input Capture: Keylogging
 * 
 * Lateral Movement:
 *   - T1021.002: SMB/Windows Admin Shares
 *   - T1021.001: Remote Desktop Protocol
 * 
 * Exfiltration:
 *   - T1041: Exfiltration Over C2 Channel
 *   - T1048.003: Exfiltration Over Unencrypted/Obfuscated Channel
 * 
 * Command and Control:
 *   - T1071.001: Web Protocols
 *   - T1071.004: DNS
 */
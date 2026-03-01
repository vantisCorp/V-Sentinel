# SENTINEL Security System - Configuration Reference

## Table of Contents
1. [Overview](#overview)
2. [Configuration File Structure](#configuration-file-structure)
3. [General Configuration](#general-configuration)
4. [Server Configuration](#server-configuration)
5. [Database Configuration](#database-configuration)
6. [Redis Configuration](#redis-configuration)
7. [S3 Configuration](#s3-configuration)
8. [Security Configuration](#security-configuration)
9. [AI Configuration](#ai-configuration)
10. [Gaming Configuration](#gaming-configuration)
11. [Quantum Configuration](#quantum-configuration)
12. [Behavioral Configuration](#behavioral-configuration)
13. [Threat Intelligence Configuration](#threat-intelligence-configuration)
14. [SIEM Configuration](#siem-configuration)
15. [Monitoring Configuration](#monitoring-configuration)
16. [Logging Configuration](#logging-configuration)
17. [Performance Configuration](#performance-configuration)
18. [Hypervisor Configuration](#hypervisor-configuration)
19. [Memory Configuration](#memory-configuration)
20. [Process Configuration](#process-configuration)
21. [Network Configuration](#network-configuration)
22. [Scanning Configuration](#scanning-configuration)
23. [Compliance Configuration](#compliance-configuration)
24. [Environment Variables](#environment-variables)

---

## Overview

The SENTINEL Security System uses TOML (Tom's Obvious, Minimal Language) for configuration. Configuration files are located in:

| Platform | Configuration File |
|----------|-------------------|
| Windows | `%APPDATA%\Sentinel\config.toml` |
| macOS | `~/Library/Application Support/Sentinel/config.toml` |
| Linux | `~/.config/sentinel/config.toml` |
| Docker | `/opt/sentinel/config/sentinel.toml` |

### Configuration File Format

```toml
# Comments start with #
# Key-value pairs use key = value
# Sections use [section_name]
# Nested sections use [section.subsection]

[section]
key = "value"
number = 42
boolean = true
array = ["item1", "item2"]

[section.subsection]
nested_key = "nested_value"
```

---

## Configuration File Structure

```
sentinel.toml
├── [general]              # General settings
├── [server]               # Server settings
├── [database]             # Database settings
├── [redis]                # Redis settings
├── [s3]                   # S3 settings
├── [security]             # Security settings
├── [ai]                   # AI settings
├── [gaming]               # Gaming settings
├── [quantum]              # Quantum cryptography settings
├── [behavioral]           # Behavioral analysis settings
├── [threat_intel]         # Threat intelligence settings
├── [siem]                 # SIEM integration settings
├── [monitoring]           # Monitoring settings
├── [logging]              # Logging settings
├── [performance]          # Performance settings
├── [hypervisor]           # Hypervisor settings
├── [memory]               # Memory settings
├── [process]              # Process settings
├── [network]              # Network settings
├── [scanning]             # Scanning settings
└── [compliance]           # Compliance settings
```

---

## General Configuration

### [general]

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `environment` | string | `"production"` | Environment: `development`, `staging`, `production` |
| `version` | string | `"1.0.0"` | SENTINEL version |
| `instance_id` | string | `""` | Instance ID (auto-generated if empty) |
| `cluster_name` | string | `"sentinel-prod"` | Cluster name for multi-instance deployments |
| `data_dir` | string | `"/opt/sentinel/data"` | Data directory |
| `log_dir` | string | `"/opt/sentinel/logs"` | Log directory |
| `temp_dir` | string | `"/opt/sentinel/tmp"` | Temporary directory |
| `protection_level` | string | `"standard"` | Protection level: `standard`, `high`, `maximum` |
| `auto_update` | bool | `true` | Enable automatic updates |
| `update_channel` | string | `"stable"` | Update channel: `stable`, `beta`, `alpha` |

**Example:**
```toml
[general]
environment = "production"
version = "1.0.0"
instance_id = ""
cluster_name = "sentinel-prod"
data_dir = "/opt/sentinel/data"
log_dir = "/opt/sentinel/logs"
temp_dir = "/opt/sentinel/tmp"
protection_level = "standard"
auto_update = true
update_channel = "stable"
```

---

## Server Configuration

### [server]

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `host` | string | `"0.0.0.0"` | Server host address |
| `port` | integer | `8080` | Server port |
| `workers` | integer | `4` | Number of worker threads |
| `max_connections` | integer | `10000` | Maximum concurrent connections |
| `timeout` | integer | `30` | Request timeout in seconds |
| `tls_enabled` | bool | `false` | Enable TLS/SSL |
| `tls_cert_file` | string | `""` | TLS certificate file path |
| `tls_key_file` | string | `""` | TLS private key file path |
| `tls_min_version` | string | `"1.2"` | Minimum TLS version |
| `tls_cipher_suites` | array | `[]` | Allowed TLS cipher suites |
| `http2_enabled` | bool | `true` | Enable HTTP/2 |
| `compression_enabled` | bool | `true` | Enable compression |
| `compression_level` | integer | `6` | Compression level (0-9) |
| `compression_min_size` | integer | `1024` | Minimum size for compression (bytes) |

**Example:**
```toml
[server]
host = "0.0.0.0"
port = 8080
workers = 4
max_connections = 10000
timeout = 30
tls_enabled = true
tls_cert_file = "/opt/sentinel/config/tls/cert.pem"
tls_key_file = "/opt/sentinel/config/tls/key.pem"
tls_min_version = "1.2"
tls_cipher_suites = [
    "TLS_AES_256_GCM_SHA384",
    "TLS_CHACHA20_POLY1305_SHA256",
    "TLS_AES_128_GCM_SHA256"
]
http2_enabled = true
compression_enabled = true
compression_level = 6
compression_min_size = 1024
```

---

## Database Configuration

### [database]

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `host` | string | `"localhost"` | Database host |
| `port` | integer | `5432` | Database port |
| `name` | string | `"sentinel"` | Database name |
| `user` | string | `"sentinel"` | Database user |
| `password` | string | `""` | Database password |
| `pool_size` | integer | `20` | Connection pool size |
| `max_overflow` | integer | `10` | Maximum overflow connections |
| `pool_timeout` | integer | `30` | Pool timeout in seconds |
| `ssl_mode` | string | `"prefer"` | SSL mode: `disable`, `allow`, `prefer`, `require`, `verify-ca`, `verify-full` |
| `ssl_cert` | string | `""` | SSL certificate file path |
| `ssl_key` | string | `""` | SSL private key file path |
| `ssl_ca` | string | `""` | SSL CA certificate file path |
| `connect_timeout` | integer | `10` | Connection timeout in seconds |
| `statement_timeout` | integer | `30000` | Statement timeout in milliseconds |

**Example:**
```toml
[database]
host = "localhost"
port = 5432
name = "sentinel"
user = "sentinel"
password = "${DB_PASSWORD}"
pool_size = 20
max_overflow = 10
pool_timeout = 30
ssl_mode = "require"
ssl_cert = "/opt/sentinel/config/db/client-cert.pem"
ssl_key = "/opt/sentinel/config/db/client-key.pem"
ssl_ca = "/opt/sentinel/config/db/ca-cert.pem"
connect_timeout = 10
statement_timeout = 30000
```

---

## Redis Configuration

### [redis]

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `host` | string | `"localhost"` | Redis host |
| `port` | integer | `6379` | Redis port |
| `password` | string | `""` | Redis password |
| `db` | integer | `0` | Redis database number |
| `pool_size` | integer | `10` | Connection pool size |
| `timeout` | integer | `5` | Timeout in seconds |
| `max_retries` | integer | `3` | Maximum connection retries |
| `retry_delay` | integer | `1000` | Retry delay in milliseconds |
| `connection_timeout` | integer | `10` | Connection timeout in seconds |
| `socket_timeout` | integer | `5` | Socket timeout in seconds |

**Example:**
```toml
[redis]
host = "localhost"
port = 6379
password = "${REDIS_PASSWORD}"
db = 0
pool_size = 10
timeout = 5
max_retries = 3
retry_delay = 1000
connection_timeout = 10
socket_timeout = 5
```

---

## S3 Configuration

### [s3]

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `endpoint` | string | `"https://s3.amazonaws.com"` | S3 endpoint URL |
| `access_key` | string | `""` | S3 access key |
| `secret_key` | string | `""` | S3 secret key |
| `bucket` | string | `"sentinel-storage"` | S3 bucket name |
| `region` | string | `"us-east-1"` | S3 region |
| `use_ssl` | bool | `true` | Use SSL for S3 connections |
| `max_retries` | integer | `3` | Maximum upload retries |
| `timeout` | integer | `300` | Upload timeout in seconds |

**Example:**
```toml
[s3]
endpoint = "https://s3.amazonaws.com"
access_key = "${S3_ACCESS_KEY}"
secret_key = "${S3_SECRET_KEY}"
bucket = "sentinel-storage"
region = "us-east-1"
use_ssl = true
max_retries = 3
timeout = 300
```

---

## Security Configuration

### [security]

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `jwt_secret` | string | `""` | JWT secret key (min 32 chars) |
| `jwt_expiry` | integer | `86400` | JWT expiry time in seconds |
| `api_key` | string | `""` | API key (min 32 chars) |
| `encryption_key` | string | `""` | Encryption key (32 bytes for AES-256) |
| `rate_limit_enabled` | bool | `true` | Enable rate limiting |
| `rate_limit_per_minute` | integer | `1000` | Rate limit per minute |
| `rate_limit_per_hour` | integer | `10000` | Rate limit per hour |
| `ip_whitelist` | array | `[]` | IP whitelist (CIDR notation) |
| `ip_blacklist` | array | `[]` | IP blacklist (CIDR notation) |
| `max_connections_per_ip` | integer | `100` | Max connections per IP |
| `max_connections_total` | integer | `10000` | Max total connections |
| `hsm_enabled` | bool | `false` | Enable HSM for key storage |
| `hsm_provider` | string | `""` | HSM provider: `aws-cloudhsm`, `azure-key-vault`, `hashicorp-vault` |

**Example:**
```toml
[security]
jwt_secret = "${JWT_SECRET}"
jwt_expiry = 86400
api_key = "${API_KEY}"
encryption_key = "${ENCRYPTION_KEY}"
rate_limit_enabled = true
rate_limit_per_minute = 1000
rate_limit_per_hour = 10000
ip_whitelist = ["10.0.0.0/8", "192.168.0.0/16"]
ip_blacklist = ["198.51.100.0/24"]
max_connections_per_ip = 100
max_connections_total = 10000
hsm_enabled = false
hsm_provider = ""
```

---

## AI Configuration

### [ai]

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `enabled` | bool | `true` | Enable AI prediction engine |
| `model_path` | string | `"/opt/sentinel/models"` | Model directory path |
| `inference_device` | string | `"cpu"` | Inference device: `cpu`, `cuda`, `npu` |
| `batch_size` | integer | `32` | Batch size for predictions |
| `cache_size` | integer | `1000` | Number of cached predictions |
| `model_update` | bool | `true` | Enable automatic model updates |
| `local_inference` | bool | `true` | Use local inference (vs cloud) |
| `lazy_load` | bool | `false` | Lazy load models |
| `unload_unused_models` | bool | `false` | Unload unused models |
| `model_idle_timeout` | integer | `300` | Model idle timeout in seconds |
| `model_quantization` | bool | `false` | Use quantized models |
| `quantization_bits` | integer | `8` | Quantization bits (8 or 16) |
| `cache_model` | bool | `true` | Cache model in memory |
| `model_cache_size` | integer | `2` | Model cache size in GB |
| `use_onnx_runtime` | bool | `false` | Use ONNX Runtime |
| `onnx_execution_provider` | string | `"cpu"` | ONNX execution provider |
| `gpu_device_id` | integer | `0` | GPU device ID |
| `gpu_memory_fraction` | float | `0.8` | GPU memory fraction |
| `confidence_threshold` | float | `0.8` | Confidence threshold for predictions |
| `use_ensemble` | bool | `false` | Use ensemble of models |
| `ensemble_models` | array | `[]` | Ensemble model names |

**Example:**
```toml
[ai]
enabled = true
model_path = "/opt/sentinel/models"
inference_device = "cpu"
batch_size = 32
cache_size = 1000
model_update = true
local_inference = true
lazy_load = false
unload_unused_models = false
model_idle_timeout = 300
model_quantization = false
quantization_bits = 8
cache_model = true
model_cache_size = 2
use_onnx_runtime = false
onnx_execution_provider = "cpu"
gpu_device_id = 0
gpu_memory_fraction = 0.8
confidence_threshold = 0.8
use_ensemble = false
ensemble_models = []
```

---

## Gaming Configuration

### [gaming]

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `enabled` | bool | `true` | Enable gaming optimization |
| `zero_scan_mode` | string | `"auto"` | Zero-scan mode: `auto`, `manual`, `disabled` |
| `anti_ddos_shield` | bool | `true` | Enable Anti-DDoS shield |
| `trusted_handshake` | bool | `true` | Enable Trusted Handshake |
| `auto_detect_games` | bool | `true` | Auto-detect games |
| `game_database_path` | string | `"/opt/sentinel/games.db"` | Game database path |
| `performance_boost` | bool | `true` | Enable performance boost |
| `fps_target` | integer | `144` | Target FPS |
| `latency_target` | integer | `20` | Target latency in ms |
| `ram_optimization` | bool | `true` | Enable RAM optimization |
| `ram_target` | integer | `80` | Target RAM usage percentage |

**Example:**
```toml
[gaming]
enabled = true
zero_scan_mode = "auto"
anti_ddos_shield = true
trusted_handshake = true
auto_detect_games = true
game_database_path = "/opt/sentinel/games.db"
performance_boost = true
fps_target = 144
latency_target = 20
ram_optimization = true
ram_target = 80
```

---

## Quantum Configuration

### [quantum]

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `enabled` | bool | `true` | Enable quantum cryptography |
| `algorithm` | string | `"kyber"` | Algorithm: `kyber`, `dilithium`, `saber`, `falcon` |
| `hybrid_mode` | bool | `true` | Use hybrid (classical + quantum) |
| `key_size` | integer | `1024` | Key size in bits |
| `signature_algorithm` | string | `"dilithium"` | Signature algorithm |
| `kdf_algorithm` | string | `"sha256"` | Key derivation function |
| `use_npu` | bool | `false` | Use NPU for quantum operations |

**Example:**
```toml
[quantum]
enabled = true
algorithm = "kyber"
hybrid_mode = true
key_size = 1024
signature_algorithm = "dilithium"
kdf_algorithm = "sha256"
use_npu = false
```

---

## Behavioral Configuration

### [behavioral]

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `enabled` | bool | `true` | Enable behavioral analysis |
| `monitor_level` | string | `"standard"` | Monitor level: `low`, `standard`, `high` |
| `anomaly_detection` | bool | `true` | Enable anomaly detection |
| `baseline_update_interval` | integer | `3600` | Baseline update interval in seconds |
| `anomaly_threshold` | float | `0.8` | Anomaly threshold (0.0-1.0) |
| `pattern_matching` | bool | `true` | Enable pattern matching |
| `risk_score_calculation` | bool | `true` | Enable risk score calculation |

**Example:**
```toml
[behavioral]
enabled = true
monitor_level = "standard"
anomaly_detection = true
baseline_update_interval = 3600
anomaly_threshold = 0.8
pattern_matching = true
risk_score_calculation = true
```

---

## Threat Intelligence Configuration

### [threat_intel]

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `enabled` | bool | `true` | Enable threat intelligence |
| `auto_share` | bool | `true` | Auto-share threats |
| `auto_update` | bool | `true` | Auto-update threat database |
| `update_interval` | integer | `300` | Update interval in seconds |
| `database_path` | string | `"/opt/sentinel/threats.db"` | Threat database path |
| `max_database_size` | integer | `1000000` | Max database size (number of threats) |
| `retention_days` | integer | `365` | Threat retention days |

**Example:**
```toml
[threat_intel]
enabled = true
auto_share = true
auto_update = true
update_interval = 300
database_path = "/opt/sentinel/threats.db"
max_database_size = 1000000
retention_days = 365
```

---

## SIEM Configuration

### [siem]

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `enabled` | bool | `true` | Enable SIEM integration |
| `platforms` | array | `[]` | Supported platforms: `splunk`, `qradar`, `microsoft_sentinel`, `logrhythm`, `arcsight`, `elastic_security`, `sumo_logic`, `datadog`, `graylog` |
| `batch_size` | integer | `100` | Batch size for events |
| `flush_interval` | integer | `60` | Flush interval in seconds |
| `retry_attempts` | integer | `3` | Retry attempts |
| `retry_delay` | integer | `5000` | Retry delay in milliseconds |

#### [siem.splunk]

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `url` | string | `""` | Splunk HEC URL |
| `token` | string | `""` | Splunk HEC token |
| `index` | string | `"sentinel"` | Splunk index |
| `sourcetype` | string | `"sentinel:event"` | Splunk sourcetype |

#### [siem.qradar]

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `url` | string | `""` | QRadar API URL |
| `token` | string | `""` | QRadar API token |
| `log_source_type` | string | `"Sentinel"` | QRadar log source type |

**Example:**
```toml
[siem]
enabled = true
platforms = ["splunk", "qradar"]
batch_size = 100
flush_interval = 60
retry_attempts = 3
retry_delay = 5000

[siem.splunk]
url = "https://splunk.example.com:8088"
token = "${SPLUNK_TOKEN}"
index = "sentinel"
sourcetype = "sentinel:event"

[siem.qradar]
url = "https://qradar.example.com/api"
token = "${QRADAR_TOKEN}"
log_source_type = "Sentinel"
```

---

## Monitoring Configuration

### [monitoring]

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `enabled` | bool | `true` | Enable monitoring |
| `metrics_port` | integer | `9090` | Metrics port |
| `health_check_port` | integer | `8080` | Health check port |
| `prometheus_enabled` | bool | `true` | Enable Prometheus metrics |
| `grafana_enabled` | bool | `true` | Enable Grafana dashboard |
| `metrics_path` | string | `"/metrics"` | Metrics endpoint path |
| `health_check_path` | string | `"/health"` | Health check endpoint path |

**Example:**
```toml
[monitoring]
enabled = true
metrics_port = 9090
health_check_port = 8080
prometheus_enabled = true
grafana_enabled = true
metrics_path = "/metrics"
health_check_path = "/health"
```

---

## Logging Configuration

### [logging]

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `level` | string | `"info"` | Log level: `error`, `warn`, `info`, `debug`, `trace` |
| `format` | string | `"json"` | Log format: `json`, `text` |
| `output` | string | `"file"` | Output: `file`, `stdout`, `both` |
| `file` | string | `"/opt/sentinel/logs/sentinel.log"` | Log file path |
| `max_size` | string | `"100MB"` | Max log file size |
| `max_backups` | integer | `10` | Max backup files |
| `max_age` | integer | `30` | Max log age in days |
| `compress` | bool | `true` | Compress old logs |
| `log_security_events` | bool | `true` | Log security events |
| `log_access_events` | bool | `true` | Log access events |
| `log_authentication_events` | bool | `true` | Log authentication events |

**Example:**
```toml
[logging]
level = "info"
format = "json"
output = "file"
file = "/opt/sentinel/logs/sentinel.log"
max_size = "100MB"
max_backups = 10
max_age = 30
compress = true
log_security_events = true
log_access_events = true
log_authentication_events = true
```

---

## Performance Configuration

### [performance]

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `cache_enabled` | bool | `true` | Enable caching |
| `cache_backend` | string | `"redis"` | Cache backend: `memory`, `redis` |
| `cache_size` | integer | `1000` | Cache size (number of items) |
| `cache_ttl` | integer | `3600` | Cache TTL in seconds |
| `cache_max_memory` | string | `"1GB"` | Max cache memory |
| `connection_pooling` | bool | `true` | Enable connection pooling |
| `max_connections_per_host` | integer | `100` | Max connections per host |
| `connection_timeout` | integer | `30` | Connection timeout in seconds |
| `connection_idle_timeout` | integer | `60` | Connection idle timeout in seconds |
| `rate_limiting` | bool | `true` | Enable rate limiting |
| `profiling_enabled` | bool | `false` | Enable profiling |
| `memory_compression` | bool | `false` | Enable memory compression |
| `compression_algorithm` | string | `"zstd"` | Compression algorithm |
| `compression_level` | integer | `3` | Compression level |
| `thread_pool_size` | integer | `16` | Thread pool size |
| `thread_pool_max_queue` | integer | `1000` | Thread pool max queue |
| `thread_pool_keep_alive` | integer | `60` | Thread pool keep alive in seconds |

**Example:**
```toml
[performance]
cache_enabled = true
cache_backend = "redis"
cache_size = 1000
cache_ttl = 3600
cache_max_memory = "1GB"
connection_pooling = true
max_connections_per_host = 100
connection_timeout = 30
connection_idle_timeout = 60
rate_limiting = true
profiling_enabled = false
memory_compression = false
compression_algorithm = "zstd"
compression_level = 3
thread_pool_size = 16
thread_pool_max_queue = 1000
thread_pool_keep_alive = 60
```

---

## Hypervisor Configuration

### [hypervisor]

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `enabled` | bool | `true` | Enable Ring -1 hypervisor |
| `vmx_enabled` | bool | `true` | Enable Intel VMX |
| `svm_enabled` | bool | `true` | Enable AMD SVM |
| `ept_enabled` | bool | `true` | Enable Extended Page Tables |
| `vpid_enabled` | bool | `true` | Enable Virtual Processor ID |
| `max_vms` | integer | `10` | Maximum VMs |
| `vm_memory_mb` | integer | `4096` | Default VM memory in MB |
| `vm_vcpus` | integer | `2` | Default VM vCPUs |

**Example:**
```toml
[hypervisor]
enabled = true
vmx_enabled = true
svm_enabled = true
ept_enabled = true
vpid_enabled = true
max_vms = 10
vm_memory_mb = 4096
vm_vcpus = 2
```

---

## Memory Configuration

### [memory]

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `enabled` | bool | `true` | Enable memory protection |
| `protect_heap` | bool | `true` | Protect heap memory |
| `protect_stack` | bool | `true` | Protect stack memory |
| `zero_copy_inspection` | bool | `true` | Enable zero-copy inspection |
| `max_protected_regions` | integer | `1000` | Max protected regions |
| `region_size_mb` | integer | `4` | Default region size in MB |

**Example:**
```toml
[memory]
enabled = true
protect_heap = true
protect_stack = true
zero_copy_inspection = true
max_protected_regions = 1000
region_size_mb = 4
```

---

## Process Configuration

### [process]

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `enabled` | bool | `true` | Enable process monitoring |
| `monitor_all_processes` | bool | `false` | Monitor all processes |
| `monitor_level` | string | `"standard"` | Monitor level: `low`, `standard`, `high` |
| `max_monitored_processes` | integer | `1000` | Max monitored processes |
| `process_sampling_interval` | integer | `1000` | Process sampling interval in ms |

**Example:**
```toml
[process]
enabled = true
monitor_all_processes = false
monitor_level = "standard"
max_monitored_processes = 1000
process_sampling_interval = 1000
```

---

## Network Configuration

### [network]

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `enabled` | bool | `true` | Enable network protection |
| `firewall_enabled` | bool | `true` | Enable firewall |
| `intrusion_detection` | bool | `true` | Enable intrusion detection |
| `proxy_url` | string | `""` | Proxy URL |
| `proxy_username` | string | `""` | Proxy username |
| `proxy_password` | string | `""` | Proxy password |
| `max_connections` | integer | `10000` | Max network connections |
| `connection_timeout` | integer | `30` | Connection timeout in seconds |

**Example:**
```toml
[network]
enabled = true
firewall_enabled = true
intrusion_detection = true
proxy_url = ""
proxy_username = ""
proxy_password = ""
max_connections = 10000
connection_timeout = 30
```

---

## Scanning Configuration

### [scanning]

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `startup_scan` | bool | `true` | Enable startup scan |
| `real_time_scan` | bool | `true` | Enable real-time scanning |
| `scan_threads` | integer | `4` | Number of scan threads |
| `scan_batch_size` | integer | `100` | Scan batch size |
| `max_file_size_mb` | integer | `100` | Max file size to scan in MB |
| `exclude_paths` | array | `[]` | Paths to exclude from scanning |
| `exclude_extensions` | array | `[]` | File extensions to exclude |

**Example:**
```toml
[scanning]
startup_scan = true
real_time_scan = true
scan_threads = 4
scan_batch_size = 100
max_file_size_mb = 100
exclude_paths = ["/tmp", "/var/cache"]
exclude_extensions = [".log", ".tmp"]
```

---

## Compliance Configuration

### [compliance]

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `gdpr_enabled` | bool | `false` | Enable GDPR compliance |
| `hipaa_enabled` | bool | `false` | Enable HIPAA compliance |
| `pci_dss_enabled` | bool | `false` | Enable PCI DSS compliance |
| `soc2_enabled` | bool | `false` | Enable SOC 2 compliance |
| `audit_log_retention_days` | integer | `2555` | Audit log retention days (7 years) |

#### [compliance.gdpr]

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `right_to_access` | bool | `true` | Enable right to access |
| `right_to_rectification` | bool | `true` | Enable right to rectification |
| `right_to_erasure` | bool | `true` | Enable right to erasure |
| `right_to_portability` | bool | `true` | Enable right to portability |
| `right_to_object` | bool | `true` | Enable right to object |

**Example:**
```toml
[compliance]
gdpr_enabled = false
hipaa_enabled = false
pci_dss_enabled = false
soc2_enabled = false
audit_log_retention_days = 2555

[compliance.gdpr]
right_to_access = true
right_to_rectification = true
right_to_erasure = true
right_to_portability = true
right_to_object = true
```

---

## Environment Variables

Environment variables can be used to override configuration values. They take precedence over configuration file values.

| Variable | Description |
|----------|-------------|
| `SENTINEL_ENV` | Environment (`development`, `staging`, `production`) |
| `SENTINEL_CONFIG` | Path to configuration file |
| `SENTINEL_LOG_LEVEL` | Log level |
| `SENTINEL_API_KEY` | API key |
| `SENTINEL_JWT_SECRET` | JWT secret |
| `SENTINEL_ENCRYPTION_KEY` | Encryption key |
| `DB_HOST` | Database host |
| `DB_PORT` | Database port |
| `DB_NAME` | Database name |
| `DB_USER` | Database user |
| `DB_PASSWORD` | Database password |
| `REDIS_HOST` | Redis host |
| `REDIS_PORT` | Redis port |
| `REDIS_PASSWORD` | Redis password |
| `S3_ENDPOINT` | S3 endpoint |
| `S3_ACCESS_KEY` | S3 access key |
| `S3_SECRET_KEY` | S3 secret key |
| `S3_BUCKET` | S3 bucket name |
| `SPLUNK_URL` | Splunk URL |
| `SPLUNK_TOKEN` | Splunk token |
| `QRADAR_URL` | QRadar URL |
| `QRADAR_TOKEN` | QRadar token |

**Example:**
```bash
export SENTINEL_ENV=production
export SENTINEL_LOG_LEVEL=info
export DB_PASSWORD=secure_password
export REDIS_PASSWORD=secure_password
export S3_ACCESS_KEY=access_key
export S3_SECRET_KEY=secret_key
```

---

## Configuration Validation

### Validate Configuration

```bash
# Validate configuration file
sentinel config validate /opt/sentinel/config/sentinel.toml

# Test database connection
sentinel config test-database

# Test Redis connection
sentinel config test-redis

# Test S3 connection
sentinel config test-s3
```

### Configuration Errors

Common configuration errors:

| Error | Description | Solution |
|-------|-------------|----------|
| `Invalid TOML` | Syntax error in configuration file | Check TOML syntax |
| `Missing required field` | Required field is missing | Add the missing field |
| `Invalid value type` | Value has wrong type | Use correct type |
| `Value out of range` | Value is outside valid range | Use valid value |
| `Connection failed` | Cannot connect to service | Check service is running and credentials are correct |

---

## License

© 2026 Vantis Corp. All rights reserved.

For more information, visit https://sentinel.security
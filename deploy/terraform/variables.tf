# SENTINEL Security System - Terraform Variables

variable "aws_region" {
  description = "AWS region for deployment"
  type        = string
  default     = "us-east-1"

  validation {
    condition     = contains(["us-east-1", "us-east-2", "us-west-1", "us-west-2", "eu-west-1", "eu-central-1"], var.aws_region)
    error_message = "Region must be one of: us-east-1, us-east-2, us-west-1, us-west-2, eu-west-1, eu-central-1"
  }
}

variable "environment" {
  description = "Environment name (production, staging, development)"
  type        = string
  default     = "production"

  validation {
    condition     = contains(["production", "staging", "development"], var.environment)
    error_message = "Environment must be one of: production, staging, development"
  }
}

variable "vpc_cidr" {
  description = "CIDR block for VPC"
  type        = string
  default     = "10.0.0.0/16"

  validation {
    condition     = can(cidrhost(var.vpc_cidr, 0))
    error_message = "VPC CIDR must be a valid CIDR block"
  }
}

variable "availability_zones" {
  description = "List of availability zones"
  type        = list(string)
  default     = ["us-east-1a", "us-east-1b", "us-east-1c"]
}

variable "domain_name" {
  description = "Domain name for the application"
  type        = string
  default     = "sentinel.security"
}

variable "db_password" {
  description = "Database password (should be stored in AWS Secrets Manager)"
  type        = string
  sensitive   = true
}

variable "redis_password" {
  description = "Redis password (should be stored in AWS Secrets Manager)"
  type        = string
  sensitive   = true
}

variable "jwt_secret" {
  description = "JWT secret key (should be stored in AWS Secrets Manager)"
  type        = string
  sensitive   = true
}

variable "api_key" {
  description = "API key (should be stored in AWS Secrets Manager)"
  type        = string
  sensitive   = true
}

variable "encryption_key" {
  description = "Encryption key (should be stored in AWS Secrets Manager)"
  type        = string
  sensitive   = true
}

variable "splunk_token" {
  description = "Splunk HEC token (should be stored in AWS Secrets Manager)"
  type        = string
  sensitive   = true
}

variable "qradar_token" {
  description = "QRadar API token (should be stored in AWS Secrets Manager)"
  type        = string
  sensitive   = true
}

variable "enable_monitoring" {
  description = "Enable CloudWatch monitoring and alarms"
  type        = bool
  default     = true
}

variable "enable_backup" {
  description = "Enable automated backups"
  type        = bool
  default     = true
}

variable "enable_security_hub" {
  description = "Enable AWS Security Hub"
  type        = bool
  default     = true
}

variable "enable_guardduty" {
  description = "Enable AWS GuardDuty"
  type        = bool
  default     = true
}

variable "enable_config" {
  description = "Enable AWS Config"
  type        = bool
  default     = true
}

variable "enable_cloudtrail" {
  description = "Enable AWS CloudTrail"
  type        = bool
  default     = true
}
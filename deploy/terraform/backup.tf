# SENTINEL Security System - Backup Strategy Configuration

# AWS Backup Plan
resource "aws_backup_plan" "main" {
  name = "${var.environment}-backup-plan"

  rule {
    rule_name         = "daily-backup"
    target_vault_name = aws_backup_vault.main.name
    schedule          = "cron(0 2 * * ? *)"

    lifecycle {
      delete_after = 30
    }

    copy_action {
      destination_vault_arn = aws_backup_vault.dr.arn
      lifecycle {
        delete_after = 90
      }
    }
  }

  advanced_backup_setting {
    resource_type = "EFS"
  }

  tags = {
    Name = "${var.environment}-backup-plan"
  }
}

# Backup Vault
resource "aws_backup_vault" "main" {
  name = "${var.environment}-backup-vault"

  tags = {
    Name = "${var.environment}-backup-vault"
  }
}

resource "aws_backup_vault" "dr" {
  name = "${var.environment}-backup-vault-dr"

  tags = {
    Name = "${var.environment}-backup-vault-dr"
  }
}

# Backup Selection for RDS
resource "aws_backup_selection" "rds" {
  iam_role_arn = aws_iam_role.backup.arn
  name         = "${var.environment}-rds-backup"
  plan_id      = aws_backup_plan.main.id

  resources = [
    "arn:aws:rds:${var.aws_region}:${data.aws_caller_identity.current.account_id}:db:${var.environment}-db"
  ]
}

# Backup Selection for EFS
resource "aws_backup_selection" "efs" {
  iam_role_arn = aws_iam_role.backup.arn
  name         = "${var.environment}-efs-backup"
  plan_id      = aws_backup_plan.main.id

  resources = [
    "arn:aws:elasticfilesystem:${var.aws_region}:${data.aws_caller_identity.current.account_id}:file-system/*"
  ]
}

# IAM Role for Backup
resource "aws_iam_role" "backup" {
  name = "${var.environment}-backup-role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = "sts:AssumeRole"
        Effect = "Allow"
        Principal = {
          Service = "backup.amazonaws.com"
        }
      }
    ]
  })

  tags = {
    Name = "${var.environment}-backup-role"
  }
}

resource "aws_iam_role_policy_attachment" "backup" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSBackupServiceRolePolicyForBackup"
  role       = aws_iam_role.backup.name
}

# S3 Lifecycle for Backups
resource "aws_s3_bucket_lifecycle_configuration" "backups" {
  bucket = aws_s3_bucket.backups.id

  rule {
    id     = "backup-lifecycle"
    status = "Enabled"

    transition {
      days          = 30
      storage_class = "STANDARD_IA"
    }

    transition {
      days          = 90
      storage_class = "GLACIER"
    }

    expiration {
      days = 365
    }
  }
}

# RDS Automated Backups
resource "aws_db_instance" "main" {
  # ... existing configuration ...

  backup_retention_period = 30
  backup_window          = "03:00-04:00"
  skip_final_snapshot    = false
  final_snapshot_identifier = "${var.environment}-db-final-snapshot"
}

# Backup Scripts
resource "aws_ssm_document" "backup_script" {
  name          = "${var.environment}-backup-script"
  document_type = "Command"
  document_format = "YAML"

  content = <<DOC
---
schemaVersion: '2.2'
description: 'Run backup script for SENTINEL'
parameters:
  BackupType:
    type: String
    description: 'Type of backup (full, incremental)'
    allowedValues:
      - full
      - incremental
    default: full
mainSteps:
  - action: aws:runShellScript
    name: runBackup
    inputs:
      runCommand:
        - |
          #!/bin/bash
          BACKUP_TYPE="{{ BackupType }}"
          BACKUP_DIR="/backup/sentinel"
          DATE=$(date +%Y%m%d_%H%M%S)
          S3_BUCKET="s3://sentinel-production-backups"
          
          # Create backup directory
          mkdir -p $BACKUP_DIR/$DATE
          
          # Backup database
          if [ "$BACK_TYPE" = "full" ]; then
            pg_dump -h $DB_HOST -U $DB_USER -d $DB_NAME | gzip > $BACKUP_DIR/$DATE/database.sql.gz
          fi
          
          # Backup configuration
          tar -czf $BACKUP_DIR/$DATE/config.tar.gz /opt/sentinel/config/
          
          # Backup logs
          find /var/log/sentinel -name "*.log" -mtime -7 -exec tar -czf $BACKUP_DIR/$DATE/logs.tar.gz {} +
          
          # Backup models
          tar -czf $BACKUP_DIR/$DATE/models.tar.gz /opt/sentinel/models/
          
          # Upload to S3
          aws s3 sync $BACKUP_DIR/$DATE $S3_BUCKET/$DATE/
          
          # Cleanup old backups
          find $BACKUP_DIR -type d -mtime +30 -exec rm -rf {} \;
          
          echo "Backup completed: $DATE"
DOC

  tags = {
    Name = "${var.environment}-backup-script"
  }
}

# Scheduled Backup
resource "aws_ssm_maintenance_window" "backup" {
  name      = "${var.environment}-backup-window"
  schedule  = "cron(0 2 * * ? *)"
  duration  = 2
  cutoff    = 1

  tags = {
    Name = "${var.environment}-backup-window"
  }
}

resource "aws_ssm_maintenance_window_task" "backup" {
  name            = "${var.environment}-backup-task"
  window_id       = aws_ssm_maintenance_window.backup.id
  task_arn        = aws_ssm_document.backup_script.arn
  task_type       = "RUN_COMMAND"
  max_concurrency = 1
  max_errors      = 1

  targets {
    key    = "tag:Environment"
    values = [var.environment]
  }

  task_parameters = {
    BackupType = ["full"]
  }

  tags = {
    Name = "${var.environment}-backup-task"
  }
}

# Backup Monitoring
resource "aws_cloudwatch_metric_alarm" "backup_failed" {
  alarm_name          = "${var.environment}-backup-failed"
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = "1"
  metric_name         = "BackupFailed"
  namespace           = "Sentinel/Backup"
  period              = 86400
  statistic           = "Sum"
  threshold           = "0"
  alarm_description   = "This metric monitors backup failures"
  alarm_actions       = [aws_sns_topic.alerts.arn]

  tags = {
    Name = "${var.environment}-backup-failed"
  }
}

resource "aws_cloudwatch_metric_alarm" "backup_age" {
  alarm_name          = "${var.environment}-backup-age"
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = "1"
  metric_name         = "BackupAge"
  namespace           = "Sentinel/Backup"
  period              = 86400
  statistic           = "Maximum"
  threshold           = "172800"
  alarm_description   = "This metric monitors backup age (48 hours)"
  alarm_actions       = [aws_sns_topic.alerts.arn]

  tags = {
    Name = "${var.environment}-backup-age"
  }
}
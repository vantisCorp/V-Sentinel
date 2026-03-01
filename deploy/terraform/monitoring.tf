# SENTINEL Security System - CloudWatch Monitoring Configuration

# CloudWatch Alarms
resource "aws_cloudwatch_metric_alarm" "high_cpu_usage" {
  alarm_name          = "${var.environment}-high-cpu-usage"
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = "3"
  metric_name         = "CPUUtilization"
  namespace           = "AWS/ECS"
  period              = "300"
  statistic           = "Average"
  threshold           = "80"
  alarm_description   = "This metric monitors ECS CPU utilization"
  alarm_actions       = [aws_sns_topic.alerts.arn]
  ok_actions          = [aws_sns_topic.alerts.arn]

  dimensions = {
    ClusterName = aws_ecs_cluster.main.name
    ServiceName = aws_ecs_service.api.name
  }

  tags = {
    Name = "${var.environment}-high-cpu-usage"
  }
}

resource "aws_cloudwatch_metric_alarm" "high_memory_usage" {
  alarm_name          = "${var.environment}-high-memory-usage"
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = "3"
  metric_name         = "MemoryUtilization"
  namespace           = "CWAgent"
  period              = "300"
  statistic           = "Average"
  threshold           = "90"
  alarm_description   = "This metric monitors memory utilization"
  alarm_actions       = [aws_sns_topic.alerts.arn]
  ok_actions          = [aws_sns_topic.alerts.arn]

  tags = {
    Name = "${var.environment}-high-memory-usage"
  }
}

resource "aws_cloudwatch_metric_alarm" "high_api_latency" {
  alarm_name          = "${var.environment}-high-api-latency"
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = "5"
  metric_name         = "APILatency"
  namespace           = "Sentinel"
  period              = "60"
  statistic           = "Average"
  threshold           = "0.5"
  alarm_description   = "This metric monitors API latency"
  alarm_actions       = [aws_sns_topic.alerts.arn]
  ok_actions          = [aws_sns_topic.alerts.arn]

  tags = {
    Name = "${var.environment}-high-api-latency"
  }
}

resource "aws_cloudwatch_metric_alarm" "high_error_rate" {
  alarm_name          = "${var.environment}-high-error-rate"
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = "3"
  metric_name         = "ErrorRate"
  namespace           = "Sentinel"
  period              = "60"
  statistic           = "Average"
  threshold           = "5"
  alarm_description   = "This metric monitors error rate"
  alarm_actions       = [aws_sns_topic.alerts.arn]
  ok_actions          = [aws_sns_topic.alerts.arn]

  tags = {
    Name = "${var.environment}-high-error-rate"
  }
}

resource "aws_cloudwatch_metric_alarm" "db_connection_failures" {
  alarm_name          = "${var.environment}-db-connection-failures"
  comparison_operator = "LessThanThreshold"
  evaluation_periods  = "3"
  metric_name         = "DatabaseConnections"
  namespace           = "AWS/RDS"
  period              = "60"
  statistic           = "Average"
  threshold           = "1"
  alarm_description   = "This metric monitors database connections"
  alarm_actions       = [aws_sns_topic.alerts.arn]
  ok_actions          = [aws_sns_topic.alerts.arn]

  dimensions = {
    DBInstanceIdentifier = aws_db_instance.main.identifier
  }

  tags = {
    Name = "${var.environment}-db-connection-failures"
  }
}

resource "aws_cloudwatch_metric_alarm" "disk_space_low" {
  alarm_name          = "${var.environment}-disk-space-low"
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = "2"
  metric_name         = "DiskSpaceUtilization"
  namespace           = "CWAgent"
  period              = "300"
  statistic           = "Average"
  threshold           = "80"
  alarm_description   = "This metric monitors disk space utilization"
  alarm_actions       = [aws_sns_topic.alerts.arn]
  ok_actions          = [aws_sns_topic.alerts.arn]

  tags = {
    Name = "${var.environment}-disk-space-low"
  }
}

resource "aws_cloudwatch_metric_alarm" "redis_memory_high" {
  alarm_name          = "${var.environment}-redis-memory-high"
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = "2"
  metric_name         = "DatabaseMemoryUsagePercentage"
  namespace           = "AWS/ElastiCache"
  period              = "300"
  statistic           = "Average"
  threshold           = "80"
  alarm_description   = "This metric monitors Redis memory usage"
  alarm_actions       = [aws_sns_topic.alerts.arn]
  ok_actions          = [aws_sns_topic.alerts.arn]

  dimensions = {
    CacheClusterId = aws_elasticache_replication_group.main.id
  }

  tags = {
    Name = "${var.environment}-redis-memory-high"
  }
}

# SNS Topic for Alerts
resource "aws_sns_topic" "alerts" {
  name = "${var.environment}-alerts"

  tags = {
    Name = "${var.environment}-alerts"
  }
}

resource "aws_sns_topic_subscription" "email" {
  topic_arn = aws_sns_topic.alerts.arn
  protocol  = "email"
  endpoint  = var.alert_email
}

# CloudWatch Dashboard
resource "aws_cloudwatch_dashboard" "main" {
  dashboard_name = "${var.environment}-dashboard"

  dashboard_body = jsonencode({
    widgets = [
      {
        type   = "metric"
        x      = 0
        y      = 0
        width  = 12
        height = 6

        properties = {
          metrics = [
            ["AWS/ECS", "CPUUtilization", "ClusterName", aws_ecs_cluster.main.name, "ServiceName", aws_ecs_service.api.name],
            [".", "CPUUtilization", ".", ".", "ServiceName", aws_ecs_service.worker.name],
            [".", "CPUUtilization", ".", ".", "ServiceName", aws_ecs_service.web.name]
          ]
          period = 300
          stat   = "Average"
          region = var.aws_region
          title  = "ECS CPU Utilization"
        }
      },
      {
        type   = "metric"
        x      = 0
        y      = 6
        width  = 12
        height = 6

        properties = {
          metrics = [
            ["Sentinel", "APILatency"],
            [".", "ErrorRate"],
            [".", "RequestsPerSecond"]
          ]
          period = 60
          stat   = "Average"
          region = var.aws_region
          title  = "API Metrics"
        }
      },
      {
        type   = "metric"
        x      = 0
        y      = 12
        width  = 12
        height = 6

        properties = {
          metrics = [
            ["AWS/RDS", "DatabaseConnections", "DBInstanceIdentifier", aws_db_instance.main.identifier],
            [".", "CPUUtilization", ".", "."],
            [".", "FreeStorageSpace", ".", "."]
          ]
          period = 60
          stat   = "Average"
          region = var.aws_region
          title  = "Database Metrics"
        }
      },
      {
        type   = "metric"
        x      = 0
        y      = 18
        width  = 12
        height = 6

        properties = {
          metrics = [
            ["AWS/ElastiCache", "DatabaseMemoryUsagePercentage", "CacheClusterId", aws_elasticache_replication_group.main.id],
            [".", "CacheHits", ".", "."],
            [".", "CacheMisses", ".", "."]
          ]
          period = 60
          stat   = "Average"
          region = var.aws_region
          title  = "Redis Metrics"
        }
      }
    ]
  })

  tags = {
    Name = "${var.environment}-dashboard"
  }
}

# Variable for alert email
variable "alert_email" {
  description = "Email address for alerts"
  type        = string
  default     = "alerts@sentinel.security"
}
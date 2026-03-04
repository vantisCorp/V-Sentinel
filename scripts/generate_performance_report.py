#!/usr/bin/env python3
"""
V-Sentinel Performance Report Generator

Generates HTML and JSON performance reports from benchmark results.
"""

import json
import os
import sys
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Any


class PerformanceReportGenerator:
    """Generates performance reports from benchmark data."""

    def __init__(self, results_dir: str = "benchmark_results"):
        self.results_dir = Path(results_dir)
        self.thresholds_file = Path("config/performance_thresholds.toml")
        self.report_data: Dict[str, Any] = {}

    def load_benchmark_results(self) -> Dict[str, Any]:
        """Load benchmark results from JSON files."""
        results = {}

        # Look for Criterion output
        criterion_dir = Path("target/criterion")
        if criterion_dir.exists():
            for bench_dir in criterion_dir.iterdir():
                if bench_dir.is_dir():
                    # Look for new/estimates.json
                    estimates_file = bench_dir / "new" / "estimates.json"
                    if estimates_file.exists():
                        with open(estimates_file) as f:
                            data = json.load(f)
                            results[bench_dir.name] = {
                                "mean_ns": data.get("mean", {}).get("point_estimate", 0),
                                "std_dev_ns": data.get("std_dev", {}).get("point_estimate", 0),
                            }

        return results

    def load_thresholds(self) -> Dict[str, Any]:
        """Load performance thresholds from TOML file."""
        thresholds = {}

        if not self.thresholds_file.exists():
            return thresholds

        # Simple TOML parsing for thresholds
        with open(self.thresholds_file) as f:
            current_section = ""
            for line in f:
                line = line.strip()
                if line.startswith("[") and line.endswith("]"):
                    current_section = line[1:-1]
                    thresholds[current_section] = {}
                elif "=" in line and current_section:
                    key, value = line.split("=", 1)
                    key = key.strip()
                    value = value.strip().split("#")[0].strip()
                    try:
                        thresholds[current_section][key] = float(value.replace("_", ""))
                    except ValueError:
                        thresholds[current_section][key] = value

        return thresholds

    def generate_summary(self, results: Dict, thresholds: Dict) -> Dict[str, Any]:
        """Generate performance summary with pass/fail status."""
        summary = {
            "timestamp": datetime.now().isoformat(),
            "total_benchmarks": len(results),
            "passed": 0,
            "failed": 0,
            "modules": {}
        }

        for bench_name, data in results.items():
            mean_ms = data.get("mean_ns", 0) / 1_000_000  # ns to ms
            std_dev_ms = data.get("std_dev_ns", 0) / 1_000_000

            # Check against threshold (simplified)
            status = "passed"  # Default to passed
            threshold = None

            # Look for matching threshold
            for section, thresh_data in thresholds.items():
                for key, value in thresh_data.items():
                    if key.replace("_ms", "") in bench_name.lower() and isinstance(value, (int, float)):
                        threshold = value
                        status = "passed" if mean_ms <= value else "failed"
                        break
                if threshold:
                    break

            if status == "passed":
                summary["passed"] += 1
            else:
                summary["failed"] += 1

            summary["modules"][bench_name] = {
                "mean_ms": round(mean_ms, 4),
                "std_dev_ms": round(std_dev_ms, 4),
                "threshold_ms": threshold,
                "status": status
            }

        return summary

    def generate_html_report(self, summary: Dict) -> str:
        """Generate HTML performance report."""
        html = f'''<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>V-Sentinel Performance Report</title>
    <style>
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}
        body {{
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            background: #0a0e17;
            color: #f3f4f6;
            min-height: 100vh;
            padding: 40px;
        }}
        .container {{
            max-width: 1200px;
            margin: 0 auto;
        }}
        .header {{
            text-align: center;
            margin-bottom: 40px;
        }}
        h1 {{
            font-size: 32px;
            margin-bottom: 10px;
        }}
        .timestamp {{
            color: #9ca3af;
        }}
        .summary {{
            display: grid;
            grid-template-columns: repeat(3, 1fr);
            gap: 20px;
            margin-bottom: 40px;
        }}
        .summary-card {{
            background: #1a2332;
            border-radius: 12px;
            padding: 24px;
            text-align: center;
            border: 1px solid #374151;
        }}
        .summary-card h3 {{
            font-size: 14px;
            color: #9ca3af;
            margin-bottom: 10px;
        }}
        .summary-card .value {{
            font-size: 36px;
            font-weight: 700;
        }}
        .summary-card.total .value {{ color: #3b82f6; }}
        .summary-card.passed .value {{ color: #10b981; }}
        .summary-card.failed .value {{ color: #ef4444; }}
        table {{
            width: 100%;
            border-collapse: collapse;
            background: #1a2332;
            border-radius: 12px;
            overflow: hidden;
        }}
        th, td {{
            padding: 16px;
            text-align: left;
            border-bottom: 1px solid #374151;
        }}
        th {{
            background: #111827;
            font-weight: 600;
            color: #9ca3af;
        }}
        .status {{
            padding: 4px 12px;
            border-radius: 20px;
            font-size: 12px;
            font-weight: 600;
        }}
        .status.passed {{
            background: rgba(16, 185, 129, 0.2);
            color: #10b981;
        }}
        .status.failed {{
            background: rgba(239, 68, 68, 0.2);
            color: #ef4444;
        }}
        .progress-bar {{
            height: 8px;
            background: #374151;
            border-radius: 4px;
            overflow: hidden;
            margin-top: 20px;
        }}
        .progress {{
            height: 100%;
            background: linear-gradient(90deg, #10b981, #3b82f6);
            border-radius: 4px;
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🚀 V-Sentinel Performance Report</h1>
            <p class="timestamp">Generated: {summary['timestamp']}</p>
        </div>

        <div class="summary">
            <div class="summary-card total">
                <h3>Total Benchmarks</h3>
                <div class="value">{summary['total_benchmarks']}</div>
            </div>
            <div class="summary-card passed">
                <h3>Passed</h3>
                <div class="value">{summary['passed']}</div>
            </div>
            <div class="summary-card failed">
                <h3>Failed</h3>
                <div class="value">{summary['failed']}</div>
            </div>
        </div>

        <div class="progress-bar">
            <div class="progress" style="width: {(summary['passed'] / max(summary['total_benchmarks'], 1)) * 100}%"></div>
        </div>

        <table>
            <thead>
                <tr>
                    <th>Benchmark</th>
                    <th>Mean (ms)</th>
                    <th>Std Dev (ms)</th>
                    <th>Threshold (ms)</th>
                    <th>Status</th>
                </tr>
            </thead>
            <tbody>
'''

        for bench_name, data in summary.get('modules', {}).items():
            status_class = "passed" if data['status'] == 'passed' else "failed"
            threshold_str = f"{data['threshold_ms']:.2f}" if data['threshold_ms'] else "N/A"
            html += f'''                <tr>
                    <td>{bench_name}</td>
                    <td>{data['mean_ms']:.4f}</td>
                    <td>{data['std_dev_ms']:.4f}</td>
                    <td>{threshold_str}</td>
                    <td><span class="status {status_class}">{data['status'].upper()}</span></td>
                </tr>
'''

        html += '''            </tbody>
        </table>
    </div>
</body>
</html>'''

        return html

    def run(self) -> None:
        """Run the report generator."""
        print("V-Sentinel Performance Report Generator")
        print("=" * 40)

        # Load data
        print("Loading benchmark results...")
        results = self.load_benchmark_results()
        print(f"Found {len(results)} benchmark results")

        print("Loading thresholds...")
        thresholds = self.load_thresholds()
        print(f"Loaded {len(thresholds)} threshold sections")

        # Generate summary
        print("Generating summary...")
        summary = self.generate_summary(results, thresholds)

        # Save JSON report
        json_path = self.results_dir / f"report_{datetime.now().strftime('%Y%m%d_%H%M%S')}.json"
        json_path.parent.mkdir(parents=True, exist_ok=True)
        with open(json_path, 'w') as f:
            json.dump(summary, f, indent=2)
        print(f"JSON report saved: {json_path}")

        # Generate and save HTML report
        html_path = self.results_dir / f"report_{datetime.now().strftime('%Y%m%d_%H%M%S')}.html"
        html_content = self.generate_html_report(summary)
        with open(html_path, 'w') as f:
            f.write(html_content)
        print(f"HTML report saved: {html_path}")

        # Print summary
        print("\n" + "=" * 40)
        print("SUMMARY")
        print("=" * 40)
        print(f"Total: {summary['total_benchmarks']}")
        print(f"Passed: {summary['passed']}")
        print(f"Failed: {summary['failed']}")

        if summary['failed'] > 0:
            print("\n⚠️  Some benchmarks failed to meet thresholds!")
            sys.exit(1)
        else:
            print("\n✅ All benchmarks passed!")
            sys.exit(0)


if __name__ == "__main__":
    generator = PerformanceReportGenerator()
    generator.run()
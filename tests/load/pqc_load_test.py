#!/usr/bin/env python3
"""
V-Sentinel PQC Load Testing Suite
Comprehensive load testing for PQC services

Usage:
    python pqc_load_test.py --config config.yaml --duration 300 --users 1000
"""

import argparse
import asyncio
import json
import logging
import statistics
import sys
import time
from dataclasses import dataclass, field
from datetime import datetime
from enum import Enum
from typing import Any, Dict, List, Optional

import aiohttp
import yaml

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)


class TestType(Enum):
    """Types of load tests"""
    GATEWAY_TLS_HANDSHAKE = "gateway_tls_handshake"
    VPN_TUNNEL_ESTABLISHMENT = "vpn_tunnel_establishment"
    MESSAGING_ENCRYPTION = "messaging_encryption"
    KEY_GENERATION = "key_generation"
    SIGNATURE_VERIFICATION = "signature_verification"
    FULL_WORKFLOW = "full_workflow"


@dataclass
class TestResult:
    """Single test result"""
    test_type: str
    start_time: float
    end_time: float
    duration_ms: float
    success: bool
    error: Optional[str] = None
    metadata: Dict[str, Any] = field(default_factory=dict)


@dataclass
class LoadTestConfig:
    """Load test configuration"""
    base_url: str
    api_key: str
    test_type: TestType
    duration_seconds: int
    concurrent_users: int
    ramp_up_seconds: int
    target_rps: Optional[int] = None
    
    @classmethod
    def from_yaml(cls, path: str) -> 'LoadTestConfig':
        with open(path, 'r') as f:
            data = yaml.safe_load(f)
        return cls(
            base_url=data['base_url'],
            api_key=data['api_key'],
            test_type=TestType(data['test_type']),
            duration_seconds=data.get('duration_seconds', 60),
            concurrent_users=data.get('concurrent_users', 100),
            ramp_up_seconds=data.get('ramp_up_seconds', 10),
            target_rps=data.get('target_rps')
        )


@dataclass
class LoadTestResults:
    """Aggregated load test results"""
    test_type: str
    total_requests: int
    successful_requests: int
    failed_requests: int
    avg_latency_ms: float
    p50_latency_ms: float
    p95_latency_ms: float
    p99_latency_ms: float
    min_latency_ms: float
    max_latency_ms: float
    requests_per_second: float
    error_rate: float
    duration_seconds: float
    
    def to_dict(self) -> Dict:
        return {
            'test_type': self.test_type,
            'total_requests': self.total_requests,
            'successful_requests': self.successful_requests,
            'failed_requests': self.failed_requests,
            'avg_latency_ms': round(self.avg_latency_ms, 2),
            'p50_latency_ms': round(self.p50_latency_ms, 2),
            'p95_latency_ms': round(self.p95_latency_ms, 2),
            'p99_latency_ms': round(self.p99_latency_ms, 2),
            'min_latency_ms': round(self.min_latency_ms, 2),
            'max_latency_ms': round(self.max_latency_ms, 2),
            'requests_per_second': round(self.requests_per_second, 2),
            'error_rate': round(self.error_rate, 4),
            'duration_seconds': round(self.duration_seconds, 2)
        }


class PqcLoadTester:
    """PQC Load Testing Engine"""
    
    def __init__(self, config: LoadTestConfig):
        self.config = config
        self.results: List[TestResult] = []
        self._session: Optional[aiohttp.ClientSession] = None
        
    async def _create_session(self) -> aiohttp.ClientSession:
        """Create HTTP session with proper configuration"""
        connector = aiohttp.TCPConnector(
            limit=self.config.concurrent_users * 2,
            limit_per_host=self.config.concurrent_users,
            enable_cleanup_closed=True
        )
        timeout = aiohttp.ClientTimeout(total=30, connect=10)
        headers = {
            'Authorization': f'Bearer {self.config.api_key}',
            'Content-Type': 'application/json',
            'X-Test-Mode': 'true'
        }
        return aiohttp.ClientSession(
            connector=connector,
            timeout=timeout,
            headers=headers
        )
    
    async def _execute_gateway_tls_test(self) -> TestResult:
        """Execute TLS handshake test against gateway"""
        start_time = time.time()
        endpoint = f"{self.config.base_url}/api/v1/pqc/handshake"
        
        try:
            async with self._session.post(endpoint, json={
                'client_hello': 'test_client_hello_data',
                'algorithms': ['kyber1024', 'dilithium5']
            }) as response:
                end_time = time.time()
                data = await response.json()
                
                return TestResult(
                    test_type='gateway_tls_handshake',
                    start_time=start_time,
                    end_time=end_time,
                    duration_ms=(end_time - start_time) * 1000,
                    success=response.status == 200,
                    metadata={
                        'status_code': response.status,
                        'algorithm': data.get('selected_algorithm'),
                        'session_id': data.get('session_id')
                    }
                )
        except Exception as e:
            end_time = time.time()
            return TestResult(
                test_type='gateway_tls_handshake',
                start_time=start_time,
                end_time=end_time,
                duration_ms=(end_time - start_time) * 1000,
                success=False,
                error=str(e)
            )
    
    async def _execute_vpn_tunnel_test(self) -> TestResult:
        """Execute VPN tunnel establishment test"""
        start_time = time.time()
        endpoint = f"{self.config.base_url}/api/v1/vpn/tunnel"
        
        try:
            async with self._session.post(endpoint, json={
                'client_public_key': 'test_public_key_data',
                'requested_algorithm': 'kyber1024',
                'client_id': f'test_client_{int(start_time * 1000)}'
            }) as response:
                end_time = time.time()
                data = await response.json()
                
                return TestResult(
                    test_type='vpn_tunnel_establishment',
                    start_time=start_time,
                    end_time=end_time,
                    duration_ms=(end_time - start_time) * 1000,
                    success=response.status == 200,
                    metadata={
                        'tunnel_id': data.get('tunnel_id'),
                        'algorithm': data.get('algorithm')
                    }
                )
        except Exception as e:
            end_time = time.time()
            return TestResult(
                test_type='vpn_tunnel_establishment',
                start_time=start_time,
                end_time=end_time,
                duration_ms=(end_time - start_time) * 1000,
                success=False,
                error=str(e)
            )
    
    async def _execute_messaging_test(self) -> TestResult:
        """Execute messaging encryption test"""
        start_time = time.time()
        endpoint = f"{self.config.base_url}/api/v1/messaging/encrypt"
        
        try:
            async with self._session.post(endpoint, json={
                'message': 'This is a test message for PQC encryption',
                'recipient_id': 'test_recipient',
                'algorithm': 'kyber1024'
            }) as response:
                end_time = time.time()
                data = await response.json()
                
                return TestResult(
                    test_type='messaging_encryption',
                    start_time=start_time,
                    end_time=end_time,
                    duration_ms=(end_time - start_time) * 1000,
                    success=response.status == 200,
                    metadata={
                        'message_id': data.get('message_id'),
                        'algorithm': data.get('algorithm')
                    }
                )
        except Exception as e:
            end_time = time.time()
            return TestResult(
                test_type='messaging_encryption',
                start_time=start_time,
                end_time=end_time,
                duration_ms=(end_time - start_time) * 1000,
                success=False,
                error=str(e)
            )
    
    async def _execute_key_generation_test(self) -> TestResult:
        """Execute key generation test"""
        start_time = time.time()
        endpoint = f"{self.config.base_url}/api/v1/keys/generate"
        
        try:
            async with self._session.post(endpoint, json={
                'kem_algorithm': 'kyber1024',
                'signature_algorithm': 'dilithium5',
                'purpose': 'test'
            }) as response:
                end_time = time.time()
                data = await response.json()
                
                return TestResult(
                    test_type='key_generation',
                    start_time=start_time,
                    end_time=end_time,
                    duration_ms=(end_time - start_time) * 1000,
                    success=response.status == 200,
                    metadata={
                        'key_id': data.get('key_id'),
                        'kem_algorithm': data.get('kem_algorithm'),
                        'signature_algorithm': data.get('signature_algorithm')
                    }
                )
        except Exception as e:
            end_time = time.time()
            return TestResult(
                test_type='key_generation',
                start_time=start_time,
                end_time=end_time,
                duration_ms=(end_time - start_time) * 1000,
                success=False,
                error=str(e)
            )
    
    async def _execute_signature_test(self) -> TestResult:
        """Execute signature verification test"""
        start_time = time.time()
        endpoint = f"{self.config.base_url}/api/v1/signatures/verify"
        
        try:
            async with self._session.post(endpoint, json={
                'message': 'Test message for signature verification',
                'signature': 'test_signature_data',
                'algorithm': 'dilithium5',
                'public_key': 'test_public_key'
            }) as response:
                end_time = time.time()
                data = await response.json()
                
                return TestResult(
                    test_type='signature_verification',
                    start_time=start_time,
                    end_time=end_time,
                    duration_ms=(end_time - start_time) * 1000,
                    success=response.status == 200,
                    metadata={
                        'verified': data.get('verified'),
                        'algorithm': data.get('algorithm')
                    }
                )
        except Exception as e:
            end_time = time.time()
            return TestResult(
                test_type='signature_verification',
                start_time=start_time,
                end_time=end_time,
                duration_ms=(end_time - start_time) * 1000,
                success=False,
                error=str(e)
            )
    
    async def _execute_full_workflow_test(self) -> TestResult:
        """Execute full PQC workflow test"""
        start_time = time.time()
        
        try:
            # Step 1: Key Generation
            key_result = await self._execute_key_generation_test()
            if not key_result.success:
                raise Exception(f"Key generation failed: {key_result.error}")
            
            # Step 2: TLS Handshake
            handshake_result = await self._execute_gateway_tls_test()
            if not handshake_result.success:
                raise Exception(f"Handshake failed: {handshake_result.error}")
            
            # Step 3: Send Message
            message_result = await self._execute_messaging_test()
            if not message_result.success:
                raise Exception(f"Messaging failed: {message_result.error}")
            
            end_time = time.time()
            total_duration = (end_time - start_time) * 1000
            
            return TestResult(
                test_type='full_workflow',
                start_time=start_time,
                end_time=end_time,
                duration_ms=total_duration,
                success=True,
                metadata={
                    'key_duration_ms': key_result.duration_ms,
                    'handshake_duration_ms': handshake_result.duration_ms,
                    'message_duration_ms': message_result.duration_ms
                }
            )
        except Exception as e:
            end_time = time.time()
            return TestResult(
                test_type='full_workflow',
                start_time=start_time,
                end_time=end_time,
                duration_ms=(end_time - start_time) * 1000,
                success=False,
                error=str(e)
            )
    
    async def _execute_test(self) -> TestResult:
        """Route to appropriate test based on configuration"""
        test_handlers = {
            TestType.GATEWAY_TLS_HANDSHAKE: self._execute_gateway_tls_test,
            TestType.VPN_TUNNEL_ESTABLISHMENT: self._execute_vpn_tunnel_test,
            TestType.MESSAGING_ENCRYPTION: self._execute_messaging_test,
            TestType.KEY_GENERATION: self._execute_key_generation_test,
            TestType.SIGNATURE_VERIFICATION: self._execute_signature_test,
            TestType.FULL_WORKFLOW: self._execute_full_workflow_test,
        }
        
        handler = test_handlers.get(self.config.test_type)
        if not handler:
            raise ValueError(f"Unknown test type: {self.config.test_type}")
        
        return await handler()
    
    async def _worker(self, worker_id: int, stop_event: asyncio.Event):
        """Worker coroutine that executes tests until stop signal"""
        while not stop_event.is_set():
            try:
                result = await self._execute_test()
                self.results.append(result)
                
                # Add small delay between requests
                await asyncio.sleep(0.01)
            except Exception as e:
                logger.error(f"Worker {worker_id} error: {e}")
    
    async def run(self) -> LoadTestResults:
        """Run the load test"""
        logger.info(f"Starting load test: {self.config.test_type.value}")
        logger.info(f"Configuration: {self.config.concurrent_users} users, {self.config.duration_seconds}s duration")
        
        self._session = await self._create_session()
        start_time = time.time()
        stop_event = asyncio.Event()
        
        # Create worker tasks with ramp-up
        workers = []
        for i in range(self.config.concurrent_users):
            worker = asyncio.create_task(self._worker(i, stop_event))
            workers.append(worker)
            
            # Ramp-up delay
            if i < self.config.concurrent_users - 1:
                await asyncio.sleep(
                    self.config.ramp_up_seconds / self.config.concurrent_users
                )
        
        logger.info(f"All {self.config.concurrent_users} workers started")
        
        # Wait for test duration
        await asyncio.sleep(self.config.duration_seconds)
        
        # Signal workers to stop
        stop_event.set()
        
        # Wait for workers to complete
        await asyncio.gather(*workers, return_exceptions=True)
        
        end_time = time.time()
        actual_duration = end_time - start_time
        
        await self._session.close()
        
        # Calculate results
        return self._calculate_results(actual_duration)
    
    def _calculate_results(self, duration: float) -> LoadTestResults:
        """Calculate aggregated results from individual test results"""
        if not self.results:
            return LoadTestResults(
                test_type=self.config.test_type.value,
                total_requests=0,
                successful_requests=0,
                failed_requests=0,
                avg_latency_ms=0,
                p50_latency_ms=0,
                p95_latency_ms=0,
                p99_latency_ms=0,
                min_latency_ms=0,
                max_latency_ms=0,
                requests_per_second=0,
                error_rate=1.0,
                duration_seconds=duration
            )
        
        latencies = [r.duration_ms for r in self.results]
        successful = [r for r in self.results if r.success]
        failed = [r for r in self.results if not r.success]
        
        # Sort latencies for percentile calculation
        sorted_latencies = sorted(latencies)
        
        def percentile(data: List[float], p: float) -> float:
            if not data:
                return 0
            k = (len(data) - 1) * p / 100
            f = int(k)
            c = f + 1 if f + 1 < len(data) else f
            return data[f] + (k - f) * (data[c] - data[f]) if c != f else data[f]
        
        return LoadTestResults(
            test_type=self.config.test_type.value,
            total_requests=len(self.results),
            successful_requests=len(successful),
            failed_requests=len(failed),
            avg_latency_ms=statistics.mean(latencies),
            p50_latency_ms=percentile(sorted_latencies, 50),
            p95_latency_ms=percentile(sorted_latencies, 95),
            p99_latency_ms=percentile(sorted_latencies, 99),
            min_latency_ms=min(latencies),
            max_latency_ms=max(latencies),
            requests_per_second=len(self.results) / duration,
            error_rate=len(failed) / len(self.results) if self.results else 0,
            duration_seconds=duration
        )


def print_results(results: LoadTestResults):
    """Print formatted test results"""
    print("\n" + "=" * 60)
    print(f"  PQC Load Test Results - {results.test_type}")
    print("=" * 60)
    print(f"\n📊 Summary:")
    print(f"   Total Requests:     {results.total_requests:,}")
    print(f"   Successful:         {results.successful_requests:,}")
    print(f"   Failed:             {results.failed_requests:,}")
    print(f"   Error Rate:         {results.error_rate * 100:.2f}%")
    print(f"   Requests/sec:       {results.requests_per_second:.2f}")
    
    print(f"\n⏱️  Latency:")
    print(f"   Average:            {results.avg_latency_ms:.2f} ms")
    print(f"   P50:                {results.p50_latency_ms:.2f} ms")
    print(f"   P95:                {results.p95_latency_ms:.2f} ms")
    print(f"   P99:                {results.p99_latency_ms:.2f} ms")
    print(f"   Min:                {results.min_latency_ms:.2f} ms")
    print(f"   Max:                {results.max_latency_ms:.2f} ms")
    
    print(f"\n⏰ Test Duration:      {results.duration_seconds:.2f} seconds")
    
    # Performance thresholds
    print(f"\n✅ Threshold Validation:")
    thresholds = {
        'p95_latency': (results.p95_latency_ms, 100, 'ms'),
        'error_rate': (results.error_rate * 100, 1, '%'),
    }
    
    for name, (value, threshold, unit) in thresholds.items():
        status = "✅ PASS" if value <= threshold else "❌ FAIL"
        print(f"   {name}: {value:.2f} {unit} (threshold: {threshold} {unit}) {status}")
    
    print("=" * 60 + "\n")


def save_results(results: LoadTestResults, output_path: str):
    """Save results to JSON file"""
    with open(output_path, 'w') as f:
        json.dump(results.to_dict(), f, indent=2)
    logger.info(f"Results saved to {output_path}")


async def main():
    parser = argparse.ArgumentParser(description='V-Sentinel PQC Load Testing')
    parser.add_argument('--config', required=True, help='Path to config file')
    parser.add_argument('--output', default='load_test_results.json', help='Output file path')
    parser.add_argument('--verbose', action='store_true', help='Enable verbose logging')
    
    args = parser.parse_args()
    
    if args.verbose:
        logging.getLogger().setLevel(logging.DEBUG)
    
    config = LoadTestConfig.from_yaml(args.config)
    tester = PqcLoadTester(config)
    
    results = await tester.run()
    
    print_results(results)
    save_results(results, args.output)
    
    # Exit with error code if error rate too high
    if results.error_rate > 0.01:
        sys.exit(1)
    
    sys.exit(0)


if __name__ == '__main__':
    asyncio.run(main())
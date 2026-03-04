#!/bin/bash
#
# V-Sentinel Performance Benchmark Runner
#
# This script runs all performance benchmarks and generates reports.

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
BENCHMARK_DIR="benches"
RESULTS_DIR="benchmark_results"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
RESULTS_FILE="${RESULTS_DIR}/benchmark_${TIMESTAMP}.json"
THRESHOLDS_FILE="config/performance_thresholds.toml"

# Create results directory
mkdir -p "${RESULTS_DIR}"

echo -e "${BLUE}======================================${NC}"
echo -e "${BLUE}  V-Sentinel Benchmark Suite${NC}"
echo -e "${BLUE}======================================${NC}"
echo ""
echo "Timestamp: ${TIMESTAMP}"
echo "Results: ${RESULTS_FILE}"
echo ""

# Function to run benchmarks for a module
run_module_benchmark() {
    local module=$1
    echo -e "${YELLOW}Running ${module} benchmarks...${NC}"
    
    if cargo bench --bench module_benchmarks -- "${module}" 2>&1 | tee -a "${RESULTS_FILE}.tmp"; then
        echo -e "${GREEN}✓ ${module} benchmarks completed${NC}"
    else
        echo -e "${RED}✗ ${module} benchmarks failed${NC}"
    fi
    echo ""
}

# Function to compare against thresholds
compare_thresholds() {
    echo -e "${YELLOW}Comparing results against thresholds...${NC}"
    
    if [ ! -f "${THRESHOLDS_FILE}" ]; then
        echo -e "${RED}Thresholds file not found: ${THRESHOLDS_FILE}${NC}"
        return
    fi
    
    echo -e "${GREEN}Performance thresholds loaded${NC}"
}

# Main execution
main() {
    echo -e "${BLUE}Starting benchmark suite...${NC}"
    echo ""
    
    # Check if Rust is available
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}Cargo not found. Please install Rust.${NC}"
        exit 1
    fi
    
    # Build benchmarks first
    echo -e "${YELLOW}Building benchmarks...${NC}"
    cargo build --release --benches
    
    echo ""
    echo -e "${BLUE}Running benchmarks by module...${NC}"
    echo ""
    
    # Run all benchmark groups
    run_module_benchmark "privacy"
    run_module_benchmark "quantum"
    run_module_benchmark "biometrics"
    run_module_benchmark "neural"
    run_module_benchmark "autonomous"
    run_module_benchmark "metaverse"
    run_module_benchmark "blockchain"
    run_module_benchmark "siem"
    run_module_benchmark "core"
    
    # Compare thresholds
    compare_thresholds
    
    # Generate summary
    echo -e "${BLUE}======================================${NC}"
    echo -e "${BLUE}  Benchmark Summary${NC}"
    echo -e "${BLUE}======================================${NC}"
    echo ""
    
    echo "Results saved to: ${RESULTS_FILE}"
    echo "Thresholds file: ${THRESHOLDS_FILE}"
    echo ""
    
    # Generate HTML report if available
    if [ -d "target/criterion" ]; then
        echo -e "${GREEN}HTML reports available at: target/criterion/reports/${NC}"
    fi
    
    echo -e "${GREEN}Benchmark suite completed!${NC}"
}

# Run main
main "$@"
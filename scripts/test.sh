#!/bin/bash

# SENTINEL Test Script
# This script runs all tests for the SENTINEL project

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
RUSTFLAGS="-C instrument-coverage"
CARGO_INCREMENTAL=0
COVERAGE_DIR=target/coverage

# Functions
log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

check_prerequisites() {
    log_info "Checking prerequisites..."
    
    if ! command -v cargo &> /dev/null; then
        log_error "Cargo is not installed"
        exit 1
    fi
    
    if ! command -v cargo-tarpaulin &> /dev/null; then
        log_warn "cargo-tarpaulin not found, installing..."
        cargo install cargo-tarpaulin
    fi
    
    log_info "Prerequisites check passed"
}

clean() {
    log_info "Cleaning previous build artifacts..."
    cargo clean
}

build() {
    log_info "Building SENTINEL in debug mode..."
    cargo build
}

build_release() {
    log_info "Building SENTINEL in release mode..."
    cargo build --release
}

run_unit_tests() {
    log_info "Running unit tests..."
    cargo test --lib --verbose
    log_info "✅ Unit tests passed"
}

run_integration_tests() {
    log_info "Running integration tests..."
    cargo test --test integration_tests --verbose
    log_info "✅ Integration tests passed"
}

run_e2e_tests() {
    log_info "Running end-to-end tests..."
    cargo test --test e2e_tests --verbose
    log_info "✅ End-to-end tests passed"
}

run_all_tests() {
    log_info "Running all tests..."
    cargo test --verbose
    log_info "✅ All tests passed"
}

run_tests_with_coverage() {
    log_info "Running tests with coverage..."
    
    mkdir -p $COVERAGE_DIR
    
    cargo tarpaulin \
        --verbose \
        --out Html \
        --output-dir $COVERAGE_DIR \
        --output-name coverage \
        --exclude-files '*/tests/*' \
        --exclude-files '*/examples/*'
    
    log_info "✅ Coverage report generated in $COVERAGE_DIR/"
    log_info "Open coverage report: file://$PWD/$COVERAGE_DIR/coverage/index.html"
}

run_clippy() {
    log_info "Running clippy..."
    cargo clippy -- -D warnings
    log_info "✅ Clippy checks passed"
}

run_fmt_check() {
    log_info "Running format check..."
    cargo fmt -- --check
    log_info "✅ Format check passed"
}

run_audit() {
    log_info "Running security audit..."
    cargo audit
    log_info "✅ Security audit passed"
}

run_benchmarks() {
    log_info "Running benchmarks..."
    cargo bench --verbose
    log_info "✅ Benchmarks completed"
}

# Main test flow
main() {
    local test_type="${1:-all}"
    
    log_info "Starting SENTINEL test suite..."
    
    check_prerequisites
    
    case $test_type in
        unit)
            run_unit_tests
            ;;
        integration)
            run_integration_tests
            ;;
        e2e)
            run_e2e_tests
            ;;
        all)
            run_all_tests
            ;;
        coverage)
            run_tests_with_coverage
            ;;
        clippy)
            run_clippy
            ;;
        fmt)
            run_fmt_check
            ;;
        audit)
            run_audit
            ;;
        benchmarks)
            run_benchmarks
            ;;
        ci)
            log_info "Running CI test suite..."
            run_fmt_check
            run_clippy
            run_all_tests
            run_audit
            log_info "✅ CI test suite completed"
            ;;
        *)
            log_error "Unknown test type: $test_type"
            echo "Usage: $0 {unit|integration|e2e|all|coverage|clippy|fmt|audit|benchmarks|ci}"
            exit 1
            ;;
    esac
    
    log_info "✅ Test suite completed successfully!"
}

# Run main function with arguments
main "$@"
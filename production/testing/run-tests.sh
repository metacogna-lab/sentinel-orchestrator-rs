#!/bin/bash
# Run production integration tests
# This script runs the Rust integration tests against a running production stack

set -euo pipefail

# Configuration
API_URL="${SENTINEL_API_URL:-http://localhost:3000}"
API_KEY="${SENTINEL_API_KEY:-sk-test-key}"
RUST_LOG="${RUST_LOG:-info}"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if API is accessible
check_api() {
    log_info "Checking if API is accessible at ${API_URL}..."
    if curl -f -s "${API_URL}/health" > /dev/null; then
        log_info "API is accessible"
        return 0
    else
        log_error "API is not accessible. Please start the production stack first."
        log_info "Run: cd production/docker && docker-compose -f docker-compose.prod.yaml up -d"
        return 1
    fi
}

# Run integration tests
run_tests() {
    log_info "Running integration tests..."
    
    # Set environment variables for tests
    export SENTINEL_API_URL="${API_URL}"
    export SENTINEL_API_KEY="${API_KEY}"
    export RUST_LOG="${RUST_LOG}"
    
    # Run LLM provider tests
    log_info "Running LLM provider simulation tests..."
    cargo test --test integration_llm_providers -- --ignored --nocapture || {
        log_error "LLM provider tests failed"
        return 1
    }
    
    # Run vector storage tests
    log_info "Running vector storage tests..."
    cargo test --test integration_vector_storage -- --ignored --nocapture || {
        log_error "Vector storage tests failed"
        return 1
    }
    
    log_info "All integration tests passed"
}

# Main execution
main() {
    if ! check_api; then
        exit 1
    fi
    
    run_tests
}

main "$@"


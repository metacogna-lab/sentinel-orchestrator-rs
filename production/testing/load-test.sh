#!/bin/bash
# Production load test script
# Simulates various LLM providers hitting our endpoints and tests vector storage

set -euo pipefail

# Configuration
API_URL="${SENTINEL_API_URL:-http://localhost:3000}"
API_KEY="${SENTINEL_API_KEY:-sk-test-key}"
CONCURRENT_REQUESTS="${CONCURRENT_REQUESTS:-10}"
TOTAL_REQUESTS="${TOTAL_REQUESTS:-100}"
TEST_DURATION="${TEST_DURATION:-60}"

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
check_health() {
    log_info "Checking API health..."
    if curl -f -s "${API_URL}/health" > /dev/null; then
        log_info "API is healthy"
        return 0
    else
        log_error "API is not accessible at ${API_URL}"
        return 1
    fi
}

# Make a chat completion request
make_request() {
    local message_id=$1
    local content=$2
    
    curl -s -w "\n%{http_code}" \
        -X POST "${API_URL}/v1/chat/completions" \
        -H "Authorization: Bearer ${API_KEY}" \
        -H "Content-Type: application/json" \
        -d "{
            \"messages\": [{
                \"id\": \"${message_id}\",
                \"role\": \"user\",
                \"content\": \"${content}\",
                \"timestamp\": \"$(date -u +%Y-%m-%dT%H:%M:%SZ)\",
                \"metadata\": {}
            }]
        }" | tail -n 1
}

# Run load test
run_load_test() {
    log_info "Starting load test..."
    log_info "API URL: ${API_URL}"
    log_info "Concurrent requests: ${CONCURRENT_REQUESTS}"
    log_info "Total requests: ${TOTAL_REQUESTS}"
    
    local success_count=0
    local error_count=0
    local rate_limit_count=0
    
    # Create temporary file for results
    local results_file=$(mktemp)
    
    # Run requests in parallel
    for i in $(seq 1 ${TOTAL_REQUESTS}); do
        local message_id="test-$(uuidgen | tr -d '-' | cut -c1-8)"
        local content="Load test message ${i}: $(date +%s)"
        
        # Run in background
        (
            local status=$(make_request "${message_id}" "${content}")
            echo "${status}" >> "${results_file}"
        ) &
        
        # Limit concurrent requests
        if (( i % CONCURRENT_REQUESTS == 0 )); then
            wait
        fi
    done
    
    wait
    
    # Count results
    while IFS= read -r status; do
        case "${status}" in
            200) ((success_count++)) ;;
            429) ((rate_limit_count++)) ;;
            *) ((error_count++)) ;;
        esac
    done < "${results_file}"
    
    rm "${results_file}"
    
    log_info "Load test completed:"
    log_info "  Success: ${success_count}"
    log_info "  Rate limited: ${rate_limit_count}"
    log_info "  Errors: ${error_count}"
    
    # Calculate success rate
    local total=$((success_count + error_count + rate_limit_count))
    if [ ${total} -gt 0 ]; then
        local success_rate=$(echo "scale=2; ${success_count} * 100 / ${total}" | bc)
        log_info "  Success rate: ${success_rate}%"
    fi
}

# Test vector storage operations
test_vector_storage() {
    log_info "Testing vector storage operations..."
    
    # Store information
    local store_message="Store this fact: The Eiffel Tower is located in Paris, France."
    local message_id="vector-test-$(uuidgen | tr -d '-' | cut -c1-8)"
    
    log_info "Storing message in vector database..."
    local status=$(make_request "${message_id}" "${store_message}")
    
    if [ "${status}" = "200" ]; then
        log_info "Message stored successfully"
        sleep 1
        
        # Query stored information
        log_info "Querying stored information..."
        local query_message="Where is the Eiffel Tower located?"
        local query_id="vector-query-$(uuidgen | tr -d '-' | cut -c1-8)"
        local query_status=$(make_request "${query_id}" "${query_message}")
        
        if [ "${query_status}" = "200" ]; then
            log_info "Vector storage query successful"
        else
            log_warn "Vector storage query returned status: ${query_status}"
        fi
    else
        log_warn "Vector storage returned status: ${status}"
    fi
}

# Test concurrent provider simulation
test_concurrent_providers() {
    log_info "Testing concurrent provider simulation..."
    
    local providers=("openai" "anthropic" "google" "cohere" "mistral")
    local success_count=0
    
    for provider in "${providers[@]}"; do
        local message_id="provider-${provider}-$(uuidgen | tr -d '-' | cut -c1-8)"
        local content="Test message from ${provider} provider"
        
        (
            local status=$(make_request "${message_id}" "${content}")
            if [ "${status}" = "200" ]; then
                echo "SUCCESS:${provider}" >> /tmp/provider_results
            else
                echo "ERROR:${provider}:${status}" >> /tmp/provider_results
            fi
        ) &
    done
    
    wait
    
    if [ -f /tmp/provider_results ]; then
        while IFS= read -r result; do
            if [[ "${result}" == SUCCESS:* ]]; then
                ((success_count++))
                log_info "Provider ${result#SUCCESS:} succeeded"
            else
                log_warn "Provider error: ${result#ERROR:}"
            fi
        done < /tmp/provider_results
        rm /tmp/provider_results
    fi
    
    log_info "Concurrent provider test: ${success_count}/${#providers[@]} succeeded"
}

# Main execution
main() {
    log_info "Starting production load tests..."
    
    if ! check_health; then
        exit 1
    fi
    
    # Run tests
    test_vector_storage
    test_concurrent_providers
    run_load_test
    
    log_info "All tests completed"
}

# Run main function
main "$@"


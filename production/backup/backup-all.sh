#!/bin/bash
# Sentinel Orchestrator - Complete Backup Script
# This script backs up all Sentinel data (Sled, Qdrant, PostgreSQL)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
LOG_FILE="/var/log/sentinel-backup-${TIMESTAMP}.log"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1" | tee -a "${LOG_FILE}"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1" | tee -a "${LOG_FILE}"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1" | tee -a "${LOG_FILE}"
}

log_info "Starting complete backup at $(date)"

# Backup Sled
log_info "Backing up Sled database..."
if bash "${SCRIPT_DIR}/backup-sled.sh"; then
    log_info "Sled backup completed"
else
    log_error "Sled backup failed"
    exit 1
fi

# Backup Qdrant
log_info "Backing up Qdrant database..."
if bash "${SCRIPT_DIR}/backup-qdrant.sh"; then
    log_info "Qdrant backup completed"
else
    log_error "Qdrant backup failed"
    exit 1
fi

# Backup PostgreSQL
log_info "Backing up PostgreSQL database..."
if bash "${SCRIPT_DIR}/backup-postgres.sh"; then
    log_info "PostgreSQL backup completed"
else
    log_error "PostgreSQL backup failed"
    exit 1
fi

log_info "Complete backup finished successfully at $(date)"


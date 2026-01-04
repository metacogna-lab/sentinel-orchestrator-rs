#!/bin/bash
# Sentinel Orchestrator - Sled Database Backup Script
# This script creates a backup of the Sled database

set -euo pipefail

# Configuration
BACKUP_DIR="${BACKUP_DIR:-/app/backups/sled}"
RETENTION_DAYS="${BACKUP_RETENTION_DAYS:-30}"
SLED_PATH="${SLED_PATH:-/app/data/sled}"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_FILE="${BACKUP_DIR}/sled_backup_${TIMESTAMP}.tar.gz"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Create backup directory if it doesn't exist
mkdir -p "${BACKUP_DIR}"

# Check if Sled database exists
if [ ! -d "${SLED_PATH}" ]; then
    log_error "Sled database not found at ${SLED_PATH}"
    exit 1
fi

log_info "Starting Sled backup..."
log_info "Source: ${SLED_PATH}"
log_info "Destination: ${BACKUP_FILE}"

# Create backup
if tar -czf "${BACKUP_FILE}" -C "$(dirname "${SLED_PATH}")" "$(basename "${SLED_PATH}")"; then
    BACKUP_SIZE=$(du -h "${BACKUP_FILE}" | cut -f1)
    log_info "Backup created successfully: ${BACKUP_FILE} (${BACKUP_SIZE})"
else
    log_error "Failed to create backup"
    exit 1
fi

# Verify backup
if [ -f "${BACKUP_FILE}" ] && [ -s "${BACKUP_FILE}" ]; then
    log_info "Backup verification: OK"
else
    log_error "Backup verification failed: file is missing or empty"
    exit 1
fi

# Clean up old backups
log_info "Cleaning up backups older than ${RETENTION_DAYS} days..."
find "${BACKUP_DIR}" -name "sled_backup_*.tar.gz" -type f -mtime +${RETENTION_DAYS} -delete
REMAINING=$(find "${BACKUP_DIR}" -name "sled_backup_*.tar.gz" -type f | wc -l)
log_info "Remaining backups: ${REMAINING}"

# Optional: Upload to S3
if [ -n "${BACKUP_S3_BUCKET:-}" ]; then
    log_info "Uploading backup to S3..."
    if command -v aws &> /dev/null; then
        aws s3 cp "${BACKUP_FILE}" "s3://${BACKUP_S3_BUCKET}/sled/${TIMESTAMP}.tar.gz" || {
            log_warn "Failed to upload to S3, but local backup exists"
        }
    else
        log_warn "AWS CLI not found, skipping S3 upload"
    fi
fi

log_info "Backup completed successfully"


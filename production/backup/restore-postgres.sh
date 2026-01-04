#!/bin/bash
# Sentinel Orchestrator - PostgreSQL Restore Script
# This script restores a PostgreSQL database from backup

set -euo pipefail

# Configuration
BACKUP_DIR="${BACKUP_DIR:-/app/backups/postgres}"
POSTGRES_HOST="${POSTGRES_HOST:-postgres}"
POSTGRES_PORT="${POSTGRES_PORT:-5432}"
POSTGRES_USER="${POSTGRES_USER:-postgres}"
POSTGRES_DB="${POSTGRES_DB:-sentinel}"

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

# Check if backup file is provided
if [ $# -eq 0 ]; then
    log_error "Usage: $0 <backup_file>"
    log_info "Available backups:"
    ls -lh "${BACKUP_DIR}"/*.sql.gz 2>/dev/null || log_warn "No backups found"
    exit 1
fi

BACKUP_FILE="$1"

# Check if backup file exists
if [ ! -f "${BACKUP_FILE}" ]; then
    log_error "Backup file not found: ${BACKUP_FILE}"
    exit 1
fi

log_warn "WARNING: This will restore the database from backup."
log_warn "This will REPLACE all current data in ${POSTGRES_DB} database."
read -p "Are you sure you want to continue? (yes/no): " CONFIRM

if [ "${CONFIRM}" != "yes" ]; then
    log_info "Restore cancelled"
    exit 0
fi

log_info "Starting PostgreSQL restore..."
log_info "Backup file: ${BACKUP_FILE}"
log_info "Database: ${POSTGRES_DB}"
log_info "Host: ${POSTGRES_HOST}:${POSTGRES_PORT}"

# Check if PostgreSQL is accessible
if ! PGPASSWORD="${POSTGRES_PASSWORD}" pg_isready -h "${POSTGRES_HOST}" -p "${POSTGRES_PORT}" -U "${POSTGRES_USER}" > /dev/null 2>&1; then
    log_error "PostgreSQL is not accessible at ${POSTGRES_HOST}:${POSTGRES_PORT}"
    exit 1
fi

# Drop and recreate database (optional - comment out if you want to restore to existing)
log_warn "Dropping existing database..."
PGPASSWORD="${POSTGRES_PASSWORD}" psql -h "${POSTGRES_HOST}" -p "${POSTGRES_PORT}" -U "${POSTGRES_USER}" -d postgres -c "DROP DATABASE IF EXISTS ${POSTGRES_DB};"
PGPASSWORD="${POSTGRES_PASSWORD}" psql -h "${POSTGRES_HOST}" -p "${POSTGRES_PORT}" -U "${POSTGRES_USER}" -d postgres -c "CREATE DATABASE ${POSTGRES_DB};"

# Restore from backup
log_info "Restoring database..."
if gunzip -c "${BACKUP_FILE}" | PGPASSWORD="${POSTGRES_PASSWORD}" psql -h "${POSTGRES_HOST}" -p "${POSTGRES_PORT}" -U "${POSTGRES_USER}" -d "${POSTGRES_DB}"; then
    log_info "Restore completed successfully"
else
    log_error "Restore failed"
    exit 1
fi

# Verify restore
log_info "Verifying restore..."
TABLE_COUNT=$(PGPASSWORD="${POSTGRES_PASSWORD}" psql -h "${POSTGRES_HOST}" -p "${POSTGRES_PORT}" -U "${POSTGRES_USER}" -d "${POSTGRES_DB}" -t -c "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = 'public';")
log_info "Tables restored: ${TABLE_COUNT}"

log_info "Restore completed successfully"


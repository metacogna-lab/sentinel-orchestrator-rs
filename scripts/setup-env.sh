#!/bin/bash

# Sentinel Orchestrator - Environment Setup Script
# This script copies .env.example to .env if .env doesn't exist
# Usage: ./scripts/setup-env.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
ENV_EXAMPLE="$PROJECT_ROOT/.env.example"
ENV_FILE="$PROJECT_ROOT/.env"

# Check if .env.example exists
if [ ! -f "$ENV_EXAMPLE" ]; then
    echo "Error: .env.example not found at $ENV_EXAMPLE"
    exit 1
fi

# Check if .env already exists
if [ -f "$ENV_FILE" ]; then
    echo "Warning: .env file already exists at $ENV_FILE"
    echo "Skipping copy. Delete .env if you want to regenerate from .env.example"
    exit 0
fi

# Copy .env.example to .env
cp "$ENV_EXAMPLE" "$ENV_FILE"

echo "✓ Created .env file from .env.example"
echo "✓ Location: $ENV_FILE"
echo ""
echo "Next steps:"
echo "  1. Review and update .env with your actual configuration values"
echo "  2. Set OPENAI_API_KEY and other sensitive values"
echo "  3. Update database passwords for production use"
echo ""
echo "Note: .env is in .gitignore and will not be committed to version control"


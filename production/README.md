# Sentinel Orchestrator - Production Artifacts

This directory contains all production-ready artifacts for deploying Sentinel Orchestrator.

## Directory Structure

```
production/
├── docker/              # Docker Compose and Dockerfile
│   ├── docker-compose.prod.yaml
│   └── Dockerfile.prod
├── monitoring/          # Monitoring configuration
│   ├── prometheus.yml
│   ├── prometheus-rules.yml
│   └── grafana/
│       └── dashboards/
├── logging/            # Logging configuration
│   ├── loki-config.yml
│   └── promtail-config.yml
├── nginx/              # Reverse proxy configuration
│   ├── nginx.conf
│   └── tls/            # TLS certificates (not in repo)
├── config/             # Configuration templates
│   └── env.production.template
├── backup/             # Backup scripts
│   ├── backup-all.sh
│   ├── backup-sled.sh
│   ├── backup-qdrant.sh
│   └── backup-postgres.sh
├── runbooks/           # Operational runbooks
│   ├── deployment.md
│   └── incident-response.md
├── security/           # Security documentation
│   └── security-hardening.md
├── docs/               # Documentation
│   └── production-deployment.md
└── testing/            # Testing scripts (future)
```

## Quick Start

1. **Configure Environment**
   ```bash
   cp config/env.production.template config/.env.production
   # Edit .env.production with actual values
   ```

2. **Setup TLS Certificates**
   ```bash
   # Place certificates in nginx/tls/
   cp /path/to/cert.pem nginx/tls/
   cp /path/to/key.pem nginx/tls/
   ```

3. **Deploy**
   ```bash
   cd docker
   docker-compose -f docker-compose.prod.yaml --env-file ../config/.env.production up -d
   ```

4. **Verify**
   ```bash
   curl http://localhost/health
   ```

## Documentation

- **[Production Deployment Guide](docs/production-deployment.md)** - Complete deployment instructions
- **[Deployment Runbook](runbooks/deployment.md)** - Step-by-step deployment procedures
- **[Incident Response Runbook](runbooks/incident-response.md)** - Incident handling procedures
- **[Security Hardening Guide](security/security-hardening.md)** - Security best practices

## Services

The production stack includes:

- **Backend**: Sentinel Orchestrator API
- **PostgreSQL**: Relational database
- **Qdrant**: Vector database
- **Nginx**: Reverse proxy with TLS
- **Prometheus**: Metrics collection
- **Grafana**: Metrics visualization
- **Loki**: Log aggregation
- **Promtail**: Log collection

## Monitoring

- **Grafana**: http://localhost:3001
- **Prometheus**: http://localhost:9090 (internal)
- **Loki**: http://localhost:3100 (internal)

## Backup

Automated backup scripts are available in `backup/`:
- `backup-all.sh` - Backup all components
- `backup-sled.sh` - Backup Sled database
- `backup-qdrant.sh` - Backup Qdrant database
- `backup-postgres.sh` - Backup PostgreSQL

## Security

- See [Security Hardening Guide](security/security-hardening.md)
- Never commit `.env.production` to version control
- Use strong passwords
- Rotate API keys regularly
- Keep images updated

## Support

For issues:
1. Check runbooks in `runbooks/`
2. Review logs in Grafana
3. Check Prometheus metrics
4. Contact on-call engineer


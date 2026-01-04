# Security Hardening Guide

Security best practices for Sentinel Orchestrator production deployment.

## Container Security

### Non-Root User

The production Dockerfile runs as non-root user (UID 1000):

```dockerfile
USER sentinel
```

### Minimal Base Image

Uses `debian:bookworm-slim` for minimal attack surface.

### Read-Only Filesystem (where possible)

Consider mounting data directories as volumes, not in root filesystem.

## Network Security

### TLS Configuration

- TLS 1.3 only
- Strong cipher suites
- HSTS headers enabled
- Certificate validation

### Firewall Rules

Restrict access to:
- **Port 80/443**: Public API access
- **Port 3001**: Grafana (restrict to internal network)
- **Port 9090**: Prometheus (internal only)
- **Port 3100**: Loki (internal only)

### Rate Limiting

Nginx configured with rate limiting:
- API endpoints: 10 requests/second
- Auth endpoints: 5 requests/second

## Secrets Management

### Environment Variables

- Never commit `.env.production` to version control
- Use strong passwords (minimum 32 characters)
- Rotate secrets regularly
- Use different secrets for each environment

### API Keys

- Store in environment variables
- Use `secrecy` crate in application
- Never log API keys
- Rotate regularly

### Database Credentials

- Use strong passwords
- Limit database user permissions
- Use separate users for different services
- Enable SSL/TLS for database connections

## Application Security

### Input Validation

- All API inputs validated
- Message content sanitized
- Timestamp validation
- UUID validation

### Authentication

- API key authentication required
- Bearer token format
- Permission levels (read/write/admin)
- Rate limiting per API key

### Error Handling

- Don't expose internal errors to clients
- Log errors with context
- Sanitize error messages

## Monitoring Security

### Access Control

- Grafana: Require authentication
- Prometheus: Internal network only
- Loki: Internal network only

### Log Security

- Don't log sensitive data (API keys, passwords)
- Sanitize logs before storage
- Set log retention policies
- Encrypt log storage

## Backup Security

### Backup Encryption

- Encrypt backups at rest
- Use secure transfer (TLS) for backup uploads
- Secure backup storage (S3 with encryption)

### Backup Access

- Limit access to backup files
- Use separate credentials for backup operations
- Audit backup access

## Compliance

### Data Protection

- Follow GDPR if handling EU data
- Implement data retention policies
- Provide data deletion capabilities

### Audit Logging

- Log all authentication attempts
- Log all API requests
- Log all administrative actions
- Retain audit logs per compliance requirements

## Security Checklist

- [ ] Non-root user in containers
- [ ] TLS certificates configured
- [ ] Strong passwords for all services
- [ ] API keys rotated regularly
- [ ] Firewall rules configured
- [ ] Rate limiting enabled
- [ ] Monitoring access restricted
- [ ] Backups encrypted
- [ ] Secrets not in version control
- [ ] Input validation implemented
- [ ] Error messages sanitized
- [ ] Logs don't contain sensitive data
- [ ] Regular security updates
- [ ] Security scanning in CI/CD

## Security Updates

### Regular Updates

- Update Docker base images monthly
- Update application dependencies (cargo audit)
- Update system packages
- Monitor security advisories

### Vulnerability Scanning

- Scan Docker images in CI/CD
- Use Trivy or similar tools
- Fix high/critical vulnerabilities immediately
- Review medium/low vulnerabilities

## Incident Response

See [Incident Response Runbook](../runbooks/incident-response.md) for security incident procedures.

### Security Incidents

If security breach detected:
1. Isolate affected services
2. Preserve logs and evidence
3. Notify security team immediately
4. Follow incident response procedures
5. Document and remediate


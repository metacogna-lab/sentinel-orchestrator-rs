# Production Deployment Guide

Complete guide for deploying Sentinel Orchestrator to production.

## Prerequisites

- Docker and Docker Compose installed
- At least 8GB RAM available
- At least 50GB disk space
- Network access for external services (OpenAI API)
- TLS certificates (for HTTPS)

## Initial Setup

### 1. Clone Repository

```bash
git clone <repository-url>
cd sentinel-orchestrator
```

### 2. Configure Environment

```bash
cd production/config
cp env.production.template .env.production
nano .env.production  # Edit with actual values
```

**Required Configuration:**
- `POSTGRES_PASSWORD`: Strong password for PostgreSQL
- `OPENAI_API_KEY`: Your OpenAI API key
- `GRAFANA_ADMIN_PASSWORD`: Password for Grafana admin
- All other values can use defaults or be customized

### 3. Setup TLS Certificates

```bash
# Create TLS directory
mkdir -p production/nginx/tls

# Option 1: Use Let's Encrypt (recommended for production)
# Install certbot and obtain certificates
certbot certonly --standalone -d your-domain.com

# Copy certificates
cp /etc/letsencrypt/live/your-domain.com/fullchain.pem production/nginx/tls/cert.pem
cp /etc/letsencrypt/live/your-domain.com/privkey.pem production/nginx/tls/key.pem

# Option 2: Self-signed (development only)
openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
  -keyout production/nginx/tls/key.pem \
  -out production/nginx/tls/cert.pem
```

### 4. Deploy Services

```bash
cd production/docker
docker-compose -f docker-compose.prod.yaml --env-file ../config/.env.production up -d
```

### 5. Verify Deployment

```bash
# Check all services are running
docker-compose -f docker-compose.prod.yaml --env-file ../config/.env.production ps

# Check health endpoints
curl http://localhost/health
curl http://localhost/health/ready
curl http://localhost/health/live

# Check metrics
curl http://localhost:9090/metrics

# Access Grafana
open http://localhost:3001
```

## Service Access

- **API**: `https://your-domain.com` (via Nginx)
- **Grafana**: `http://localhost:3001` (or configure domain)
- **Prometheus**: `http://localhost:9090` (internal only)
- **Loki**: `http://localhost:3100` (internal only)

## Monitoring Setup

### 1. Import Grafana Dashboards

1. Access Grafana at `http://localhost:3001`
2. Login with admin credentials
3. Go to Dashboards → Import
4. Import dashboards from `production/monitoring/grafana/dashboards/`

### 2. Configure Prometheus Alerts

Prometheus alerts are configured in `production/monitoring/prometheus-rules.yml` and loaded automatically.

### 3. Setup Log Aggregation

Loki and Promtail are configured automatically. View logs in Grafana:
1. Go to Explore
2. Select Loki data source
3. Query logs: `{service="sentinel-backend-prod"}`

## Backup Configuration

### Automated Backups

Setup cron job for automated backups:

```bash
# Edit crontab
crontab -e

# Add backup job (daily at 2 AM)
0 2 * * * /path/to/sentinel-orchestrator/production/backup/backup-all.sh
```

### Manual Backups

```bash
# Backup all data
./production/backup/backup-all.sh

# Backup specific component
./production/backup/backup-sled.sh
./production/backup/backup-qdrant.sh
./production/backup/backup-postgres.sh
```

## Maintenance

### Update Services

```bash
# Pull latest images
docker-compose -f production/docker/docker-compose.prod.yaml \
  --env-file production/config/.env.production pull

# Recreate services
docker-compose -f production/docker/docker-compose.prod.yaml \
  --env-file production/config/.env.production up -d
```

### View Logs

```bash
# All services
docker-compose -f production/docker/docker-compose.prod.yaml logs -f

# Specific service
docker-compose -f production/docker/docker-compose.prod.yaml logs -f backend
```

### Restart Services

```bash
# Restart all
docker-compose -f production/docker/docker-compose.prod.yaml \
  --env-file production/config/.env.production restart

# Restart specific service
docker-compose -f production/docker/docker-compose.prod.yaml \
  --env-file production/config/.env.production restart backend
```

## Security Considerations

1. **TLS Certificates**: Use Let's Encrypt for production
2. **Secrets**: Never commit `.env.production` to version control
3. **Firewall**: Restrict access to monitoring ports (9090, 3001, 3100)
4. **API Keys**: Rotate API keys regularly
5. **Updates**: Keep Docker images updated

## Troubleshooting

See [Incident Response Runbook](../runbooks/incident-response.md) for detailed troubleshooting.

### Common Issues

**Services won't start:**
- Check logs: `docker-compose logs <service>`
- Verify environment variables
- Check resource limits

**Health checks failing:**
- Verify service is running
- Check network connectivity
- Review service logs

**High resource usage:**
- Check resource limits in docker-compose
- Review application logs
- Consider scaling resources

## Support

For issues or questions:
- Check runbooks in `production/runbooks/`
- Review logs in Grafana
- Check Prometheus metrics
- Contact on-call engineer


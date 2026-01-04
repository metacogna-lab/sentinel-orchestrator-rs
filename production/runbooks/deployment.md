# Deployment Runbook

This runbook provides step-by-step procedures for deploying Sentinel Orchestrator to production.

## Pre-Deployment Checklist

- [ ] All tests passing in CI
- [ ] Security audit passed
- [ ] Docker image built and scanned
- [ ] Environment variables configured
- [ ] Database migrations reviewed (if applicable)
- [ ] Backup of current production data completed
- [ ] Rollback plan prepared
- [ ] Team notified of deployment window

## Deployment Steps

### 1. Prepare Environment

```bash
# Navigate to production directory
cd production/docker

# Copy environment template
cp ../config/env.production.template .env.production

# Edit .env.production with actual values
# NEVER commit this file to version control
nano .env.production
```

### 2. Verify Configuration

```bash
# Validate Docker Compose configuration
docker-compose -f docker-compose.prod.yaml config

# Check environment variables
docker-compose -f docker-compose.prod.yaml --env-file .env.production config
```

### 3. Pull Latest Images

```bash
# Pull latest images (if using registry)
docker-compose -f docker-compose.prod.yaml --env-file .env.production pull

# Or build locally
docker-compose -f docker-compose.prod.yaml --env-file .env.production build
```

### 4. Deploy Services

```bash
# Start services in detached mode
docker-compose -f docker-compose.prod.yaml --env-file .env.production up -d

# Monitor startup logs
docker-compose -f docker-compose.prod.yaml --env-file .env.production logs -f
```

### 5. Verify Deployment

```bash
# Check service health
curl http://localhost/health
curl http://localhost/health/ready
curl http://localhost/health/live

# Check service status
docker-compose -f docker-compose.prod.yaml --env-file .env.production ps

# Verify all services are healthy
docker-compose -f docker-compose.prod.yaml --env-file .env.production ps | grep -v "Up (healthy)"
```

### 6. Smoke Tests

```bash
# Test API endpoint
curl -X POST http://localhost/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d '{
    "messages": [{
      "id": "test-1",
      "role": "user",
      "content": "Hello",
      "timestamp": "2025-01-20T10:00:00Z",
      "metadata": {}
    }]
  }'

# Check metrics endpoint
curl http://localhost:9090/metrics

# Verify Grafana is accessible
curl http://localhost:3001/api/health
```

## Post-Deployment Verification

- [ ] All health checks passing
- [ ] API endpoints responding correctly
- [ ] Metrics being collected
- [ ] Logs being aggregated
- [ ] No errors in service logs
- [ ] Database connections working
- [ ] Vector store accessible

## Rollback Procedure

If deployment fails:

```bash
# Stop new services
docker-compose -f docker-compose.prod.yaml --env-file .env.production down

# Restore previous version
# (Restore from backup or use previous image tag)
docker-compose -f docker-compose.prod.yaml --env-file .env.production up -d

# Verify rollback
curl http://localhost/health
```

## Service Restart

To restart a specific service:

```bash
# Restart backend
docker-compose -f docker-compose.prod.yaml --env-file .env.production restart backend

# Restart all services
docker-compose -f docker-compose.prod.yaml --env-file .env.production restart
```

## Update Procedure

To update to a new version:

```bash
# Pull new images
docker-compose -f docker-compose.prod.yaml --env-file .env.production pull backend

# Recreate services with new images
docker-compose -f docker-compose.prod.yaml --env-file .env.production up -d --no-deps backend

# Verify update
docker-compose -f docker-compose.prod.yaml --env-file .env.production ps backend
```

## Troubleshooting

### Service Won't Start

1. Check logs: `docker-compose logs <service-name>`
2. Verify environment variables
3. Check resource limits
4. Verify dependencies are healthy

### Health Checks Failing

1. Check service logs
2. Verify network connectivity
3. Check port conflicts
4. Verify health check endpoint is accessible

### Database Connection Issues

1. Verify PostgreSQL is running
2. Check connection string in environment
3. Verify network connectivity
4. Check database credentials

## Emergency Contacts

- On-call Engineer: [Contact Info]
- Database Admin: [Contact Info]
- Infrastructure Team: [Contact Info]


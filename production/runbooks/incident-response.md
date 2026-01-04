# Incident Response Runbook

This runbook provides procedures for responding to incidents in Sentinel Orchestrator production.

## Incident Severity Levels

### Critical (P0)
- Service completely down
- Data loss or corruption
- Security breach
- **Response Time**: Immediate

### High (P1)
- Service degraded significantly
- High error rate (>10%)
- Performance issues affecting users
- **Response Time**: < 15 minutes

### Medium (P2)
- Service degraded moderately
- Some errors occurring
- Performance degradation
- **Response Time**: < 1 hour

### Low (P3)
- Minor issues
- Non-critical bugs
- **Response Time**: < 4 hours

## Incident Response Process

### 1. Detection

Incidents may be detected through:
- Alert notifications (Prometheus alerts)
- Monitoring dashboards (Grafana)
- User reports
- Log analysis

### 2. Triage

**Immediate Actions:**
1. Acknowledge the incident
2. Assess severity level
3. Check service status: `curl http://localhost/health`
4. Review recent changes/deployments
5. Check monitoring dashboards

**Information to Gather:**
- When did the issue start?
- What services are affected?
- Error messages from logs
- Recent deployment or configuration changes
- Metrics showing anomalies

### 3. Investigation

**Check Service Status:**
```bash
# Service health
docker-compose -f production/docker/docker-compose.prod.yaml ps

# Service logs
docker-compose -f production/docker/docker-compose.prod.yaml logs --tail=100 backend

# Resource usage
docker stats
```

**Check Metrics:**
- Open Grafana dashboards
- Review Prometheus metrics
- Check error rates, latency, resource usage

**Check Logs:**
- Review Loki logs
- Search for error patterns
- Check for exceptions

### 4. Resolution

**Common Issues and Resolutions:**

#### High Error Rate
1. Check backend logs for errors
2. Verify database connectivity
3. Check for resource exhaustion
4. Restart service if needed: `docker-compose restart backend`

#### High Latency
1. Check resource usage (CPU, memory)
2. Review slow queries in database
3. Check for backpressure in channels
4. Scale resources if needed

#### Service Down
1. Check if container is running: `docker ps`
2. Check container logs: `docker logs sentinel-backend-prod`
3. Verify dependencies (PostgreSQL, Qdrant)
4. Restart service: `docker-compose restart backend`
5. If persistent, check configuration and environment variables

#### Database Connection Issues
1. Verify PostgreSQL is running: `docker ps | grep postgres`
2. Check connection string
3. Verify network connectivity
4. Check database logs: `docker logs sentinel-postgres-prod`

#### Circuit Breaker Open
1. Check why circuit breaker opened (too many failures)
2. Verify upstream service health
3. Wait for circuit breaker to close (automatic)
4. Or manually reset if needed

#### Memory Issues
1. Check memory usage: `docker stats`
2. Review memory consolidation logs
3. Check for memory leaks
4. Restart service if needed

### 5. Communication

**During Incident:**
- Update status page/chat channel
- Notify stakeholders
- Document actions taken

**After Resolution:**
- Post-mortem meeting
- Document root cause
- Create action items to prevent recurrence

## Escalation

If incident cannot be resolved:
1. Escalate to senior engineer
2. Contact on-call manager
3. Consider rollback if recent deployment

## Post-Incident

### Post-Mortem Template

1. **Incident Summary**
   - What happened?
   - When did it occur?
   - Duration of incident
   - Impact (users affected, services affected)

2. **Timeline**
   - Detection time
   - Response time
   - Resolution time
   - Key actions taken

3. **Root Cause**
   - Primary cause
   - Contributing factors

4. **Resolution**
   - How was it fixed?
   - Temporary vs permanent fix

5. **Action Items**
   - Prevent recurrence
   - Improve detection
   - Improve response time
   - Update documentation

## Common Alerts and Responses

### Alert: HighErrorRate
- **Action**: Check logs, verify dependencies, restart if needed
- **Escalation**: If persists > 10 minutes

### Alert: HighLatency
- **Action**: Check resource usage, review slow operations
- **Escalation**: If p99 > 1s for > 15 minutes

### Alert: CircuitBreakerOpen
- **Action**: Check upstream service, wait for auto-recovery
- **Escalation**: If open > 5 minutes

### Alert: ServiceDown
- **Action**: Immediate restart, check logs
- **Escalation**: If restart fails

### Alert: BudgetExceeded
- **Action**: Stop accepting new requests, investigate usage
- **Escalation**: Immediate - critical

## Emergency Contacts

- On-call Engineer: [Contact]
- Database Admin: [Contact]
- Infrastructure: [Contact]
- Security Team: [Contact]


# SpaceComms Operations & Runbook

_Deployment, monitoring, and troubleshooting guide_

---

## Deployment

### Prerequisites

- Rust 1.75+ or pre-built binary
- TLS certificates (for production)
- Network connectivity to peers

### Development Deployment

```bash
# Clone repository
git clone https://github.com/your-org/spacecomms.git
cd spacecomms

# Build
cd spacecomms-core
cargo build --release

# Create config
cp ../examples/config.yaml config.yaml
# Edit config.yaml with your settings

# Run
./target/release/spacecomms start --config config.yaml
```

### Production Deployment

#### Using Docker

```bash
# Build image
docker build -t spacecomms:latest .

# Run with mounted config
docker run -d \
  --name spacecomms \
  -p 8080:8080 \
  -v /etc/spacecomms:/etc/spacecomms:ro \
  -v /var/lib/spacecomms:/var/lib/spacecomms \
  spacecomms:latest \
  start --config /etc/spacecomms/config.yaml
```

#### Using systemd

Create `/etc/systemd/system/spacecomms.service`:

```ini
[Unit]
Description=SpaceComms Protocol Service
After=network.target

[Service]
Type=simple
User=spacecomms
Group=spacecomms
ExecStart=/usr/local/bin/spacecomms start --config /etc/spacecomms/config.yaml
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

```bash
sudo systemctl enable spacecomms
sudo systemctl start spacecomms
```

---

## Configuration

### Configuration File

`/etc/spacecomms/config.yaml`:

```yaml
# Node identity
node:
  id: "node-prod-01"
  name: "Production Node 01"

# Network settings
server:
  host: "0.0.0.0"
  port: 8080
  tls:
    enabled: true
    cert_path: "/etc/spacecomms/certs/server.crt"
    key_path: "/etc/spacecomms/certs/server.key"

# API authentication
api:
  auth:
    enabled: true
    tokens:
      - id: "admin"
        secret: "${SPACECOMMS_ADMIN_TOKEN}"
        permissions: ["read", "write", "admin"]
      - id: "readonly"
        secret: "${SPACECOMMS_READONLY_TOKEN}"
        permissions: ["read"]

# Peer connections
peers:
  - id: "peer-operator-a"
    address: "https://operator-a.example.com:8443"
    auth_token: "${PEER_A_TOKEN}"
    policies:
      accept_cdm: true
      accept_object_state: true
      forward_cdm: true

# Storage
storage:
  type: "memory" # or "file" for persistence
  file_path: "/var/lib/spacecomms/data"

# Logging
logging:
  level: "info" # debug, info, warn, error
  format: "json" # json or pretty
  output: "stdout" # stdout or file path

# Protocol settings
protocol:
  heartbeat_interval_seconds: 30
  session_timeout_seconds: 120
  max_hop_count: 10
```

### Environment Variables

| Variable                    | Description         |
| --------------------------- | ------------------- |
| `SPACECOMMS_CONFIG`         | Config file path    |
| `SPACECOMMS_LOG_LEVEL`      | Override log level  |
| `SPACECOMMS_ADMIN_TOKEN`    | Admin API token     |
| `SPACECOMMS_READONLY_TOKEN` | Read-only API token |

---

## Peering Setup

### Adding a New Peer

1. **Exchange certificates** (if using mTLS)

   ```bash
   # Generate peer-specific client cert
   openssl req -new -key client.key -out client.csr
   # Have peer sign with their CA
   ```

2. **Get peer authentication token**
   - Coordinate with peer administrator
   - Store securely (not in config file)

3. **Add peer via API**

   ```bash
   curl -X POST http://localhost:8080/peers \
     -H "Authorization: Bearer $ADMIN_TOKEN" \
     -H "Content-Type: application/json" \
     -d '{
       "peer_id": "peer-new-operator",
       "address": "https://new-operator.example.com:8443",
       "auth_token": "'$PEER_TOKEN'",
       "policies": {
         "accept_cdm": true,
         "accept_object_state": true
       }
     }'
   ```

4. **Verify connection**
   ```bash
   curl http://localhost:8080/peers | jq '.peers[] | select(.peer_id == "peer-new-operator")'
   ```

### Removing a Peer

```bash
curl -X DELETE http://localhost:8080/peers/peer-new-operator \
  -H "Authorization: Bearer $ADMIN_TOKEN"
```

---

## Monitoring

### Health Check

```bash
# Basic health
curl http://localhost:8080/health

# Expected response
{
  "status": "healthy",
  "node_id": "node-prod-01",
  "uptime_seconds": 86400,
  "peers": {
    "connected": 3,
    "total": 5
  },
  "objects_tracked": 1250,
  "cdms_active": 42
}
```

### Logs to Watch

| Log Pattern                | Meaning                     | Action                    |
| -------------------------- | --------------------------- | ------------------------- |
| `peer session established` | New peer connected          | Normal                    |
| `peer session lost`        | Peer disconnected           | Investigate if unexpected |
| `cdm validation failed`    | Invalid CDM received        | Check source data         |
| `rate limit exceeded`      | Too many messages from peer | Review peer policies      |
| `authentication failed`    | Invalid token               | Check credentials         |

### Key Metrics

Monitor these for operational health:

| Metric                   | Normal Range | Alert Threshold |
| ------------------------ | ------------ | --------------- |
| `peers_connected`        | >0           | 0 (no peers)    |
| `heartbeat_latency_ms`   | <1000        | >5000           |
| `cdm_processing_time_ms` | <100         | >1000           |
| `message_queue_depth`    | <100         | >1000           |
| `error_rate`             | <1%          | >5%             |

---

## Troubleshooting

### Common Issues

#### Node won't start

**Symptom**: Service fails to start, immediate exit

**Check**:

```bash
# Validate config
spacecomms validate-config --config config.yaml

# Check port availability
lsof -i :8080

# Check TLS certs
openssl x509 -in /etc/spacecomms/certs/server.crt -text -noout
```

**Common causes**:

- Port already in use
- Invalid config YAML
- TLS cert/key mismatch
- Insufficient permissions

---

#### Peer won't connect

**Symptom**: Peer status shows "disconnected" or "connecting"

**Check**:

```bash
# Test network connectivity
curl -v https://peer.example.com:8443/health

# Check DNS resolution
dig peer.example.com

# Verify TLS (if mTLS)
openssl s_client -connect peer.example.com:8443
```

**Common causes**:

- Network/firewall blocking
- DNS resolution failure
- TLS certificate issues
- Authentication token mismatch

---

#### CDMs not propagating

**Symptom**: CDMs ingested but not appearing at peers

**Check**:

```bash
# Check peer connection status
curl http://localhost:8080/peers

# Check routing policies
grep -A 10 "policies:" /etc/spacecomms/config.yaml

# Check logs for routing decisions
journalctl -u spacecomms | grep "routing decision"
```

**Common causes**:

- Peer not connected
- Routing policy rejecting messages
- TTL exhausted
- Loop detection blocking

---

#### High memory usage

**Symptom**: Memory consumption growing over time

**Check**:

```bash
# Check object/CDM counts
curl http://localhost:8080/health

# List old CDMs
curl "http://localhost:8080/cdms?limit=10&sort=created_at"
```

**Common causes**:

- CDMs not being withdrawn after TCA
- Object states accumulating
- Memory leak (report as bug)

**Mitigation**:

```yaml
# Configure automatic cleanup in config.yaml
storage:
  cleanup:
    enabled: true
    cdm_retention_hours: 168 # 7 days
    object_retention_hours: 720 # 30 days
```

---

### Debug Mode

For detailed troubleshooting:

```bash
# Run with debug logging
SPACECOMMS_LOG_LEVEL=debug spacecomms start --config config.yaml

# Or set in config
logging:
  level: "debug"
```

**Warning**: Debug logging is verbose. Don't run in production for extended periods.

---

## Backup and Recovery

### Data Backup

If using file-based storage:

```bash
# Stop service
sudo systemctl stop spacecomms

# Backup data directory
tar -czf spacecomms-backup-$(date +%Y%m%d).tar.gz /var/lib/spacecomms/

# Start service
sudo systemctl start spacecomms
```

### Recovery from Backup

```bash
# Stop service
sudo systemctl stop spacecomms

# Restore data
tar -xzf spacecomms-backup-20240115.tar.gz -C /

# Start service
sudo systemctl start spacecomms
```

### Disaster Recovery

For complete node failure:

1. Deploy new instance
2. Configure with same `node_id`
3. Re-establish peer connections
4. Peers will re-announce current CDMs/objects

---

## Maintenance

### Rolling Restart

For zero-downtime updates with multiple nodes:

```bash
# Node 1
sudo systemctl stop spacecomms
# ... update binary/config ...
sudo systemctl start spacecomms
# Wait for peers to reconnect

# Node 2
# ... repeat ...
```

### Config Reload

Some changes don't require restart:

```bash
# Reload config (peers, policies)
kill -HUP $(pidof spacecomms)
```

### Version Upgrade

1. Review release notes for breaking changes
2. Backup current configuration and data
3. Test upgrade in staging environment
4. Coordinate with peers on version compatibility
5. Perform rolling upgrade

---

## Security Hardening

### Network

- Use TLS 1.3 only
- Enable mTLS for peer connections
- Restrict API access to authorized networks
- Use firewall rules to limit peer IPs

### Authentication

- Rotate API tokens regularly
- Use separate tokens for different access levels
- Store tokens in secrets manager, not config files

### Audit

- Enable debug logging for security events
- Forward logs to SIEM
- Review peer connection history

### Container Security

```yaml
# docker-compose.yml security settings
services:
  spacecomms:
    security_opt:
      - no-new-privileges:true
    read_only: true
    tmpfs:
      - /tmp
    user: "1000:1000"
```

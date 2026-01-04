# Demo Certificates for SpaceComms

⚠️ **WARNING: These certificates are for LOCAL DEVELOPMENT AND DEMO ONLY.**

Do NOT use these certificates in production environments.

## Generation

Run the generation script:

```bash
cd dev-certs
chmod +x generate-certs.sh
./generate-certs.sh
```

This creates:

| File                       | Purpose                                       |
| -------------------------- | --------------------------------------------- |
| `ca.key`, `ca.crt`         | Certificate Authority (signs all other certs) |
| `node-a.key`, `node-a.crt` | Node A server/client certificate              |
| `node-b.key`, `node-b.crt` | Node B server/client certificate              |
| `client.key`, `client.crt` | Generic client certificate for testing        |

## Certificate Details

- **Validity**: 365 days from generation
- **Key Size**: 2048-bit RSA
- **SAN Entries**: localhost, 127.0.0.1, ::1, node-specific hostnames
- **Usage**: Server and client authentication (mTLS)

## Usage with SpaceComms

### Starting Secure Nodes

```bash
# Start Node A with TLS
./spacecomms start --config examples/node-a-tls-config.yaml

# Start Node B with TLS
./spacecomms start --config examples/node-b-tls-config.yaml
```

### Testing with curl

```bash
# Health check with client cert
curl --cacert dev-certs/ca.crt \
     --cert dev-certs/client.crt \
     --key dev-certs/client.key \
     https://localhost:8443/health

# Without client cert (should fail if mTLS required)
curl --cacert dev-certs/ca.crt https://localhost:8443/health
```

## Regenerating Certificates

If certificates expire or you need fresh ones:

```bash
cd dev-certs
rm -f *.key *.crt
./generate-certs.sh
```

## Security Notes

1. **Private keys** (`.key` files) are set to mode 600 (owner read/write only)
2. **Certificates** (`.crt` files) are set to mode 644 (world readable)
3. These certs use a self-signed CA - browsers/tools will show warnings
4. The CA private key should NEVER be distributed in production

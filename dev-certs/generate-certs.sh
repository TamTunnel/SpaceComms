#!/bin/bash
# Generate demo certificates for SpaceComms mTLS
#
# WARNING: These certificates are for LOCAL DEVELOPMENT AND DEMO ONLY.
# Do NOT use in production environments.

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "╔═══════════════════════════════════════════════════════════╗"
echo "║     SpaceComms Demo Certificate Generator                 ║"
echo "╚═══════════════════════════════════════════════════════════╝"
echo ""
echo "⚠️  WARNING: These certificates are for LOCAL DEMO ONLY!"
echo ""

# Configuration
DAYS=365
RSA_BITS=2048

# Generate CA private key and certificate
echo "[1/5] Generating Certificate Authority (CA)..."
openssl genrsa -out ca.key $RSA_BITS 2>/dev/null
openssl req -new -x509 -days $DAYS -key ca.key -out ca.crt \
    -subj "/C=US/ST=Demo/L=Demo/O=SpaceComms Demo CA/CN=SpaceComms Demo CA" \
    2>/dev/null
echo "      ✓ ca.key, ca.crt"

# Generate Node A certificate
echo "[2/5] Generating Node A certificate..."
openssl genrsa -out node-a.key $RSA_BITS 2>/dev/null
openssl req -new -key node-a.key -out node-a.csr \
    -subj "/C=US/ST=Demo/L=Demo/O=SpaceComms Node A/CN=localhost" \
    2>/dev/null

# Create SAN config for Node A
cat > node-a-san.cnf << EOF
[req]
distinguished_name = req_distinguished_name
req_extensions = v3_req
prompt = no

[req_distinguished_name]
C = US
ST = Demo
L = Demo
O = SpaceComms Node A
CN = localhost

[v3_req]
keyUsage = keyEncipherment, dataEncipherment
extendedKeyUsage = serverAuth, clientAuth
subjectAltName = @alt_names

[alt_names]
DNS.1 = localhost
DNS.2 = node-a
DNS.3 = spacecomms-node-a
IP.1 = 127.0.0.1
IP.2 = ::1
EOF

openssl x509 -req -days $DAYS -in node-a.csr -CA ca.crt -CAkey ca.key \
    -CAcreateserial -out node-a.crt -extfile node-a-san.cnf -extensions v3_req \
    2>/dev/null
rm node-a.csr node-a-san.cnf
echo "      ✓ node-a.key, node-a.crt"

# Generate Node B certificate
echo "[3/5] Generating Node B certificate..."
openssl genrsa -out node-b.key $RSA_BITS 2>/dev/null
openssl req -new -key node-b.key -out node-b.csr \
    -subj "/C=US/ST=Demo/L=Demo/O=SpaceComms Node B/CN=localhost" \
    2>/dev/null

# Create SAN config for Node B
cat > node-b-san.cnf << EOF
[req]
distinguished_name = req_distinguished_name
req_extensions = v3_req
prompt = no

[req_distinguished_name]
C = US
ST = Demo
L = Demo
O = SpaceComms Node B
CN = localhost

[v3_req]
keyUsage = keyEncipherment, dataEncipherment
extendedKeyUsage = serverAuth, clientAuth
subjectAltName = @alt_names

[alt_names]
DNS.1 = localhost
DNS.2 = node-b
DNS.3 = spacecomms-node-b
IP.1 = 127.0.0.1
IP.2 = ::1
EOF

openssl x509 -req -days $DAYS -in node-b.csr -CA ca.crt -CAkey ca.key \
    -CAcreateserial -out node-b.crt -extfile node-b-san.cnf -extensions v3_req \
    2>/dev/null
rm node-b.csr node-b-san.cnf
echo "      ✓ node-b.key, node-b.crt"

# Generate client certificate (for testing)
echo "[4/5] Generating client certificate..."
openssl genrsa -out client.key $RSA_BITS 2>/dev/null
openssl req -new -key client.key -out client.csr \
    -subj "/C=US/ST=Demo/L=Demo/O=SpaceComms Client/CN=test-client" \
    2>/dev/null
openssl x509 -req -days $DAYS -in client.csr -CA ca.crt -CAkey ca.key \
    -CAcreateserial -out client.crt \
    2>/dev/null
rm client.csr
echo "      ✓ client.key, client.crt"

# Clean up serial file
rm -f ca.srl

echo "[5/5] Setting permissions..."
chmod 600 *.key
chmod 644 *.crt
echo "      ✓ Keys: 600, Certs: 644"

echo ""
echo "═══════════════════════════════════════════════════════════"
echo "Certificate generation complete!"
echo ""
echo "Files created:"
echo "  CA:       ca.key, ca.crt"
echo "  Node A:   node-a.key, node-a.crt"
echo "  Node B:   node-b.key, node-b.crt"
echo "  Client:   client.key, client.crt"
echo ""
echo "Usage:"
echo "  Start Node A:  ./spacecomms start --config examples/node-a-tls-config.yaml"
echo "  Start Node B:  ./spacecomms start --config examples/node-b-tls-config.yaml"
echo ""
echo "⚠️  These certs expire in $DAYS days and are for DEMO ONLY!"
echo "═══════════════════════════════════════════════════════════"

#!/bin/sh
mkdir certs && cd certs

# 1. Criar a CA (Autoridade Certificadora) raiz
openssl genrsa -out ca.key 4096
openssl req -new -x509 -days 365 -key ca.key -out ca.pem \
  -subj "/CN=MinhaCARaiz"

# 2. Criar certificado do SERVIDOR
openssl genrsa -out server.key 2048
openssl req -new -key server.key -out server.csr -subj "/CN=localhost"
openssl x509 -req -days 365 -in server.csr -CA ca.pem \
  -CAkey ca.key -CAcreateserial -out server.crt

# 3. Criar certificado do CLIENTE (para mTLS)
openssl genrsa -out client.key 2048
openssl req -new -key client.key -out client.csr -subj "/CN=cliente-autorizado"
openssl x509 -req -days 365 -in client.csr -CA ca.pem \
  -CAkey ca.key -CAcreateserial -out client.crt
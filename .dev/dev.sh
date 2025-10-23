openssl req -x509 -newkey rsa:4096 \
  -keyout key.pem -out cert.pem \
  -days 365 -nodes \
  -subj "/CN=localhost" \
  -addext "basicConstraints=CA:FALSE" \
  -addext "keyUsage = digitalSignature, keyEncipherment" \
  -addext "extendedKeyUsage = serverAuth, clientAuth" \
  -addext "subjectAltName=DNS:localhost"

python -m http.server 8000
python -m http.server 8080

curl --cacert certs/cert.pem https://localhost:2776
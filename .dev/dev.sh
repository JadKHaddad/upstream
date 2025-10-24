openssl req -x509 -newkey rsa:4096 \
  -keyout key.pem -out cert.pem \
  -days 365 -nodes \
  -subj "/CN=localhost" \
  -addext "basicConstraints=CA:FALSE" \
  -addext "keyUsage = digitalSignature, keyEncipherment" \
  -addext "extendedKeyUsage = serverAuth, clientAuth" \
  -addext "subjectAltName=DNS:localhost"

python3 -m http.server 8001
python3 -m http.server 8081

curl --cacert certs/cert.pem https://localhost:2776
curl http://localhost:2777
# Round trip
curl http://localhost:2779 # --> https://localhost:2776 --> [127.0.0.1:8001, 127.0.0.1:8081]

curl http://localhost:2778 # should connect to google.com:443. Google will respond with 404 since curl is setting the Host header to localhost

# This works around curl setting the Host header to google.com
printf 'GET / HTTP/1.1\r\nHost: google.com\r\nConnection: close\r\n\r\n' | nc localhost 2778 -q 1

FROM rust:1.90-alpine AS builder
RUN apk add --no-cache musl-dev
WORKDIR /usr/src/upstream
COPY . .
RUN cargo build -p upstream --release --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /usr/src/upstream/target/x86_64-unknown-linux-musl/release/upstream /upstream
ENTRYPOINT ["/upstream"]

# docker build -t jadkhaddad/upstream:latest .
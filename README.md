# certcheck

Check TLS certificate details for a host.

## Install

```console
cargo build --release
sudo cp target/release/certcheck /usr/local/bin/
```

## Usage

```console
certcheck example.com
certcheck google.com --port 443
```

Output:

```
Subject: /CN=example.com
Issuer: /CN=Let's Encrypt Authority X3
Valid from: 2025-01-01
Valid until: 2025-04-01
```

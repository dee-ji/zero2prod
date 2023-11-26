# zero2prod
An email newsletter Rust app

---

## Purpose
This is a Rust Project to learn how to build a backend. This project is for a newsletter app where someone can subscribe to it.

---

## Useful packages

### clippy - Linting
- Ensure `clippy` was installed with `rustup`
```bash
rustup component add clippy
```
- Run `clippy` on the project
```bash
cargo clippy
```
- Run `clippy` with additional warnings, which would fail commit if linter sees any warnings
```bash
cargo clippy -- -D warnings
```

### fmt - Code Formatting
- Ensure `fmt` was installed with `rustup`
```bash
rustup component add rustfmt
```
- Run `fmt` on the project
```bash
cargo fmt
```
- Run fmt check on the project. This would fail if the commit contains unformatted code
```bash
cargo fmt -- --check
```

### cargo-audit - Security Vulnerabilities
- To install
```bash
cargo install cargo-audit
```
- To run
```bash
cargo audit
```

### cargo watch
- To install
```bash
cargo install cargo-watch
```
- To run `cargo watch` to continuously check project compilation errors
```bash
cargo watch -x check
```
- To run `cargo watch` to run check, then tests, then runs program if everything passes
```bash
cargo watch -x check -x test -x run
```

---

## Useful commands

### sqlx
- Install `sqlx-cli`
```bash
cargo install --version="~0.6" sqlx-cli --no-default-features --features rustls,postgres
```

---

### DigitalOcean
- To run a migration on the DigitalOcean Postgres DB:
```bash
sqlx migrate run --database-url postgresql://${DB_USERNAME}:${DB_PASSWORD}@${DB_URL}:${DB_PORT}/newsletter?sslmode=require --connect-timeout 90
```
- To list active services on DigitalOcean
```bash
doctl apps list
```
- To create an app on DigitalOcean
```bash
doctl apps create --spec spec.yaml
```
- To update an existing app on DigitalOcean
```bash
doctl apps update ${APP_ID} --spec=spec.yaml
```

---

### cURL
- To create a test account in the Postgres DB:
```bash
curl --request POST --data 'name=le%20guin&email=ursula_le_guin%40gmail.com' http://localhost/subscriptions --verbose
```

---
  
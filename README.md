# zero2prod
A Rust app for email newsletter

## Purpose
This is a Rust Project to learn how to build a backend. This project is for a newsletter app where someone can subscribe to it.

## Useful commands
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
- To create a test account in the Postgres DB:
```bash
curl --request POST --data 'name=le%20guin&email=ursula_le_guin%40gmail.com' http://localhost/subscriptions --verbose
```
  
# Kvault (back edition)

[![build status](https://img.shields.io/github/actions/workflow/status/skewnart/kvault-back/build_test.yml)](https://github.com/skewnart/kvault-back/actions)

This repository contains the Rust HTTP API for Kvault. It provides account
registration and login, authentication, and storage for encrypted folders,
entries, and encryption envelopes. The front-end is maintained separately in
[skewnart/kvault-front](https://github.com/skewnart/kvault-front).

## Features

- Register an account using an invitation identifier.
- Authenticate users and issue RS256 JWTs.
- Manage encrypted folders and entries belonging to the authenticated user.
- Retrieve the user's profile and encryption envelope.
- Manage registration invitations as an administrator.
- Expose the API version.

## Tech stack

- Rust (edition 2024) and [Actix Web](https://actix.rs/)
- PostgreSQL with `tokio-postgres` and `deadpool-postgres`
- RS256 JSON Web Tokens

## Prerequisites

- Rust **1.85 or newer** (edition 2024 support)
- PostgreSQL
- RSA public and private keys in PEM format, base64-encoded for the environment
  variables

## Installation and development

Clone the repository, configure the environment as described below, then build
and run the API from the repository root:

```bash
cargo build
cargo run
```

The server binds to `0.0.0.0` on the port configured by `SERVER__PORT`. With the
example configuration below, the API is available at
`http://localhost:1234/api`.

Run the test suite with:

```bash
cargo test
```

## .env file

Following inputs are exhaustive:

```dotenv
# Server
SERVER__PORT=1234

# PostgreSQL configuration
DATABASE__USER=the_user
DATABASE__PASSWORD=his_password
DATABASE__HOST=the_host
DATABASE__PORT=5432
DATABASE__DBNAME=db_name
DATABASE__POOL_MAX_SIZE=16

# RSA KEYS
JWT__SK=base_64_utf8_encoded_RSA_256_secret_key
JWT__PK=base_64_utf8_encoded_RSA_256_public_key
JWT__TTL=ttl_in_seconds

# Log level
RUST_LOG=INFO
```

The application loads `.env` at startup. `JWT__SK` and `JWT__PK` must each
contain the base64-encoded PEM contents of the matching RSA private and public
keys. Keep the private key secret.

## JWT key management

JWT generation and validation require the matching RSA private and public keys.
Rotate these keys regularly. They are currently loaded from `.env` at startup,
so restart the application after changing either key.

## Database migrations

The application checks its PostgreSQL connection at startup but does not run
migrations automatically. Before the first launch, apply the `up.sql` files in
`resources/migrations/` in version order, from `0.1.0-alpha` through the latest
version. The corresponding `down.sql` files contain rollback scripts.

## API

All routes are mounted under `/api`. Protected routes require an
`Authorization` header with the `Bearer` scheme followed by a JWT.

| Method | Path | Authentication | Purpose |
| --- | --- | --- | --- |
| `POST` | `/api/connection/login` | No | Sign in and receive a JWT. |
| `POST` | `/api/connection/register/{guid}` | No | Register using an invitation identifier. |
| `GET` | `/api/connection/register/invitations` | Admin | List registration invitations. |
| `POST` | `/api/connection/register/invitations` | Admin | Create a registration invitation. |
| `GET` | `/api/profile` | Yes | Retrieve the authenticated user's profile. |
| `GET` | `/api/envelope` | Yes | Retrieve the user's encryption envelope. |
| `GET` | `/api/folder` | Yes | List the user's folders. |
| `POST` | `/api/folder` | Yes | Replace the user's encrypted folder data. |
| `POST` | `/api/folder/new` | Yes | Create a folder. |
| `GET` | `/api/folder/{id}` | Yes | Retrieve a folder's encrypted data. |
| `PUT` | `/api/folder/{id}` | Yes | Update a folder. |
| `DELETE` | `/api/folder/{id}` | Yes | Delete a folder. |
| `POST` | `/api/entry` | Yes | Create an entry. |
| `DELETE` | `/api/entry` | Yes | Delete multiple entries by their IDs. |
| `GET` | `/api/entry/{id}` | Yes | Retrieve an entry's encrypted data. |
| `PUT` | `/api/entry/{id}` | Yes | Update an entry. |
| `DELETE` | `/api/entry/{id}` | Yes | Delete an entry. |
| `GET` | `/api/infos/version` | No | Retrieve the API version. |

Folder and entry payloads use the `enc_data` JSON field. Login, registration,
and invitation requests and responses are defined by the corresponding
controllers and data-transfer objects in `src/`.

## Front-end integration and API testing

The default CORS configuration allows the front-end origin
`http://localhost:5173`. Update the CORS configuration in `src/middlewares/`
when using a different origin.

An API request collection for [Bruno](https://www.usebruno.com/) is available
in `docs/bruno-collection/`. Select an environment and configure its API base
URL and authentication values before sending requests.

## License

This project is licensed under the GNU General Public License v3.0 or later.
See the [LICENSE](./LICENSE) file for the full text.

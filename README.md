# K-Vault (back edition)

[![build status](https://img.shields.io/github/actions/workflow/status/skewnart/kvault-back/build_test.yml)](https://github.com/skewnart/kvault-back/actions)

K-Vault README file will come with its 1st major version.<br>
See you soon !

## .env file

Following inputs are exhaustive :

```
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
RUST_LOG= TRACE|DEBUG|INFO|WARN|ERROR
```

## JWT

JWT generation needs a RSA-256 public and private (secret) keys in the .env file in order to work.<br>
Please be aware that encryption keys **_NEED TO_** be rotated regularly.

In this version, encryption keys are stored in the .env file and the application needs to restart to reload them.<br>
One day they will be stored outside and hot-reloaded to allow you to automate the rotation.

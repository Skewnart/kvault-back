# K-Vault (back edition)

[![build status](https://img.shields.io/github/actions/workflow/status/skewnart/kvault-back/build_test.yml)](https://github.com/skewnart/kvault-back/actions)

K-Vault README file will come with its 1st major version.
See you soon !

## .env file

Waiting for the README file, I need to write down the .env file structure here, because the application needs some env inputs in order to work.

The following inputs are exhaustive :

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

# Log level
RUST_LOG= TRACE|DEBUG|INFO|WARN|ERROR
```

This needs to be in a ".env" file (it's its full name)

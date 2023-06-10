# Production Overview

This document details how the production instance of Tildes Statistics at [ts.bauke.xyz](https://ts.bauke.xyz) is put together.

## Compose

The Compose setup uses a shared network for every service so no ports ever need to be exposed, helps with making sure UFW and Docker play nice together. Then three services are created, namely:

* `tildes-statistics-app`, the main Tildes Statistics CLI, pulled from `git.bauke.xyz/tildes-community/tildes-statistics:latest`.
* `tildes-statistics-db`, a PostgreSQL 14 instance from the official image.
* `tildes-statistics-netlify`, an unofficial Netlify CLI image (because there is no official one).

The `tildes-statistics-app` and `tildes-statistics-netlify` services share the `data` volume so the CLI can build the website and then Netlify can publish it. The `tildes-statistics-db` service has its own `db` volume so data is persisted.

```yaml
# compose.yaml
version: "3"

networks:
  shared-network:
    external: true

services:
  tildes-statistics-app:
    image: "git.bauke.xyz/tildes-community/tildes-statistics:latest"
    hostname: "tildes-statistics-app"
    container_name: "tildes-statistics-app"
    restart: "no"
    depends_on:
      - "tildes-statistics-db"
    environment:
      DATABASE_URL: "${DATABASE_URL}"
      USER_AGENT: "${USER_AGENT}"
    networks:
      - "shared-network"
    volumes:
      - "data:/public"

  tildes-statistics-db:
    image: "postgres:14"
    hostname: "tildes-statistics-db"
    container_name: "tildes-statistics-db"
    restart: "unless-stopped"
    mem_limit: "200m"
    environment:
      POSTGRES_DB: "${POSTGRES_DB}"
      POSTGRES_USER: "${POSTGRES_USER}"
      POSTGRES_PASSWORD: "${POSTGRES_PASSWORD}"
    networks:
      - "shared-network"
    volumes:
      - "db:/var/lib/postgresql/data"

  tildes-statistics-netlify:
    image: "williamjackson/netlify-cli"
    hostname: "tildes-statistics-netlify"
    container_name: "tildes-statistics-netlify"
    restart: "no"
    depends_on:
      - "tildes-statistics-db"
    environment:
      NETLIFY_AUTH_TOKEN: "${NETLIFY_AUTH_TOKEN}"
    networks:
      - "shared-network"
    volumes:
      - "data:/project"

volumes:
  data:
  db:
```

## Environment

The `.env` file is loaded in by Docker Compose and the variables are replaced in the `compose.yaml` file for each service.

```sh
# .env

# The User-Agent string for HTTP requests, replace the email with yours.
USER_AGENT="Tildes Statistics (Production, your-email@example.org)"

# The full PostgreSQL connection string for the Tildes Statistics CLI.
DATABASE_URL="postgres://username:password@localhost:5432/database"

# The connection details for the PostgreSQL service.
# Make sure these match the DATABASE_URL.
POSTGRES_DB="database"
POSTGRES_USER="username"
POSTGRES_PASSWORD="password"

# The Netlify token to authenticate with.
# https://docs.netlify.com/cli/get-started/#authentication
NETLIFY_AUTH_TOKEN=""
```

## Crontab

The server is set to the UTC timezone so using Cron, deploy the website once a day at 12:00.

```txt
# crontab -e
# Change $USER to your username if you're copying this.
00 12 * * * cd "/home/$USER/tildes-statistics" && ./deploy.sh
```

## Deployment

The `deploy.sh` script then creates the snapshot for today, builds the website and publishes it to Netlify.

```sh
#!/usr/bin/env bash

docker compose run tildes-statistics-app tildes-statistics --no-migrate snapshot create
docker compose run tildes-statistics-app tildes-statistics --no-migrate web build
docker compose run tildes-statistics-netlify deploy --dir . --message "Automated daily 12:00 UTC deployment." --prod --site tildes-statistics
```

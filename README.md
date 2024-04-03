# SearXiv: archivist

[![builds.sr.ht status](https://builds.sr.ht/~mchernigin/searxiv-archivist.svg)](https://builds.sr.ht/~mchernigin/searxiv-archivist?)

This module orchestrates scraping process and acts as a data holder. Only
archivist has direct access to database. It is archivist's responsibly for
building index of papers from https://arxiv.org.

## How to start archivist

Archivist is the one managing database, it's expected to be running on the same
machine that database is running on. So database is included in
`docker-compose.yaml`.

To run archivist you have 2 options.

1. Clone the repo and use `docker-compose.yaml`.

    Before starting it up you need to provided credentials in `.env` file. Copy
    provided `.env.example` into `.env` and make sure all of the variables fit
    your needs. For the development environment there is no need to change
    anything.

    To start archivist and database use docker compose:

    ```sh
    docker compose up
    ```

1. Use docker image built by GitHub CI. In this case you would need to connect
   to already existing database or start one separately.

    ```sh
    docker run -e DATABASE_URL="postgres://searxiv:1234@fire:5432/searxiv" ghcr.io/searxiv/archivist:latest
    ```

## Available API

You can explore everything archivist can and can't do via RapiDoc available at
`/docs`.


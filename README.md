# SearXiv: archivist

This module orchestrates scraping process and acts as data holder. Only
archivist has direct access to database. It is archivist's responsibly for
building index of papers from <arxiv.org>.

## How to startup archivist

Archivist is the one managing database, it's expected to be running on the same
machine that database is running on. So database is included in
`docker-compose.yaml`.

Before starting it up you need to provided credentials in `.env` file. Copy
provided `.env.example` into `.env` and make sure all of the variables fit your
needs. For development environment there is no need to change anything.

To start archivist and database use docker compose:

```sh
$ docker compose up
```
## Available API

You can explore everything archivist can and can't do via RapiDoc available at
"/docs".

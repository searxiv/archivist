# Builder ######################################################################

FROM rust:1.76-bookworm as builder

WORKDIR /searxiv
COPY Cargo* .
COPY src src
COPY .sqlx .sqlx

ENV SQLX_OFFLINE true

RUN cargo build --release

# Runtime ######################################################################

FROM debian:bookworm-slim AS runtime

COPY --from=builder /searxiv/target/release/archivist /archivist

ENTRYPOINT ["/archivist"]


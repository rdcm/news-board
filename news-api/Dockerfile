# build image
FROM rust:1.79.0-slim-buster as build
WORKDIR /app
COPY . .
RUN apt-get update && apt-get install -y protobuf-compiler \
     libpq-dev
RUN --mount=type=cache,target=/app/target cargo build --release && cp target/release/news-api /news-api

# app image
FROM debian:buster-slim as news-api
RUN apt-get update && apt-get install -y libpq5
COPY --from=build /news-api /news-api
CMD ["/news-api"]
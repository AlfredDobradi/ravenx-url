FROM rust:bullseye AS build
LABEL authors="brvy"

WORKDIR /src
COPY . .
RUN cargo install --path .

FROM debian:bullseye

COPY --from=build /usr/local/cargo/bin/ravenx-url /usr/bin/ravenx

ENTRYPOINT ["ravenx"]
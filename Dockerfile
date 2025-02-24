FROM rust:bullseye
LABEL authors="brvy"

WORKDIR /src
COPY . .
RUN cargo install --path .
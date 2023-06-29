FROM rust:1.7 as builder

RUN USER=root cargo new --bin steinertree-web-demo
WORKDIR ./steinertree-web-demo
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release --features rebuild_c
RUN rm src/*.rs
RUN rm -rf src/geosteiner

ADD . ./

RUN rm ./target/release/deps/steinertree-web-demo*
RUN cargo build --release


FROM debian:buster-slim
ARG APP=/usr/src/app


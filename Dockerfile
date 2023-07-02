FROM rust:latest as build

RUN USER=root cargo new --bin steinertree-web-demo
WORKDIR /steinertree-web-demo

COPY ./Cargo.toml ./Cargo.toml
COPY ./build.rs ./build.rs
COPY ./src/ ./src/

# build the C libraries (just like with --features rebuild_c which doesn't work in docker)
RUN make -C ./src/geosteiner/ librs_geosteiner.la

RUN cargo build --release
RUN rm src/*.rs

FROM rust:latest

COPY --from=build /steinertree-web-demo/target/release/steinertree-web-demo ./steinertree-web-demo
COPY ./static/ ./static/

CMD ["/steinertree-web-demo"]

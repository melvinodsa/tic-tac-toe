# select image
FROM rust:1.23

RUN USER=root cargo new --bin tic-tac-toe
WORKDIR /tic-tac-toe

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs
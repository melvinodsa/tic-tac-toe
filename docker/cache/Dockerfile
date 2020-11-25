# select image
FROM rust:1.46

# copy your source tree
COPY ./ ./

# build for release
RUN cargo build --release
RUN rm src/*
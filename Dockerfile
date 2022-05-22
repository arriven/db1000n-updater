FROM rust:1.61 as builder
WORKDIR /usr/src/db1000n-updater
COPY . .
RUN cargo install --path .
CMD ["db1000n-updater"]
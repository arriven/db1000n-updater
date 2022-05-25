FROM rust:1.61 AS build
WORKDIR /usr/src/db1000n-updater
COPY . .
RUN cargo install --path .
CMD ["db1000n-updater"]
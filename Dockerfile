FROM rust:1.61 AS build
WORKDIR /usr/src/db1000n-updater
COPY . .
RUN cargo install --path .

FROM alpine
COPY --from=build /usr/local/cargo/bin/db1000n-updater /usr/local/bin/db1000n-updater
CMD ["db1000n-updater"]
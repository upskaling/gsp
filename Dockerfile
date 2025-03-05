FROM rust:1.85 AS build

COPY ./ ./

RUN cargo build --release

FROM debian:unstable-slim

COPY --from=build /target/release/gsp .

CMD ["./gsp"]

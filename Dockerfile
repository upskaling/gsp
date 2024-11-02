FROM rust:1.82 AS build

COPY ./ ./

RUN cargo build --release

FROM debian:unstable-slim

COPY --from=build /target/release/gsp .

CMD ["./gsp"]
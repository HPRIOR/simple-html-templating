FROM rust:1.58 as build

RUN mkdir /html_templating
WORKDIR /html_templating

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

RUN cargo build --release

FROM debian:buster-slim as final
COPY --from=build /html_templating/target/release/html_templating_bin .

CMD ["./html_templating_bin"]




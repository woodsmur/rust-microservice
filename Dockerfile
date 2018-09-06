FROM rustlang/rust:nightly AS build

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

# FROM debian:stretch AS package
FROM rust:1.28-stretch AS package
COPY --from=build /usr/src/app/target/release/rust-microservice ./

EXPOSE 8000
ENV ROCKET_ADDRESS="0.0.0.0"
ENV DATABASE_URL="postgres://diesel:diesel@localhost/rust_microservice"
CMD ["./rust-microservice"]
FROM rust:latest AS build

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

FROM alpine:3.20.3
COPY --from=build /usr/src/app/target/release/bmstu-rsoi-lab1 /
CMD ["/bmstu-rsoi-lab1"]


FROM rust:latest AS build

RUN mkdir /src
WORKDIR /src
COPY . .

RUN cargo build --release
RUN rm .gitignore

FROM debian:bookworm-slim
EXPOSE 3000
COPY --from=build /src/target/release/bmstu-rsoi-lab1 /bmstu-rsoi-lab1
RUN apt update && apt upgrade -y
RUN apt install -y libpq-dev
ENTRYPOINT ["/bin/sh", "-c", "./bmstu-rsoi-lab1"]


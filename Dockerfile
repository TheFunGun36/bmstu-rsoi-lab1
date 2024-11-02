FROM alpine:3.20.3
COPY target/release/bmstu-rsoi-lab1 /app
ENTRYPOINT ["/app"]


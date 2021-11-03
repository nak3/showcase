FROM rust as builder
WORKDIR /app
COPY . .
RUN cargo install trunk wasm-bindgen-cli
RUN rustup target add wasm32-unknown-unknown
RUN trunk build -d /tmp/dist

#FROM debian:buster-slim as runner
FROM python:3 as runner
COPY --from=builder /tmp/dist .
EXPOSE 8080
CMD ["python", "-m", "http.server", "8080"]

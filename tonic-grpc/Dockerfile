FROM rust as builder
WORKDIR /app
COPY . .
RUN rustup component add rustfmt
RUN cargo install --path .

FROM debian:buster-slim as runner
COPY --from=builder /usr/local/cargo/bin/tonic-grpc /usr/local/bin/tonic-grpc
EXPOSE 8080
CMD ["tonic-grpc"]

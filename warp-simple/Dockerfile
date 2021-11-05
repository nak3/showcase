FROM rust as builder
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM debian:buster-slim as runner
COPY --from=builder /usr/local/cargo/bin/warp-simple /usr/local/bin/warp-simple
EXPOSE 8080
CMD ["warp-simple"]

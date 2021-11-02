FROM rust as builder
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM debian:buster-slim as runner
COPY --from=builder /usr/local/cargo/bin/hyper-simple /usr/local/bin/hyper-simple
EXPOSE 8080
CMD ["hyper-simple"]

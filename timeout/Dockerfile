FROM rust as builder
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM debian:buster-slim as runner
COPY --from=builder /usr/local/cargo/bin/timeout /usr/local/bin/timeout
EXPOSE 8080
CMD ["timeout"]

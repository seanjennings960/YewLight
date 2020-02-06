FROM rust:1.39 as builder

RUN cargo install https

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl-dev

# Copy rust-friendly http server from builder image.
COPY --from=builder /usr/local/cargo/bin/http /usr/local/bin/http

# Copy the content to serve.
COPY ./dist /dist

EXPOSE 80

CMD ["http", "dist", "-p", "80"]

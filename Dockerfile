FROM rustlang/rust:nightly as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

# Installs the orm cli tool that generates migrations
RUN cargo install sea-orm-cli
RUN curl -L https://github.com/rust-analyzer/rust-analyzer/releases/latest/download/rust-analyzer-linux -o /rust-analyzer
RUN chmod 755 /rust-analyzer

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libpq-dev
COPY ./Rocket.toml ./Rocket.toml
COPY --from=builder /usr/local/cargo/bin/webserver /usr/local/bin/webserver
CMD ["webserver"]

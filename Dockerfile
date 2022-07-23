FROM rust:buster as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

# Installs the orm cli tool that generates migrations
# RUN cargo install sea-orm-cli
# RUN curl -L https://github.com/rust-analyzer/rust-analyzer/releases/latest/download/rust-analyzer-linux -o /rust-analyzer
# RUN chmod 755 /rust-analyzer

CMD ["cargo", "run"]

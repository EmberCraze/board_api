# Cargo build stage
FROM rust:buster as build-container

# Fake files for caching
WORKDIR /usr/src
RUN USER=root cargo new app
WORKDIR /usr/src/app
# Copy required libraries for our project
COPY Cargo.lock Cargo.toml Rocket.toml .
# Compile all required libraries for our project
# RUN cargo build --release
RUN cargo build

#Copy our actual code
COPY migrations .
COPY src src/
# Compile our actual code
RUN cargo build
# RUN cargo build --release \
#     && mv target/release/board_api /bin

# CMD ["/bin/board_api"]
CMD ["cargo", "run"]

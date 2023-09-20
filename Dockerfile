# Build stage
FROM rust:1.69-buster as builder

WORKDIR /app

# accept the build arguments
ARG DATABASE_URL
ARG HOST

ENV DATABASE_URL=$DATABASE_URL
ENV HOST=$HOST

# cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir ./src && echo 'fn main() { println!("Dummy!"); }' > ./src/main.rs
RUN cargo build --release

# copy the source and build the application
RUN rm -rf ./src
COPY ./src ./src

# touch to force cargo rebuild main.rs with the real project
# The last modified attribute of main.rs needs to be updated manually,
# otherwise cargo won't rebuild it.
RUN touch -a -m ./src/main.rs
RUN cargo build --release

# Production stage
FROM debian:buster-slim

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/rust-user-api .

# create group and user
RUN groupadd -r docker && useradd -g docker docker

# set permissions
RUN chown -R docker:docker /usr/local/bin/rust-user-api

# switch to non-root user
USER docker

# run the application
CMD ["./rust-user-api"]

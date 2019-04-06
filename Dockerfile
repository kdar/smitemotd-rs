FROM clux/muslrust

WORKDIR /
RUN USER=root cargo new smitemotd --bin

WORKDIR /smitemotd

COPY ./Cargo.toml .
COPY ./Cargo.lock .

# We cache rust build dependencies here
RUN USER=root cargo build --release
RUN rm src/*.rs

COPY ./build.rs .
COPY ./src ./src

RUN rm ./target/x86_64-unknown-linux-musl/release/deps/smitemotd*
RUN cargo build --release
RUN strip ./target/x86_64-unknown-linux-musl/release/smitemotd

FROM scratch
COPY --from=0 /smitemotd/target/x86_64-unknown-linux-musl/release/smitemotd /
COPY --from=0 /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
CMD ["./smitemotd"]
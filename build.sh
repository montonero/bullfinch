docker run -it --rm \
    -v $PWD:/workdir \
    -v ~/.cargo/git:/root/.cargo/git \
    -v ~/.cargo/registry:/root/.cargo/registry \
    registry.gitlab.com/rust_musl_docker/image:stable-latest \
    cargo build --release -vv --target=x86_64-unknown-linux-musl

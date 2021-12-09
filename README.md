# Rust: proof of concept
- Online playground: https://play.rust-lang.org/

## General
- Very fast
- Everything is immutable by default
- You can pick the number of bits/bytes to use for storing variables, unsigned, etc.
- There is no garbage collector. For variables allocated on the Heap:
    - Variable is destroyed once it gets out of scope
- Stack vs Heap
- The language is thread safe by default
- https://doc.rust-lang.org/rust-by-example/scope/borrow.html

## Hello world
- `cargo new hello`
- `cargo build`
- `cargo build --release`

## Rust with Docker
- https://hub.docker.com/_/rust
- https://dev.to/rogertorres/first-steps-with-docker-rust-30oi
- `docker build -t my-rust-app .`
- `docker run -it --rm --name my-running-app my-rust-app`
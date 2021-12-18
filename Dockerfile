FROM rust:1.56.1
RUN rustup component add rustfmt && \
  cargo install cargo-edit

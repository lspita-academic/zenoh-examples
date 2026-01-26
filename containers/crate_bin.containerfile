FROM rust:1.92.0-slim AS chef
RUN cargo install cargo-chef --locked
WORKDIR /app

# planner and builder in two different stages
# - recipe preparation depends on source code changes
# - recipe cooking depends on recipe changes

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json
COPY . .
ARG CRATES_ROOT=crates
ARG CRATE_NAME
WORKDIR ${CRATES_ROOT}/${CRATE_NAME}
RUN cargo build --bins

FROM debian:trixie-slim
ARG BIN_NAME
COPY --from=builder /app/target/debug/${BIN_NAME} /usr/local/bin/entrypoint
ENTRYPOINT [ "/usr/local/bin/entrypoint" ]

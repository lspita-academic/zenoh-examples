# Zenoh Examples

A collection of examples to try out [Zenoh](https://zenoh.io/)'s features.

## Requirements

Everything was tested using

- Rust 1.9.2
- Docker 29.1.5, Docker Compose 5.0.2
- Podman 5.7.0, Podman Compose 1.5.0

The usage of docker is highly suggested to better emulate LAN network communication and
to increase the scale of services at will.

## Structure

- [examples](./crates/examples/): all example crates
- [common](./crates/common/): common crates used by the examples

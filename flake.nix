{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    # https://wiki.nixos.org/wiki/Rust#Custom_Rust_version_or_targets
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            (import rust-overlay)
            (_: prev: {
              # https://docs.espressif.com/projects/rust/book/getting-started/toolchain.html#risc-v-devices
              rustToolchain = prev.rust-bin.stable.latest.default.override {
                extensions = [
                  "rust-src"
                  "rust-analyzer"
                ];
                targets = [
                  "riscv32imc-unknown-none-elf" # For ESP32-C2 and ESP32-C3
                ];
              };
            })
          ];
        };
      in
      {
        devShell =
          with pkgs;
          mkShell {
            buildInputs = [
              # nix
              nixd
              nil
              nixfmt
              # rust
              rustToolchain
              # toml
              tombi
              # esp32
              esp-generate
              espflash
              probe-rs-tools
            ];
            shellHook = ''
              set -a
              source .env 2> /dev/null
              PATH="$(realpath ./scripts/):$PATH"
              set +a

              chmod u+x ./scripts/*
            '';
          };
      }
    );
}

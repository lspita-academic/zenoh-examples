{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    # https://wiki.nixos.org/wiki/Rust#Custom_Rust_version_or_targets
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    # https://github.com/mirrexagon/nixpkgs-esp-dev
    esp-idf = {
      url = "github:mirrexagon/nixpkgs-esp-dev";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      esp-idf,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            (import rust-overlay)
            esp-idf.overlays.default
            (_: prev: {
              rustToolchain = prev.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
            })
          ];
          config = {
            permittedInsecurePackages = [
              "python3.13-ecdsa-0.19.1"
            ];
          };
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
              esp-idf-full
              libz
              libclang
              openssl
            ];
            shellHook = ''
              set -a
              source .env 2> /dev/null
              LD_LIBRARY_PATH="${
                lib.makeLibraryPath [
                  libz
                  libclang
                  openssl
                  stdenv.cc.cc
                ]
              }:$LD_LIBRARY_PATH"
              set +a
            '';
          };
      }
    );
}

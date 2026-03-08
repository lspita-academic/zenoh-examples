{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
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
          overlays = [ rust-overlay.overlays.default ];
        };
        rust-toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        # esp-clang ships as a pre-built binary linked against libxml2.so.2
        # (the old v2.x soname). Nix's current libxml2 uses libxml2.so.16.
        # This package creates a lib directory that has both the real .so.16
        # and a .so.2 symlink pointing at it, satisfying the dynamic linker.
        libxml2-16-path = "${pkgs.libxml2.out}/lib/libxml2.so.16";
        libxml2-2-links = pkgs.runCommand "libxml2-2" { } ''
          mkdir -p $out/lib
          # Real library
          ln -s ${libxml2-16-path} $out/lib/libxml2.so.16
          # Compatibility symlink expected by esp-clang
          ln -s ${libxml2-16-path} $out/lib/libxml2.so.2
          ln -s ${libxml2-16-path} $out/lib/libxml2.so
        '';
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
              rust-toolchain
              rust-analyzer
              # toml
              tombi
              # esp32 rust
              espflash
              python3
              ldproxy
              # esp-idf-sys runtime deps
              libxml2.out
              libxml2-2-links
              zlib
              stdenv.cc.cc.lib
            ];

            env = {
              LD_LIBRARY_PATH = lib.makeLibraryPath [
                libxml2-2-links
                zlib
                stdenv.cc.cc.lib
              ];
            };

            shellHook = ''
              set -a
              source .env 2> /dev/null
              set +a
            '';
          };
      }
    );
}

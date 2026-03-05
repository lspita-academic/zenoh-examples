{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
        # esp-clang ships as a pre-built binary linked against libxml2.so.2
        # (the old v2.x soname). Nix's current libxml2 uses libxml2.so.16.
        # This package creates a lib directory that has both the real .so.16
        # and a .so.2 symlink pointing at it, satisfying the dynamic linker.
        libxml2-16-path = "${pkgs.libxml2.out}/lib/libxml2.so.16";
        libxml2-2 = pkgs.runCommand "libxml2-2" { } ''
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
              # toml
              tombi
              # esp32 rust
              espflash
              espup
              rustup # needed to detect rust
              gcc
              cmake
              ninja
              gnumake
              perl
              curl
              pkg-config
              python3
              ldproxy
              # esp-idf-sys runtime deps
              libxml2.out
              libxml2-2
              zlib
              stdenv.cc.cc.lib
            ];

            env = {
              LD_LIBRARY_PATH = lib.makeLibraryPath [
                libxml2-2
                zlib
                stdenv.cc.cc.lib
              ];
            };

            shellHook = ''
              set -a
              TOOLCHAIN_DIR="$(pwd)/.toolchain"
              CARGO_HOME="$TOOLCHAIN_DIR/cargo"
              RUSTUP_HOME="$TOOLCHAIN_DIR/rustup"
              ESPUP_EXPORT_FILE="$TOOLCHAIN_DIR/export-esp.sh"
              ESP_CLANG_LINK="$HOME/.espup/esp-clang"
              source .env 2> /dev/null
              set +a

              if [ ! -f "$ESPUP_EXPORT_FILE" ]; then
                  [ -L "$ESP_CLANG_LINK" ] && rm "$ESP_CLANG_LINK"
                  mkdir -p "$RUSTUP_HOME"
                  espup install -f "$ESPUP_EXPORT_FILE"
              fi
              if [ -f "$ESPUP_EXPORT_FILE" ]; then
                  source "$ESPUP_EXPORT_FILE"
              fi
            '';
          };
      }
    );
}

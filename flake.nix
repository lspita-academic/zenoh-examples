{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    # https://github.com/oxalica/rust-overlay
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
        rpi-pkgs = pkgs.pkgsCross.raspberryPi;
        rust-toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        # esp-clang ships as a pre-built binary linked against libxml2.so.2
        # (the old v2.x soname). Nix's current libxml2 uses libxml2.so.16.
        # This package creates a lib directory that has both the real .so.16
        # and a .so.2 symlink pointing at it, satisfying the dynamic linker.
        # https://github.com/esp-rs/esp-idf-template/issues/282
        libxml2-16 = pkgs.libxml2.out;
        libxml2-2-links = pkgs.runCommand "libxml2-2-links" { } ''
          mkdir -p $out/lib
          ln -s "${libxml2-16}/lib/libxml2.so" $out/lib/libxml2.so.2
        '';
        embuild-libraries = with pkgs; [
          stdenv.cc.cc.lib
          libxml2-2-links
          zlib
          libclang
        ];
        rpi-libraries = with rpi-pkgs; [
          # stdenv.cc.cc.lib
        ];
        rpi-target-prefix = rpi-pkgs.stdenv.cc.targetPrefix;
        rpi-linker = pkgs.writeShellScriptBin "armv6l-unknown-linux-gnueabihf-cc-wrapper" ''
          # mkShell populates NIX_LDFLAGS with -L paths for every buildInput lib,
          # including host (x86_64) gcc-lib. The cc-wrapper forwards these to the
          # linker, which then finds x86_64 libgcc_s.so.1 and rejects it for ARM.
          # Unsetting these forces the cross-gcc to rely solely on its own built-in
          # sysroot paths, which correctly point to the ARM target libs.
          unset NIX_LDFLAGS
          unset NIX_LDFLAGS_FOR_TARGET
          unset LD_LIBRARY_PATH
          exec ${rpi-pkgs.stdenv.cc}/bin/${rpi-target-prefix}cc "$@"
        '';
      in
      {
        devShell =
          with pkgs;
          mkShell {
            buildInputs = lib.lists.flatten [
              # nix
              nixd
              nil
              nixfmt
              # rust
              rust-toolchain
              rust-analyzer
              # toml
              tombi
              # esp32
              espflash
              python313 # there is a warning in the logs about 3.14
              ldproxy
              libxml2-16
              embuild-libraries
              # rpi zero
              rpi-pkgs.stdenv.cc
              rpi-libraries
              rpi-linker
            ];

            env = {
              LD_LIBRARY_PATH = lib.makeLibraryPath (embuild-libraries ++ rpi-libraries);
              CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABIHF_LINKER = "${rpi-linker}/bin/${rpi-target-prefix}cc-wrapper";
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

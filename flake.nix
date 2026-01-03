{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
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
              rustc
              cargo
              rustfmt
              # toml
              tombi
            ];
            env = {
              RUST_SRC_PATH = "${rust.packages.stable.rustPlatform.rustLibSrc}";
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

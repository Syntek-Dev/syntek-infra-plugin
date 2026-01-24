{
  description = "syntek-infra-tool - CLI for NixOS and Wireguard detection";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "syntek-infra-tool";
          version = "0.1.0";
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          buildInputs = with pkgs; [
            openssl
          ];

          meta = with pkgs.lib; {
            description = "CLI tool for NixOS and Wireguard environment detection";
            homepage = "https://github.com/Syntek-Studio/syntek-infra-plugin";
            license = licenses.mit;
          };
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            pkg-config
            openssl

            # For testing
            wireguard-tools
          ];

          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
        };
      }
    );
}

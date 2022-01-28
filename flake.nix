{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";

    nixpkgs.url = "nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, fenix }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        rust = with fenix.packages.${system}; rec {
          native = latest;
          dev.toolchain = combine [ native.toolchain rust-analyzer ];
        };
      in
      {
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            # Rust
            rust.dev.toolchain
            mold
          ];

          # redirect ld calls to mold
          MOLD_PATH = "${pkgs.mold}/bin/mold";
          LD_PRELOAD = "${pkgs.mold}/lib/mold/mold-wrapper.so";
        };
      }
    );
}

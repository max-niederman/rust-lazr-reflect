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
            rust.dev.toolchain
            mold
            cmake
          ];

          LD_LIBRARY_PATH = nixpkgs.lib.strings.makeLibraryPath
            (with pkgs; [
              xorg.libX11
              xorg.libXcursor
              # xorg.libXrandr
              libxkbcommon
            ]);

          # redirect ld calls to mold
          MOLD_PATH = "${pkgs.mold}/bin/mold";
          LD_PRELOAD = "${pkgs.mold}/lib/mold/mold-wrapper.so";

          # required for minifb crate
          XKBCOMMON_LIB_DIR = "${pkgs.libxkbcommon}/lib";
        };
      }
    );
}

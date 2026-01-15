{
  description = "librustc - Universal dynamic rustc loader";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        
        librustc = pkgs.callPackage ./default.nix {};
      in {
        packages.default = librustc;
        
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" "rustfmt" "clippy" ];
            })
            cargo
            rustc
            pkg-config
          ];
          
          shellHook = ''
            export RUSTC_DRIVER_SO=$(find ${pkgs.rustc}/lib -name "librustc_driver-*.so" | head -1)
            echo "RUSTC_DRIVER_SO=$RUSTC_DRIVER_SO"
          '';
        };
      }
    );
}

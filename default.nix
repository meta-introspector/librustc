{ pkgs ? import <nixpkgs> {} }:

pkgs.rustPlatform.buildRustPackage {
  pname = "librustc";
  version = "0.1.0";

  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  buildInputs = with pkgs; [
    # libloading is a Rust crate, not a system dependency
  ];

  meta = with pkgs.lib; {
    description = "Universal dynamic rustc loader";
    license = with licenses; [ mit asl20 ];
  };
}

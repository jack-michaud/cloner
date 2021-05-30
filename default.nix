{ pkgs ? import <nixpkgs> {} }:
with pkgs;
rustPlatform.buildRustPackage {
  name = "cloner";
  version = "0.0.1";
  src = ./.;
  cargoHash = "sha256-dPkeDaITJlDuzdih4Ax8mk/Pv1KKsOMkT6Kf7KXuB3w=";
}

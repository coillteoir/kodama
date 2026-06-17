{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {

  buildInputs = [
    pkgs.cargo
    pkgs.cargo-fmt
    pkgs.clippy
  ];

}

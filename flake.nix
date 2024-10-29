{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = [
        pkgs.vim
        pkgs.cargo
        pkgs.clippy
        pkgs.rustfmt
        pkgs.f3d
        pkgs.git
      ];
    };
    packages.x86_64-linux.default = pkgs.rustPlatform.buildRustPackage rec {
      name = "kodama";
      src = ./.;

      cargoHash = "sha256-U9Un9x9EfIrJ2Zmem935SIes3KF2Aq8eip5u4PkBWFI=";
    };
  };
}

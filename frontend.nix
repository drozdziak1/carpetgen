{ sources ? import ./nix/sources.nix, pkgs ? import sources.nixpkgs {} }:
let
  src = ./.;
  yarn2nix = pkgs.callPackage sources.yarn2nix {};
in
yarn2nix.mkYarnPackage {
  name = "carpetgen-frontend";
  inherit src;
  packageJSON = ./package.json;
  yarnLock = ./yarn.lock;
}

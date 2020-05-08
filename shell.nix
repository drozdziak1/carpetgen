let
  sources = import ./nix/sources.nix;
  pkgs = import sources.nixpkgs {};
  rust = import ./nix/rust.nix { inherit sources; };
  deps = import ./common_deps.nix;
in
pkgs.mkShell {
  buildInputs = [
    rust
    pkgs.yarn2nix
  ] ++ deps { inherit pkgs; };
}

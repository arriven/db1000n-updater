# default.nix
with import <nixpkgs> {};
stdenv.mkDerivation {
    name = "dev-environment";
}
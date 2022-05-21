# default.nix
with import <nixpkgs> {};
stdenv.mkDerivation {
    name = "dev-environment";
    buildInputs = [ openssl ];
    shellHook = ''
        export OPENSSL_DIR="${openssl.dev}"
        export OPENSSL_LIB_DIR="${openssl.out}/lib"
    '';
}
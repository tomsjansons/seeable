# default.nix
with import <nixpkgs> {};
stdenv.mkDerivation {
    name = "seeable-local-dev"; # Probably put a more meaningful name here
    buildInputs = [ pkg-config openssl nodejs_21 corepack_21 steam-run ];
    RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}

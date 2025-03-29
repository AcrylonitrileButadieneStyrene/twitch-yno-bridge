nix develop .flake --command cargo build --release
nix run nixpkgs#patchelf -- --set-interpreter "$(nix eval nixpkgs#stdenv.cc.bintools.dynamicLinker --raw)" ./target/release/controller
{
  pkgs ? import <nixpkgs> { },
}:

with pkgs;

let
  rustup-toolchain = rust-bin.fromRustupToolchainFile ./rustup-toolchain.toml;
in
mkShell {
  packages = [
    nodejs_24
    pnpm_9
    rustup-toolchain
  ];
}

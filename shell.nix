{ pkgs ? (import <nixpkgs> {
  config.allowUnfree = true;
}), ... }:

pkgs.mkShell {
  buildInputs = [
    pkgs.gcc
    pkgs.rustup
    pkgs.vscode
  ];
}

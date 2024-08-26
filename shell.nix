{ pkgs ? (import <nixpkgs> {
  config.allowUnfree = true;
}), ... }:

pkgs.mkShell {
  buildInputs = [
    pkgs.rustc
    pkgs.cargo
    pkgs.gcc
    pkgs.rustup
    pkgs.vscode
  ];
 # allowUnfree = true;
}

{ pkgs ? (import <nixpkgs> { 
    config.allowUnfree = true;
}), ... }:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    rustup 
    rust-analyzer
    python3
    google-chrome
    trunk
  ];
}

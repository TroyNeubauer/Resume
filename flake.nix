{
  description = "Rust flake";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            (import rust-overlay)
          ];
          # Allow Google Chrome
          config.allowUnfree = true;
        };
        rust-pkgs = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-analyzer" "rust-src" ];
          targets = [ "wasm32-unknown-unknown" ];
        };
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rust-pkgs;
          rustc = rust-pkgs;
        };
        # chrome-pkg = if pkgs.stdenv.isLinux then pkgs.ungoogled-chromium else if pkgs.stdenv.isDarwin then pkgs.google-chrome else [];
        # chrome = if pkgs.stdenv.isLinux then "${chrome-pkg}/bin/chromium" else if pkgs.stdenv.isDarwin then "${chrome-pkg}/bin/google-chrome" else [];
        wasm = import ./wasm.nix {
          inherit rustPlatform;
          inherit (pkgs) trunk wasm-bindgen-cli;
        };
        generate-pdf = import ./generate-pdf.nix {
          inherit (pkgs) writeShellScriptBin google-chrome python3;
          # inherit wasm;
          wasm = "/nix/store/cyfjfy1rjfqif37wd570i4jalv7c8bg0-wasm-0.1.0";
        };
        deploy = import ./deploy.nix {
          inherit (pkgs) writeShellScriptBin yq;
          wasm = "/nix/store/cyfjfy1rjfqif37wd570i4jalv7c8bg0-wasm-0.1.0";
        };
        
      in {
        packages.wasm = wasm;
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            rust-pkgs
            google-chrome
            trunk
            generate-pdf
            deploy
          ];
        };
      });
}

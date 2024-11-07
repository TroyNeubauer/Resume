{ rustPlatform, trunk, wasm-bindgen-cli }:
rustPlatform.buildRustPackage {
  pname = "wasm";
  version = "0.1.0";
  src = ./.;
  cargoLock = { lockFile = ./Cargo.lock; };

  nativeBuildInputs = [ trunk wasm-bindgen-cli ];

  buildPhase = ''
    echo $src
    trunk --skip-version-check build --offline --release
  '';

  installPhase = ''
    mkdir -p $out/resume
    cp dist/* $out/resume
  '';
}

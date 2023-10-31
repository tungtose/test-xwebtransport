{
  description = "rust shell dev";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShells.default = (mkShell.override { stdenv = clangStdenv; }) rec {
          nativeBuildInputs = [
            makeWrapper
            pkg-config
          ];

          buildInputs = [
            just
            rust-analyzer
            libiconv
            llvmPackages.libclang
            llvmPackages.libcxxClang
            wasm-pack
            gcc
            cmake
            clang
            clang-tools
            udev
            alsa-lib
            vulkan-loader
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr # To use the x11 feature
            libxkbcommon
            binaryen

            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" ];
              targets = [ "wasm32-unknown-unknown" ];
            })
          ];

          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";

          env = {
            ZSTD_SYS_USE_PKG_CONFIG = true;
          };

          LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";

          shellHook = ''
            alias run="cargo run"
            alias ls=exa
            alias find=fd
            alias build="cargo build"
            alias web="cargo run --release --target wasm32-unknown-unknown"
          '';
        };
      }
    );
}

{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShells.default = pkgs.mkShell {
          #LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
          #BINDGEN_EXTRA_CLANG_ARGS = "-isystem ${pkgs.libclang.lib}/lib/clang/${flake-utils.lib.getVersion pkgs.clang}/include";

          buildInputs = with pkgs; [
            rust-bin.stable.latest.default
            rust-bin.stable.latest.rustfmt
            rust-bin.stable.latest.clippy
          ];
          nativeBuildInputs = [
              pkgs.pkg-config
              pkgs.cmake
              pkgs.vcpkg
          ];
            packages = with pkgs; [
                rust-analyzer
                pkg-config
                opencv
                alsa-lib
                systemdLibs
                cmake
                fontconfig
                linuxHeaders
                rustPlatform.bindgenHook
                llvmPackages.libclang.lib
                llvmPackages.clang
                v4l-utils
                libv4l
            ];
            LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
            shellHook = ''
              export LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
              cargo version
            '';

        };
      }
    );
}

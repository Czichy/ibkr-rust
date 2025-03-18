{inputs, ...}: {
  perSystem = {
    config,
    pkgs,
    system,
    inputs',
    self',
    ...
  }:
    with inputs; let
      inherit (self'.packages) rust-toolchain;
      inherit (self'.legacyPackages) cargoExtraPackages ciPackages;

      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          (import rust-overlay)
          (final: prev: {
            lpi = inputs.lpi.packages.${system}.default;
          })
        ];
      };

      devTools = with pkgs; [
        # rust tooling
        rust-toolchain
        cargo-audit
        cargo-udeps
        cargo-limit
        bacon
        cargo-watch
        cargo-audit
        cargo-deny
        # cargo-llvm-cov
        cargo-tarpaulin
        cargo-nextest
        cargo-outdated
        # formatting
        self'.packages.treefmt
        taplo
        # misc
        lpi
        # logging
        bunyan-rs.out
        # command runner
        just
        nushell
      ];
    in {
      devShells = {
        default = pkgs.mkShell {
          name = "IBKR-shell";
          RUST_SRC_PATH = "${self'.packages.rust-toolchain}/lib/rustlib/src/rust/src";
          AMD_VULKAN_ICD = "RADV";

          LD_LIBRARY_PATH = "$LD_LIBRARY_PATH:${with pkgs;
            lib.makeLibraryPath [
              udev
              alsa-lib
              vulkan-loader
              libxkbcommon
              openssl
              wayland # To use wayland feature
            ]}";
          packages = devTools ++ cargoExtraPackages ++ ciPackages;

          # shellHook = ''
          # export LD_LIBRARY_PATH="${
          #   lib.makeLibraryPath libraries
          # }:$LD_LIBRARY_PATH"

          # export OPENSSL_INCLUDE_DIR="${openssl.dev}/include/openssl"

          # export OPENSSL_LIB_DIR="${openssl.out}/lib"

          # export OPENSSL_ROOT_DIR="${openssl.out}"

          # export RUST_SRC_PATH="${toolchain}/lib/rustlib/src/rust/library"
          # export EDITOR=hx
          # '';
        };
      };
    };
}

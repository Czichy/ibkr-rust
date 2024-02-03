{
  fenix-toolchain,
  fenix-channel,
  cargoArgs,
  commonArgs,
  unitTestArgs,
  pkgs,
  writeShellScriptBin,
  ...
}: let
  cargo-ext = pkgs.callPackage ./cargo-ext.nix {inherit cargoArgs unitTestArgs;};
in
  pkgs.mkShell rec {
    name = "Seeking-Edge-shell";

    buildInputs = commonArgs.buildInputs;
    nativeBuildInputs = commonArgs.nativeBuildInputs ++ (with pkgs; [
      cargo-ext.cargo-build-all
      cargo-ext.cargo-clippy-all
      cargo-ext.cargo-doc-all
      cargo-ext.cargo-nextest-all
      cargo-ext.cargo-test-all
      cargo-ext.cargo-udeps-all
      cargo-ext.cargo-watch-all
      cargo-nextest
      cargo-udeps
      cargo-watch
      cargo-audit
      # cargo-lichking
      fenix-toolchain
      bacon
      bunyan-rs.out
      just

      nixpkgs-fmt
      shellcheck
      rnix-lsp
      nodePackages.bash-language-server
    ]);
    RUST_SRC_PATH = "${fenix-channel.rust-src}/lib/rustlib/src/rust/library";
    AMD_VULKAN_ICD = "RADV";

    LD_LIBRARY_PATH = "$LD_LIBRARY_PATH:${with pkgs;
      lib.makeLibraryPath [
        udev
        alsa-lib
        vulkan-loader
        libxkbcommon
        openssl
        # wayland # To use wayland feature
      ]}";

    # shellHook = ''
    #   export NIX_PATH="nixpkgs=${pkgs.path}"
    #   export PATH=$PWD/dev-support/bin:$PATH
    # '';
  }

{inputs, ...}: {
  perSystem = {
    config,
    system,
    inputs',
    self',
    lib,
    ...
  }:
    with inputs; let
      manifest = (pkgs.lib.importTOML ../Cargo.toml).workspace.package;
      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          (import rust-overlay)
          (final: prev: {
            lpi = inputs.lpi.packages.${system}.default;
          })
        ];
      };
      fenix-channel = fenix.packages.${system}.latest;

      fenix-toolchain = fenix-channel.withComponents [
        "rustc"
        "cargo"
        "clippy"
        "rust-src"
        "llvm-tools-preview"
      ];

      craneLib = (crane.mkLib pkgs).overrideToolchain fenix-toolchain;

      common-build-args = {
        src = lib.cleanSourceWith {
          src = ../.;
          filter = path: type: (lib.hasSuffix ".wgsl" path) || (craneLib.filterCargoSources path type);
        };
        # src = filterWorkspaceFiles ../.;
        strictDeps = true;
        extraPackages = [
          pkgs.pkg-config
        ];
        bevyDependencies = with pkgs; [
          llvmPackages.bintools
          udev
          alsa-lib
          vulkan-loader
          xorg.libX11
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
          libxkbcommon
          wayland
          clang
        ];
        nativeBuildInputs = with pkgs;
          [
            openssl
          ]
          ++ common-build-args.extraPackages
          ++ common-build-args.bevyDependencies;
        buildInputs = with pkgs; [
          # clang
          mold
          lld
          flatbuffers

          amdvlk
          atk
          glib
          glibc
          gtk3
          libxkbcommon
          openssl
          vulkan-headers
          vulkan-tools
          vulkan-validation-layers
        ];
      };

      # filter source code at path `src` to include only the list of `modules`
      filterModules = modules: src: let
        basePath = toString src + "/";
      in
        lib.cleanSourceWith {
          filter = path: type: let
            relPath = lib.removePrefix basePath (toString path);
            includePath =
              (type
                == "directory"
                && builtins.match "^[^/]+$" relPath != null)
              || lib.any (re: builtins.match re relPath != null)
              (["Cargo.lock" "Cargo.toml" ".*/Cargo.toml"]
                ++ builtins.concatLists
                (map (name: [name "${name}/.*"]) modules));
            # uncomment to debug:
          in
            # builtins.trace "${relPath}: ${lib.boolToString includePath}"
            includePath;
          inherit src;
        };

      # Filter only files needed to build project dependencies
      #
      # To get good build times it's vitally important to not have to
      # rebuild derivation needlessly. The way Nix caches things
      # is very simple: if any input file changed, derivation needs to
      # be rebuild.
      #
      # For this reason this filter function strips the `src` from
      # any files that are not relevant to the build.
      #
      # Lile `filterWorkspaceFiles` but doesn't even need *.rs files
      # (because they are not used for building dependencies)
      filterWorkspaceDepsBuildFiles = src:
        filterSrcWithRegexes ["Cargo.lock" "Cargo.toml" ".*/Cargo.toml"]
        src;

      # Filter only files relevant to building the workspace
      filterWorkspaceFiles = src:
        filterSrcWithRegexes [
          "Cargo.lock"
          "Cargo.toml"
          "crates"
          ".cargo"
          ".cargo/.*"
          ".*/Cargo.toml"
          ".*\.rs"
          ".*.rs"
          "*.rs"
          ".*/rc/doc/.*.md"
          ".*.txt"
          ".*.json"
          ".*.otf"
          ".*.png"
          ".*.jpg"
        ]
        src;

      filterSrcWithRegexes = regexes: src: let
        basePath = toString src + "/";
      in
        lib.cleanSourceWith {
          filter = path: type: let
            relPath = lib.removePrefix basePath (toString path);
            includePath =
              (type == "directory")
              || lib.any (re: builtins.match re relPath != null) regexes;
            # uncomment to debug:
          in
            # builtins.trace "${relPath}: ${lib.boolToString includePath}"
            includePath;
          inherit src;
        };

      workspaceDeps = craneLib.buildDepsOnly (common-build-args
        // {
          src = filterWorkspaceDepsBuildFiles ../.;
          pname = "workspace-deps";
          version = manifest.version;
          buildPhaseCargoCommand = "cargo doc && cargo check --profile release --all-targets && cargo build --profile release --all-targets";
          doCheck = false;
        });

      # a function to define cargo&nix package, listing
      # all the dependencies (as dir) to help limit the
      # amount of things that need to rebuild when some
      # file change
      pkg = {
        name ? null,
        dir,
        extraDirs ? [],
      }: {
        package = craneLib.buildPackage (common-build-args
          // {
            cargoArtifacts = workspaceDeps;

            # src = filterModules ([dir] ++ extraDirs) ../.;
            # filter the source to reduce cache misses
            # add a path here if you need other files, e.g. bc of `include_str!()`
            src = nix-filter {
              root = ../.;
              include = [
                (nix-filter.lib.matchExt "toml")
                "Cargo.lock"
                "crates"
              ];
            };

            # if needed we will check the whole workspace at once with `workspaceBuild`
            doCheck = false;
          }
          // lib.optionalAttrs (name != null) {
            pname = name;
            version = manifest.version;
            cargoExtraArgs = "--bin ${name}";
          });
      };

      workspaceBuild = craneLib.cargoBuild (common-build-args
        // {
          pname = "workspace-build";
          version = manifest.version;
          cargoArtifacts = workspaceDeps;
          doCheck = false;
        });

      workspaceTest = craneLib.cargoBuild (common-build-args
        // {
          pname = "workspace-test";
          cargoArtifacts = workspaceBuild;
          doCheck = true;
        });

      # Note: can't use `cargoClippy` because it implies `--all-targets`, while
      # we can't build benches on stable
      # See: https://github.com/ipetkov/crane/issues/64
      workspaceClippy = craneLib.cargoBuild (common-build-args
        // {
          pname = "workspace-clippy";
          cargoArtifacts = workspaceBuild;

          cargoBuildCommand = "cargo clippy --profile release --no-deps --lib --bins --tests --examples --workspace -- --deny warnings";
          doInstallCargoArtifacts = false;
          doCheck = false;
        });

      workspaceDoc = craneLib.cargoBuild (common-build-args
        // {
          pname = "workspace-doc";
          cargoArtifacts = workspaceBuild;
          cargoBuildCommand = "env RUSTDOCFLAGS='-D rustdoc::broken_intra_doc_links' cargo doc --no-deps --document-private-items && cp -a target/doc $out";
          doCheck = false;
        });
      api = craneLib.buildPackage (individualCrateArgs
        // rec {
          pname = manifest.name;
          version = manifest.version;
          cargoExtraArgs = "--lib ${pname}";
          src = fileSetForCrate [
            "crates/api/src"
            "crates/api/Cargo.toml"
          ];
        });

      flex = craneLib.buildPackage (individualCrateArgs
        // rec {
          pname = "ibkr-rust-flex";
          cargoExtraArgs = "--bin ${pname}";
          src = fileSetForCrate [
            "crates/flex/src"
            "crates/flex/Cargo.toml"
          ];
        });
      # ibkr = pkg {
      #   name = manifest.name;
      #   dir = "../crates";
      #   extraDirs = [
      #     # "crates/seeking-edge"
      #     "crates/api"
      #     "crates/flex"
      #   ];
      # };
    in {
      packages = {
        inherit flex api;
        inherit (self.checks.${system}) coverage;
        default = self.packages.${system}.flex;
      };
      # packages = {
      #   default = ibkr.package;
      #   ibkr = ibkr.package;
      #   # inherit (self.checks.${system}) coverage;
      #   deps = workspaceDeps;
      #   workspaceBuild = workspaceBuild;
      #   workspaceClippy = workspaceClippy;
      #   workspaceTest = workspaceTest;
      #   workspaceDoc = workspaceDoc;
      #   # container = {seeking-edge = seeking-edge.container;};
      # };
      legacyPackages = {
        cargoExtraPackages = common-build-args.nativeBuildInputs;
        bevyDependencies = bevyDependencies;
      };

      formatter = pkgs.alejandra;
    };
}

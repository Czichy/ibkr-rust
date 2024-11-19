{inputs, ...}: {
  perSystem = {
    config,
    # pkgs,
    system,
    inputs',
    self',
    lib,
    ...
  }:
    with inputs; let
      rustToolchain = fenix.packages.${system}.fromToolchainFile {
        file = ../rust-toolchain.toml;
        sha256 = "sha256-dbdRPCQQnuQ66Ie8CmgGutRLK61weyYlAhcERrz2koE=";
      };

      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          (import rust-overlay)
          (final: prev: {
            lpi = inputs.lpi.packages.${system}.default;
          })
        ];
      };

      craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

      src = nix-filter.lib {
        root = ../.;
        include = [
          ../Cargo.toml
          ../Cargo.lock
          ../taplo.toml
          ../rustfmt.toml
          ../rust-toolchain.toml
          ../.config
          ../api
          ../flex
        ];
      };

      inherit (craneLib.crateNameFromCargoToml {inherit src;}) pname version;

      args = {
        inherit src;
        strictDeps = true;
        nativeBuildInputs = with pkgs; [
          alsa-lib
          libxkbcommon
          openssl
          pkg-config
          udev
        ];
        buildInputs = with pkgs; [
          clang
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
          udev
        ];
      };

      individualCrateArgs =
        args
        // {
          inherit cargoArtifacts version;
          doCheck = false;
        };

      fileSetForCrate = crateFiles:
        nix-filter.lib {
          root = ../.;
          include =
            [
              ../Cargo.toml
              ../Cargo.lock
            ]
            ++ crateFiles;
        };

      cargoArtifacts = craneLib.buildDepsOnly args;

      api = craneLib.buildPackage (individualCrateArgs
        // rec {
          pname = "api";
          cargoExtraArgs = "-p ${pname}";
          src = fileSetForCrate [
            ../crates/api/src
            ../crates/api/Cargo.toml
          ];
        });

      flex = craneLib.buildPackage (individualCrateArgs
        // rec {
          pname = "api";
          cargoExtraArgs = "-p ${pname}";
          src = fileSetForCrate [
            ../crates/flex/src
            ../crates/flex/Cargo.toml
          ];
        });

      # server = craneLib.buildPackage (individualCrateArgs
      #   // rec {
      #     pname = "server";
      #     cargoExtraArgs = "-p ${pname}";
      #     src = fileSetForCrate [
      #       ../crates/api
      #       ../crates/server/src
      #       ../crates/server/templates
      #       ../crates/server/styles
      #       ../crates/server/Cargo.toml
      #     ];
      #   });

      seeking-edge = craneLib.buildPackage (individualCrateArgs
        // rec {
          pname = "seeking-edge";
          cargoExtraArgs = "-p ${pname}";
          src = fileSetForCrate [
            ../crates/api
            ../crates/server/src
            ../crates/server/Cargo.toml
          ];
        });
      # app = pkgs.writeShellScriptBin pname ''
      #   WEBSERVER_ASSETS=${assets}/assets ${kickbase}/bin/kickbase
      # '';
      # postmanerator-theme = pkgs.stdenv.mkDerivation {
      #   name = "postmanerator-theme";
      #   src = pkgs.fetchFromGitHub {
      #     owner = "aubm";
      #     repo = "postmanerator-default-theme";
      #     rev = "c4ffa9d6b8973d8d71897e03d2f92a6b775b0cae";
      #     hash = "sha256-5EjjFXTuai79h7IjCNfCy9mJCmtg98K8ZlpTjDa6ro4=";
      #   };
      #   installPhase = ''
      #     mkdir -p $out/themes
      #     cp -r $src $out/themes/default
      #   '';
      # };
      # seeking-edge-api-doc = pkgs.stdenv.mkDerivation rec {
      #   POSTMANERATOR_PATH = postmanerator-theme;
      #   name = "kickbase-api-doc";
      #   pname = name;
      #   src = ../assets/.;
      #   buildPhase = ''
      #     ${pkgs.postmanerator}/bin/postmanerator \
      #       -collection=kickbase.postman_collection.json \
      #       -environment=kickbase.postman_environment.json \
      #       -output=../index.html
      #   '';
      #   installPhase = ''
      #     mkdir -p $out/share
      #     cp -r index.html $out/share
      #   '';
      # };
    in {
      checks = {
        inherit seeking-edge;
        # inherit app api server kickbase assets kickbase-api-doc;
        inherit (self.packages.${system}) services;

        clippy = craneLib.cargoClippy (args
          // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });

        doc = craneLib.cargoDoc (args
          // {
            inherit cargoArtifacts;
          });

        fmt = craneLib.cargoFmt {
          inherit src;
        };

        toml-fmt = craneLib.taploFmt {
          src = pkgs.lib.sources.sourceFilesBySuffices src [".toml"];
          taploExtraArgs = "--config ../taplo.toml";
        };

        audit = craneLib.cargoAudit {
          inherit src advisory-db;
        };

        deny = craneLib.cargoDeny {
          inherit src;
        };

        nextest = craneLib.cargoNextest (args
          // {
            inherit cargoArtifacts;
            partitions = 1;
            partitionType = "count";
          });
      };

      packages = {
        inherit flex api;
        inherit (self.checks.${system}) coverage;
        default = self.packages.${system}.flex;
      };
      legacyPackages = {
        cargoExtraPackages = args.nativeBuildInputs;
      };

      apps = {
        default = {
          program = self.packages.${system}.flex;
        };
      };

      formatter = pkgs.alejandra;
    };
}

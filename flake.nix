{
  description = "Environment to package a tauri app.";

  inputs.nixpkgs.url = "github:nixos/nixpkgs/master";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";
  inputs.tauri-driver.url = "github:tauri-apps/tauri";
  inputs.tauri-driver.flake = false;

  outputs =
    { nixpkgs, rust-overlay, flake-utils, tauri-driver, ... }:
    flake-utils.lib.eachSystem
      [ "x86_64-linux" ]
      (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            (import rust-overlay)
          ];
        };
        node = pkgs.nodejs-16_x;

        # hippity hoppitied from https://github.com/concrete-utopia/utopia/blob/master/shell.nix
        pnpmPkg = pkgs.nodePackages_latest.pnpm.override {
          nativeBuildInputs = [ pkgs.makeWrapper ];
          preRebuild = ''
            sed 's/"link:/"file:/g' --in-place package.json
          '';
          postInstall =
            let
              pnpmLibPath = pkgs.lib.makeBinPath [
                node.passthru.python
                node
              ];
            in
            ''
              for prog in $out/lib/node_modules/pnpm/bin/*; do
                wrapProgram "$prog" --prefix PATH : ${pnpmLibPath}
              done
            '';
        };
        pnpm = "${pnpmPkg}/lib/node_modules/pnpm/bin/pnpm.cjs";
        pnpx = "${pnpmPkg}/lib/node_modules/pnpm/bin/pnpx.cjs";
      in
      rec {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            rust-bin.stable.latest.default
            binutils
            zlib
            wget
            curl
            openssl
            libressl
            glib-networking
            dbus
            squashfsTools
            pkg-config
            libsoup

            webkitgtk
            gtk3-x11
            gtksourceview
            libayatana-appindicator-gtk3

            (pkgs.stdenv.mkDerivation {
              name = "scripts";
              phases = "installPhase";
              installPhase = ''
                mkdir -p $out/bin
                ln -s ${pnpm} $out/bin/pnpm
                ln -s ${pnpx} $out/bin/pnpx
              '';
            })
            node
            nodePackages.prisma

            (rustPlatform.buildRustPackage
              rec {
                pname = "tauri-driver";
                version = "0.1.2";

                src = tauri-driver + "/tooling/webdriver";
                cargoSha256 = "TYEGpZxtSBpEoh5qTRLd8QPWlzAzteA787fNLx+zoBM=";
              })

            # Language servers
            rnix-lsp
            rust-analyzer
            nodePackages.svelte-language-server
            nodePackages.typescript-language-server
            nodePackages.eslint_d
          ];

          shellHook = ''
            export OPENSSL_DIR="${pkgs.openssl.dev}"
            export OPENSSL_LIB_DIR="${pkgs.openssl.out}/lib"

            export PRISMA_MIGRATION_ENGINE_BINARY="${pkgs.prisma-engines}/bin/migration-engine"
            export PRISMA_QUERY_ENGINE_BINARY="${pkgs.prisma-engines}/bin/query-engine"
            export PRISMA_QUERY_ENGINE_LIBRARY="${pkgs.prisma-engines}/lib/libquery_engine.node"
            export PRISMA_INTROSPECTION_ENGINE_BINARY="${pkgs.prisma-engines}/bin/introspection-engine"
            export PRISMA_FMT_BINARY="${pkgs.prisma-engines}/bin/prisma-fmt"

            export WEBKIT_DISABLE_COMPOSITING_MODE=1
            export GIO_MODULE_DIR=${pkgs.glib-networking}/lib/gio/modules/
            export PATH=".extra:$PATH"
          '';
        };
      });
}

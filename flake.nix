{
  description = "Environment to package a tauri app.";

  inputs.nixpkgs.url = "github:nixos/nixpkgs/master";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";

  outputs =
    { nixpkgs, rust-overlay, flake-utils, ... }:
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
            dbus
            squashfsTools
            pkg-config
            libsoup

            webkitgtk
            gtk3-x11
            gtksourceview
            libayatana-appindicator-gtk3
            yarn
            nodejs

            # Language servers
            rnix-lsp
            rust-analyzer
            nodePackages.svelte-language-server
            nodePackages.typescript-language-server
            vscode-extensions.dbaeumer.vscode-eslint
          ];

          shellHook = ''
            export OPENSSL_DIR="${pkgs.openssl.dev}"
            export OPENSSL_LIB_DIR="${pkgs.openssl.out}/lib"
            export WEBKIT_DISABLE_COMPOSITING_MODE=1
          '';
        };
      });
}

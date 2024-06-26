{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
      };

      rustTC = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

      libraries = with pkgs; [
        openssl_3
        pkg-config
        libGL
        mesa_drivers
        glfw-wayland
        libglvnd
        webkitgtk_4_1
        gtk3
        cairo
        gdk-pixbuf
        glib
        dbus
        librsvg
      ];

      packages = with pkgs; [
        # binaries
        nodejs
        corepack
        rustTC
        cargo-watch
        cargo-binstall # just in case
        playwright-driver.browsers
        temurin-jre-bin-21

        # lsps
        nil
        nodePackages.svelte-language-server
        nodePackages.typescript-language-server
        marksman
        alejandra
      ];
    in {
      devShell = pkgs.mkShell {
        buildInputs = packages;
        nativeBuildInputs = libraries;

        PLAYWRIGHT_BROWSERS_PATH = pkgs.playwright-driver.browsers;
        PLAYWRIGHT_SKIP_VALIDATE_HOST_REQUIREMENTS = true;
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath libraries;
        PKG_CONFIG_PATH = pkgs.lib.makeLibraryPath libraries;
        RUST_SRC_PATH = "${rustTC}/lib/rustlib/src/rust/library/";
      };
    });
}

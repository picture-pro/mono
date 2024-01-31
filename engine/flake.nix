{
  inputs = {
    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0.2311.555002.tar.gz";
    rust-overlay.url = "https://flakehub.com/f/oxalica/rust-overlay/0.1.1271.tar.gz";
    crane.url = "https://flakehub.com/f/ipetkov/crane/0.16.1.tar.gz";
  };

  outputs = { self, flake-utils, crane, nixpkgs, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = (import nixpkgs) {
          inherit system overlays;
          config.allowUnfree = true;
        };
        
        toolchain = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        });
        
        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

        common_args = {
          src = craneLib.cleanCargoSource (craneLib.path ./.);
          doCheck = false;
          pname = "engine";
          version = "0.1.0";

          nativeBuildInputs = [ ];
          buildInputs = with pkgs; [
            pkg-config
            openssl
          ];
        };

        engine_deps = craneLib.buildDepsOnly (common_args // {
        });
        engine = craneLib.buildPackage (common_args // {
          cargoArtifacts = engine_deps;
        });

        surreal_deps = [ pkgs.surrealdb ];
        rust_dev_deps = [ pkgs.bacon ];
      in {
        defaultPackage = engine;

        devShell = pkgs.mkShell {
          nativeBuildInputs = [
            toolchain
          ] ++ surreal_deps ++ rust_dev_deps
          ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
            pkgs.darwin.Security
          ];
        };
      }
    );
}

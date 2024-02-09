{
  inputs = {
    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0.2311.555002.tar.gz";
    rust-overlay.url = "https://flakehub.com/f/oxalica/rust-overlay/0.1.1271.tar.gz";
    crane.url = "https://flakehub.com/f/ipetkov/crane/0.16.1.tar.gz";
    cargo-leptos-src = { url = "github:benwis/cargo-leptos"; flake = false; };
  };

  outputs = { self, nixpkgs, rust-overlay, crane, cargo-leptos-src, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = (import nixpkgs) {
          inherit system overlays;
          config.allowUnfree = true;
        };
        
        toolchain = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
          targets = [ "wasm32-unknown-unknown" ];
        });
        
        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

        filterGenerator = pattern: path: _type: builtins.match pattern path != null;
        protoOrCargo = path: type:
          (craneLib.filterCargoSources path type)
            || (filterGenerator ".*css$" path type)
            || (filterGenerator ".*js$" path type)
            || (filterGenerator ".*json$" path type)
            || (filterGenerator ".*lock$" path type)
            || (filterGenerator ".*ttf$" path type)
            || (filterGenerator ".*woff2$" path type)
            || (filterGenerator ".*webp$" path type)
            || (filterGenerator ".*jpeg$" path type)
            || (filterGenerator ".*png$" path type)
            || (filterGenerator ".*ico$" path type);

        # Include more types of files in our bundle
        src = pkgs.lib.cleanSourceWith {
          src = ./.; # The original, unfiltered source
          filter = protoOrCargo;
        };

        cargo-leptos = (import ./nix/cargo-leptos.nix) {
          inherit pkgs;
          cargo-leptos = cargo-leptos-src;
        };

        common_args = {
          inherit src;

          pname = "site-server";
          version = "0.1.0";

          nativeBuildInputs = [
            # Add additional build inputs here
            cargo-leptos
            pkgs.cargo-generate
            pkgs.binaryen
            pkgs.clang
            pkgs.mold

            # for styling
            pkgs.dart-sass
            pkgs.tailwindcss
            pkgs.yarn
            pkgs.yarn2nix-moretea.fixup_yarn_lock
          ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
            # Additional darwin specific inputs can be set here
            pkgs.libiconv
          ];

          buildInputs = [
            pkgs.pkg-config
            pkgs.openssl
          ];

        };

        # Build *just* the cargo dependencies, so we can reuse
        # all of that work (e.g. via cachix) when running in CI
        site-server-deps = craneLib.buildDepsOnly (common_args // {
          doCheck = false;
        });

        # an offline yarn registry for the tailwind packages
        style-js-packages-yarn-registry = pkgs.fetchYarnDeps {
          yarnLock = ./crates/site-app/style/tailwind/yarn.lock;
          hash = "sha256-nqOJBcjX+dFl/XkBH+HfRO6Ce+CErm3YkQjG1W+aUPw=";
          # hash = "";
        };

        # Build the actual crate itself, reusing the dependency
        # artifacts from above.
        site-server = craneLib.buildPackage (common_args // {
          buildPhaseCargoCommand = ''
            # install the yarn packages so `cargo-leptos` can build the styles
            export HOME=$(mktemp -d)
            cd crates/site-app/style/tailwind
            yarn config --offline set yarn-offline-mirror ${style-js-packages-yarn-registry}
            fixup_yarn_lock yarn.lock
            yarn install --offline --frozen-lockfile
            cd ../../../..

            # build the application
            cargo leptos build --release -vvv
          '';
          installPhaseCommand = ''
            mkdir -p $out/bin
            cp target/release/site-server $out/bin/
            cp -r target/site $out/bin/
          '';
          # Prevent cargo test and nextest from duplicating tests
          doCheck = false;
          cargoArtifacts = site-server-deps;

          SQLX_OFFLINE = "true";
          LEPTOS_BIN_PROFILE_RELEASE = "release";
          LEPTOS_LIB_PROFILE_RELEASE = "release-wasm-size";
          APP_ENVIRONMENT = "production";
        });
      in {
        defaultPackage = site-server;
        devShell = pkgs.mkShell {
          nativeBuildInputs = (with pkgs; [
            toolchain # cargo and such
            dive # docker images
            cargo-leptos
            flyctl # fly.io
            bacon # cargo check w/ hot reload

            # surreal stuff
            surrealdb surrealdb-migrations
          ])
            ++ common_args.buildInputs
            ++ common_args.nativeBuildInputs
            ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
              pkgs.darwin.Security
            ];
        };
      }
    );
}

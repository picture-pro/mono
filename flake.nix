{
  inputs = {
    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0.2311.556557.tar.gz";
    rust-overlay.url = "https://flakehub.com/f/oxalica/rust-overlay/0.1.1271.tar.gz";
    crane.url = "https://flakehub.com/f/ipetkov/crane/0.16.1.tar.gz";
    cargo-leptos-src = { url = "github:leptos-rs/cargo-leptos"; flake = false; };
    nix-filter.url = "github:numtide/nix-filter";
  };

  outputs = { self, nixpkgs, rust-overlay, crane, cargo-leptos-src, nix-filter, flake-utils }:
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

        src = nix-filter {
          root = ./.;
          include = [
            ./Cargo.toml
            ./Cargo.lock
            ./crates
            (nix-filter.lib.matchExt "toml")
          ];
        };

        cargo-leptos = (import ./nix/cargo-leptos.nix) {
          inherit pkgs craneLib;
          cargo-leptos = cargo-leptos-src;
        };

        common_args = {
          inherit src;

          pname = "site-server";
          version = "0.1.0";

          doCheck = false;

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
          ] ++ pkgs.lib.optionals (system == "x86_64-linux") [
            # extra packages only for x86_64-linux
            pkgs.nasm
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
          # if work is duplicated by the `server-site` package, update these
          # commands from the logs of `cargo leptos build --release -vvv`
          buildPhaseCargoCommand = ''
            # build the server dependencies
            cargo build --package=site-server --no-default-features --release
            # build the frontend dependencies
            cargo build --package=site-frontend --lib --target-dir=/build/source/target/front --target=wasm32-unknown-unknown --no-default-features --profile=wasm-release
          '';
        });

        # an offline yarn registry for the tailwind packages
        style-js-packages-yarn-registry = pkgs.fetchYarnDeps {
          yarnLock = ./crates/site-app/style/tailwind/yarn.lock;
          hash = "sha256-uYcqauHqsk58oWtA2uUYsJ2OuW8o2Rh6KrW88fK9UfE=";
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
            cp target/release/hash.txt $out/bin/
            cp -r target/site $out/bin/
          '';
          # Prevent cargo test and nextest from duplicating tests
          doCheck = false;
          cargoArtifacts = site-server-deps;

          APP_ENVIRONMENT = "production";
        });

        site-server-container = pkgs.dockerTools.buildLayeredImage {
          name = "site-server";
          tag = "latest";
          contents = [ site-server pkgs.cacert pkgs.surrealdb ];
          config = {
            Cmd = [ "site-server" ];
            WorkingDir = "${site-server}/bin";
          };
        };
      
      in {
        checks = {
          # Run clippy (and deny all warnings) on the crate source,
          # again, resuing the dependency artifacts from above.
          #
          # Note that this is done as a separate derivation so that
          # we can block the CI if there are issues here, but not
          # prevent downstream consumers from building our crate by itself.
          site-server-clippy = craneLib.cargoClippy (common_args // {
            cargoArtifacts = site-server-deps;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });

          site-server-doc = craneLib.cargoDoc (common_args // {
            cargoArtifacts = site-server-deps;
          });

          # Check formatting
          site-server-fmt = craneLib.cargoFmt {
            pname = common_args.pname;
            version = common_args.version;
            
            inherit src;
          };

          # Audit licenses
          site-server-deny = craneLib.cargoDeny {
            pname = common_args.pname;
            version = common_args.version;

            inherit src;
          };

          # Run tests with cargo-nextest
          # Consider setting `doCheck = false` on `site-server` if you do not want
          # the tests to run twice
          site-server-nextest = craneLib.cargoNextest (common_args // {
            cargoArtifacts = site-server-deps;
            partitions = 1;
            partitionType = "count";
          });
        };

        packages = {
          default = site-server;
          server = site-server;
          container = site-server-container;
        };
        
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = (with pkgs; [
            toolchain # cargo and such
            dive # docker images
            cargo-leptos
            flyctl # fly.io
            bacon # cargo check w/ hot reload
            cargo-deny # license checking

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

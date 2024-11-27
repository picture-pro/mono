{ inputs, ... }: {
  perSystem = { pkgs, rust-toolchain, ... }: let
    filter = inputs.nix-filter.lib;

    # configure the source
    src = filter {
      root = ../../.; # project root
      include = [
        "crates" "Cargo.toml" "Cargo.lock" # typical rust source
        ".cargo"                           # extra rust config
        (filter.matchExt "toml")           # extra toml used by other projects
        "media"                            # static assets
      ];
    };

    # build arguments for the whole workspace
    workspace-base-args = {
      inherit src;
      strictDeps = true;

      pname = "picturepro";
      version = "0.1";
      doCheck = false;

      # inputs assumed to be relevant for all crates
      nativeBuildInputs = with pkgs; [
        pkg-config clang lld
      ];
      buildInputs = [ ];
    };

    # build the deps for the whole workspace
    workspace-base-cargo-artifacts = rust-toolchain.craneLib.buildDepsOnly workspace-base-args;
  in {
    # pass back to the flake
    config._module.args.rust-workspace = {
      inherit workspace-base-args workspace-base-cargo-artifacts;
    };
    config.checks = {
      # run clippy, denying warnings
      rust-cargo-clippy = rust-toolchain.craneLib.cargoClippy (workspace-base-args // {
        cargoArtifacts = workspace-base-cargo-artifacts;
        cargoClippyExtraArgs = "--all-targets --no-deps -- --deny warnings";
      });
      # run rust-doc, denying warnings
      rust-cargo-docs = rust-toolchain.craneLib.cargoDoc (workspace-base-args // {
        cargoArtifacts = workspace-base-cargo-artifacts;
        cargoClippyExtraArgs = "--no-deps";
        RUSTDOCFLAGS = "-D warnings";
      });
      # run rust tests with nextest
      rust-cargo-nextest = rust-toolchain.craneLib.cargoNextest (workspace-base-args // {
        cargoArtifacts = workspace-base-cargo-artifacts;
        partitions = 1;
        partitionType = "count";
      });
      # run cargo fmt, failing if not already formatted perfectly
      rust-cargo-fmt = rust-toolchain.craneLib.cargoFmt workspace-base-args;
    };
  };
}

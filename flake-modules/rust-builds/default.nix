localFlake: { inputs, ... }: {
  perSystem = { pkgs, rust, ... }: let
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
    common-args = {
      inherit src;
      strictDeps = true;

      pname = "picturepro";
      version = "0.1";
      doCheck = false;

      # inputs assumed to be relevant for all crates
      nativeBuildInputs = with pkgs; [
        pkg-config
      ];
      buildInputs = [ ];
    };

    # build the deps for the whole workspace
    cargoArtifacts = rust.craneLib.buildDepsOnly common-args;

    
  in {
    checks = {
      # run clippy, denying warnings
      rust-cargo-clippy = rust.craneLib.cargoClippy (common-args // {
        inherit cargoArtifacts;
        cargoClippyExtraArgs = "--all-targets --no-deps -- --deny warnings";
      });
      # run rust-doc, denying warnings
      rust-cargo-docs = rust.craneLib.cargoDoc (common-args // {
        inherit cargoArtifacts;
        cargoClippyExtraArgs = "--no-deps";
        RUSTDOCFLAGS = "-D warnings";
      });
      # run rust tests with nextest
      rust-cargo-nextest = rust.craneLib.cargoNextest (common-args // {
        inherit cargoArtifacts;
        partitions = 1;
        partitionType = "count";
      });
      # run cargo fmt, failing if not already formatted perfectly
      rust-cargo-fmt = rust.craneLib.cargoFmt common-args;
    };
  };
}

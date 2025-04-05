localFlake: { inputs, ... }: {
  perSystem = { pkgs, ... }: let
    # build the CI and dev toolchains
    toolchain = p: p.rust-bin.selectLatestNightlyWith (toolchain: toolchain.minimal.override {
      extensions = [ "rustfmt" "clippy" ];
      targets = [ "wasm32-unknown-unknown" ];
    });
    dev-toolchain = p: p.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
      extensions = [ "rust-src" "rust-analyzer" "rustc-codegen-cranelift-preview" ];
      targets = [ "wasm32-unknown-unknown" ];
    });

    # configure crane to use the CI toolchain
    craneLib = (inputs.crane.mkLib pkgs).overrideToolchain toolchain;
  in {
    config._module.args.rust-toolchain = {
      inherit toolchain dev-toolchain craneLib;
    };
  };
}

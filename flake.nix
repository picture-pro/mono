{
  description = "Flake for everything involved in PicturePro";

  inputs = {
    # foundational
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "https://flakehub.com/f/hercules-ci/flake-parts/0.1.tar.gz";

    # still first party I guess
    rambit.url = "github:rambit-systems/rambit";
    tikv-explorer.url = "github:rambit-systems/tikv-explorer";

    # nix tools
    nix-filter.url = "github:numtide/nix-filter";
    devshell.url = "github:numtide/devshell";

    # rust toolchaining
    rust-overlay = {
      url = "https://flakehub.com/f/oxalica/rust-overlay/0.1.tar.gz";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "https://flakehub.com/f/ipetkov/crane/0.20.tar.gz";
  };

  outputs = inputs: inputs.flake-parts.lib.mkFlake { inherit inputs; } (top @ { ... }: {
    systems = [ "x86_64-linux" "aarch64-linux" ];

    imports = let
      inherit (top.flake-parts-lib) importApply;
    in [
      # configures extra flake outputs
      (importApply ./flake-modules/flake-outputs { })
      # configures nixpkgs with overlays
      (importApply ./flake-modules/nixpkgs { })
      # configures rust toolchains
      (importApply ./flake-modules/rust-toolchain { })
      # defines rust builds
      (importApply ./flake-modules/rust-builds { })
      # defines devshell
      (importApply ./flake-modules/devshell { })
    ];
  });
}

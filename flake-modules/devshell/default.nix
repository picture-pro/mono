localFlake: { ... }: {
  perSystem = { pkgs, rust, ... }: let
    mkShell = pkgs.devshell.mkShell;
  in {
    devShells.default = mkShell {
      packages = with pkgs; [
        # rust dev toolchain (with RA), built from current nixpkgs
        (rust.dev-toolchain pkgs)

        # dependencies for local rust builds
        pkg-config
        openssl
        gcc
      ];

      motd = "\n  Welcome to the {2}picturepro{reset} dev shell. Run {1}menu{reset} for commands.\n";
    };
  };
}

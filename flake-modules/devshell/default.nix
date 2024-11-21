localFlake: { ... }: {
  perSystem = { pkgs, rust, inputs', ... }: let
    mkShell = pkgs.devshell.mkShell;

    # note; there's a UTF-8 control character in the esc string below
    esc = "";
    # for highlighting binary names in the help text
    # nix doesn't escape the ctrl character, so we can just put it straight in
    bin-hl = s: "${esc}[31;1m${s}${esc}[0m";

    dockerLoad = imagePath: "docker load -i ${imagePath}";
    runEphemeralDocker = { imageName, imageVersion }: ''
      docker run --rm --network host ${imageName}-server:${imageVersion}
    '';
    ephemeralDockerCommand = args @ { imagePath, imageName, imageVersion, ... }: {
      # use the commandName argument if provided, otherwise default to the image name
      name = args.commandName or "run-${imageName}";
      command = ''
        ${dockerLoad imagePath} && \
        ${runEphemeralDocker { inherit imageName imageVersion; }}
      '';
      help = args.desc or "Run the ${bin-hl imageName} server in an ephemeral container.";
      category = "[docker actions]";
    };
    tikv-docker-commands = [
      ( ephemeralDockerCommand {
        imagePath = inputs'.rambit.images.tikv;
        imageName = "tikv";
        imageVersion = "8.1.1";
      } )
      ( ephemeralDockerCommand {
        imagePath = inputs'.rambit.images.pd;
        imageName = "pd";
        imageVersion = "8.1.1";
      } )
      ( ephemeralDockerCommand {
        imagePath = inputs'.tikv-explorer.packages.container;
        imageName = "tikv-explorer";
        imageVersion = "latest";
        commandName = "tikv-explorer";
        desc = "Run ${bin-hl "tikv-explorer"}.";
      } )
    ];
    
  in {
    devShells.default = mkShell {
      packages = with pkgs; [
        # rust dev toolchain (with RA), built from current nixpkgs
        (rust.dev-toolchain pkgs)

        # tools
        mprocs # runs commands in parallel

        # dependencies for local rust builds
        pkg-config
        openssl
        gcc
      ];

      motd = "\n  Welcome to the {2}picturepro{reset} dev shell. Run {1}menu{reset} for commands.\n";

      commands = [
        {
          name = "tikv";
          command = "mprocs \"run-tikv\" \"run-pd\"";
          help = "Run the ${bin-hl "tikv"} stack";
          category = "[stack actions]";
        }
      ]
        ++ tikv-docker-commands
        ;
    };
  };
}

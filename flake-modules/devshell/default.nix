localFlake: { ... }: {
  perSystem = { pkgs, rust-toolchain, inputs', ... }: let
    mkShell = pkgs.devshell.mkShell;

    # note; there's a UTF-8 control character in the esc string below
    esc = "";
    # for highlighting binary names in the help text
    # nix doesn't escape the ctrl character, so we can just put it straight in
    bin-hl = s: "${esc}[31;1m${s}${esc}[0m";

    dockerLoad = imagePath: "docker load -i ${imagePath}";
    runEphemeralDocker = { imageName, imageVersion }: ''
      docker run --rm --network host ${imageName}:${imageVersion}
    '';
    ephemeralDockerCommand = args @ { imagePath, imageName, imageVersion, ... }: {
      # use the commandName argument if provided, otherwise default to the image name
      name = args.commandName or "run-${imageName}";
      command = ''
        ${dockerLoad imagePath} && \
        ${runEphemeralDocker { inherit imageName imageVersion; }}
      '';
      help = args.desc or "Runs the ${bin-hl imageName} server in an ephemeral container.";
      category = "[docker actions]";
    };
    tikv-docker-commands = [
      ( ephemeralDockerCommand {
        imagePath = inputs'.rambit.images.tikv;
        imageName = "tikv-server";
        imageVersion = "8.1.1";
        commandName = "run-tikv";
      } )
      ( ephemeralDockerCommand {
        imagePath = inputs'.rambit.images.pd;
        imageName = "pd-server";
        imageVersion = "8.1.1";
        commandName = "run-pd";
      } )
      ( ephemeralDockerCommand {
        imagePath = inputs'.tikv-explorer.packages.container;
        imageName = "tikv-explorer";
        imageVersion = "latest";
        commandName = "tikv-explorer";
        desc = "Runs ${bin-hl "tikv-explorer"}.";
      } )
    ];
    
  in {
    devShells.default = mkShell {
      packages = with pkgs; [
        # rust dev toolchain (with RA), built from current nixpkgs
        (rust-toolchain.dev-toolchain pkgs)

        # tools
        mprocs # runs commands in parallel
        cargo-leptos # leptos build tool
        dart-sass tailwindcss yarn # css build tools
        flyctl

        # dependencies for local rust builds
        pkg-config openssl
        clang lld mold # faster linking + gcc for some crates
      ];

      motd = "\n  Welcome to the {2}picturepro{reset} dev shell. Run {1}menu{reset} for commands.\n";

      commands = [
        {
          name = "tikv";
          command = "mprocs \"run-tikv\" \"run-pd\" $@";
          help = "Runs the ${bin-hl "tikv"} stack.";
          category = "[stack actions]";
        }
        {
          name = "check";
          command = "nix flake check $@";
          help = "Runs nix flake checks.";
          category = "[nix actions]";
        }
        {
          name = "container";
          command = ''
            docker load -i $(nix build .#site-server-container --print-out-paths --no-link) && \
            docker run --rm --network host -e TIKV_URLS=$TIKV_URLS site-server:latest
          '';
          help = "Runs the ${bin-hl "site-server"} in a container.";
        }
        {
          name = "watch";
          command = "cargo leptos watch -v $@";
          help = "Runs ${bin-hl "cargo-leptos"} in watch mode.";
          category = "[build actions]";
        }
        {
          name = "serve";
          command = "cargo leptos serve -v $@";
          help = "Runs ${bin-hl "cargo-leptos"} in watch mode.";
          category = "[build actions]";
        }
        {
          name = "serve-release";
          command = "cargo leptos serve -v --release $@";
          help = "Runs ${bin-hl "cargo-leptos"} in watch mode.";
          category = "[build actions]";
        }
      ]
        ++ tikv-docker-commands
        ;
    };
  };
}

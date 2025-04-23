localFlake: { ... }: {
  imports = [
    ./workspace.nix
    ./leptos.nix
    ./crate-graph.nix
  ];
}

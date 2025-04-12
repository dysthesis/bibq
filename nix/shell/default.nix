pkgs:
pkgs.mkShell {
  name = "bibq";
  packages = with pkgs; [
    nixd
    alejandra
    statix
    deadnix
    npins
    cargo
    rustToolchains.nightly
    bacon
  ];
}

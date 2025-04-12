{
  self,
  pkgs,
  lib,
  inputs,
  ...
}: rec {
  default = mdq;
  mdq = pkgs.callPackage ./bibq.nix {inherit pkgs inputs lib self;};
}

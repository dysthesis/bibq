{
  self,
  pkgs,
  lib,
  inputs,
  ...
}: rec {
  default = bibq;
  bibq = pkgs.callPackage ./bibq.nix {inherit pkgs inputs lib self;};
}

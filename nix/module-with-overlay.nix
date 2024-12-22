self:
{ pkgs, ... }:
{
  imports = [ self.nixosModules.letterbox ];

  config = {
    nixpkgs.overlays = [ self.overlays.default ];
    services.letterbox.package = pkgs.letterbox;
  };
}

# SPDX-FileCopyrightText: 2023 Sefa Eyeoglu <contact@scrumplex.net>
#
# SPDX-License-Identifier: GPL-3.0-or-later
{ module, ... }:
{
  name = "skinprox-test";

  nodes.machine =
    { pkgs, ... }:
    {
      imports = [
        module
      ];

      services.letterbox = {
        enable = true;
        environmentFile = pkgs.writeText "letterbox.env" ''
          DISCORD_BOT_TOKEN=example
        '';
        settings = {
          server_id = 1;
          staff_roles = [ 2 ];
          forum_channel.id = 3;
        };
      };
    };

  testScript = ''
    start_all()
    machine.wait_for_unit("letterbox.service")
  '';
}

self:
{
  config,
  lib,
  pkgs,
  ...
}:
let
  cfg = config.services.letterbox;

  inherit (lib)
    getExe
    mkEnableOption
    mkIf
    mkOption
    mkPackageOption
    optionals
    types
    ;

  tomlFormat = pkgs.formats.toml { };
in
{
  options.services.letterbox = {
    enable = mkEnableOption "letterbox";
    package = mkPackageOption self.packages.${pkgs.stdenv.hostPlatform.system} "letterbox" { };

    createDatabaseLocally = mkOption {
      description = ''
        Whether a PostgreSQL database should be automatically created and
        configured on the local host. If set to `false`, you need provision a
        database yourself and make sure to create the hstore extension in it.
      '';
      type = types.bool;
      default = true;
      example = false;
    };

    environmentFile = mkOption {
      description = ''
        Environment file as defined in {manpage}`systemd.exec(5)`
      '';
      type = types.nullOr types.path;
      default = null;
      example = "/run/agenix/letterbox";
    };

    settings = mkOption {
      description = ''
        Contents of `config.toml`. See `config.toml.example`.
      '';
      type = types.submodule {
        freeformType = tomlFormat.type;
        options = {
          server_id = mkOption {
            description = ''
              Commands will be limited to this server and its icon will be used
              for anonymous replies - required
            '';
            type = with types; nullOr ints.unsigned;
            default = null;
            example = 1031648380885147709;
          };

          staff_roles = mkOption {
            description = ''
              Commands will only be useable by IDs here - required to use the
              app in any meaningful way
            '';
            type = with types; listOf ints.unsigned;
            default = [ ];
            example = [ 1061922913839763487 ];
          };

          forum_channel.id = mkOption {
            description = ''
              Threads will be created in this channel - required
            '';
            type = with types; nullOr ints.unsigned;
            default = null;
            example = 1323366387313279056;
          };
        };
      };
    };
  };

  config = mkIf cfg.enable {
    assertions = [
      {
        assertion = cfg.settings.server_id != null;
        message = "Letterbox server id is required.";
      }
      {
        assertion = cfg.settings.forum_channel.id != null;
        message = "Letterbox form channel id is required.";
      }
    ];

    services.postgresql = mkIf cfg.createDatabaseLocally {
      ensureDatabases = [ "letterbox" ];
      ensureUsers = [
        {
          name = "letterbox";
          ensureDBOwnership = true;
        }
      ];
    };

    systemd.services."letterbox" = {
      enable = true;
      wantedBy = [ "multi-user.target" ];
      after = [ "network.target" ] ++ optionals (cfg.createDatabaseLocally) [ "postgresql.service" ];

      environment = mkIf cfg.createDatabaseLocally {
        POSTGRES_CONNECTION = "postgres://letterbox?host=/var/run/postgresql";
        CONFIG_PATH = tomlFormat.generate "letterbox-config.toml" cfg.settings;
      };

      serviceConfig = {
        Restart = "on-failure";
        ExecStart = getExe cfg.package;

        EnvironmentFile = mkIf (cfg.environmentFile != null) [ cfg.environmentFile ];

        DynamicUser = true;

        # hardening
        NoNewPrivileges = true;
        PrivateDevices = true;
        PrivateTmp = true;
        PrivateUsers = true;
        ProtectClock = true;
        ProtectControlGroups = true;
        ProtectHome = true;
        ProtectHostname = true;
        ProtectKernelLogs = true;
        ProtectKernelModules = true;
        ProtectKernelTunables = true;
        ProtectSystem = "strict";
        RestrictNamespaces = true;
        RestrictSUIDSGID = true;
        SystemCallArchitectures = "native";
        SystemCallFilter = [
          "@system-service"
          "~@resources"
          "~@privileged"
        ];
      };
    };
  };
}

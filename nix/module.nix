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
    literalExpression
    mkEnableOption
    mkIf
    mkOption
    mkPackageOption
    optionals
    types
    ;
in
{
  options.services.letterbox = {
    enable = mkEnableOption "letterbox";
    package = mkPackageOption self.packages.${pkgs.stdenv.hostPlatform.system} "letterbox" { };

    createDatabaseLocally = mkOption {
      type = types.bool;
      default = true;
      description = ''
        Whether a PostgreSQL database should be automatically created and
        configured on the local host. If set to `false`, you need provision a
        database yourself and make sure to create the hstore extension in it.
      '';
    };

    environmentFile = mkOption {
      description = ''
        Environment file as defined in {manpage}`systemd.exec(5)`
      '';
      type = types.nullOr types.path;
      default = null;
      example = literalExpression ''
        "/run/agenix.d/1/letterbox"
      '';
    };
  };

  config = mkIf cfg.enable {
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

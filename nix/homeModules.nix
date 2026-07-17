{
  config,
  lib,
  pkgs,
  asteride-pkg,
}:
let
  cfg = config.programs.asteride;
  tomlFormat = pkgs.formats.toml { };
in
{
  options.programs.asteride = {
    enable = lib.mkEnableOption "asteride";
  
    package = lib.mkOption {
      type = lib.types.package;
      default = asteride-pkg;
      description = "The asteride package to install.";
    };   

    settings = lib.mkOption {
      inherit (tomlFormat) type;
      default = { };
      description = "Settings written to asteride/config.toml, as-is.";
      example = lib.literalExpression ''
        {
          ui_scale = 1.25;
          theme = "dark";
        }
      '';
    };
  };
  config = lib.mkIf cfg.enable {
    home.packages = [ cfg.package ];

    xdg.configFile."asteride/config.toml" = lib.mkIf (cfg.settings != { }) {
      source = tomlFormat.generate "asteride-config.toml" cfg.settings;
    };
  };
}

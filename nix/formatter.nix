{
  pkgs,
  inputs
}:
(inputs.treefmt-nix.lib.evalModule pkgs (_: {
  projectRootFile = ".git/config";
  programs = {
    nixfmt.enable = true;
    nixf-diagnose.enable = true;
    taplo.enable = true;
    rustfmt.enable = true;
  };
  settings.formatter = {
    rustfmt = {
      options = [
        "--config"
        "condense_wildcard_suffixes=true,tab_spaces=2,imports_layout=vertical"
        "--style-edition"
        "2024"
      ];
    };
  };
})).config.build

{
  description = "routing metaheuristics";
  nixConfig.bash-prompt = "e[1;34m [TMSalgos@$HOSTNAME:$PWD]$ e[0m";

  inputs = {
    nixpkgs.url =
      "github:NixOS/nixpkgs?rev=46821ea01c8f54d2a20f5a503809abfc605269d7";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        packageName = "TMSalgos";

      in {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [ rls cargo rustc rustfmt ];
        };
      });
}

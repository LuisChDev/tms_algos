{
  description = "routing metaheuristics";
  nixConfig.bash-prompt = "\\e[1;34m\n\[tms_algos@\$HOSTNAME:\$PWD\]$ \\e[0m";

  inputs = {
    nixpkgs.url =
      "github:NixOS/nixpkgs?rev=46821ea01c8f54d2a20f5a503809abfc605269d7";
    rust-overlay.url =
      "github:oxalica/rust-overlay?rev=43f4c4319fd29d07912a65d405ff03069c7748c4";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [rust-overlay.overlay ];
        };
        packageName = "TMSalgos";

      in {
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default))
          ];

          buildInputs = with pkgs; [
            # cargo
            # cargo-edit
            # rustc
            # rustfmt
            # rust-analyzer

            # debugger support
            gdb
            gdbgui
          ];
        };
      });
}

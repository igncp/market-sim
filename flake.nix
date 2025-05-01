{
  inputs = {
    unstable.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = {
    unstable,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      is-ci = builtins.pathExists ./is-ci;

      pkgs = import unstable {
        inherit system;
      };
      dev-hook = ''
        PATH="$HOME/.rustup/bin:$PATH"

        if [ -z "$(rustup component list | grep analy | grep install || true)" ]; then
          rustup component add rust-analyzer
        fi
      '';
    in {
      devShell = pkgs.mkShell {
        RUST_BACKTRACE = 1;

        shellHook =
          ''
            export PATH=$PATH:$HOME/.cargo/bin
          ''
          + (
            if is-ci
            then ""
            else dev-hook
          );

        packages = with pkgs; [
          iredis
          openssl
          pkg-config
          rustup
        ];
      };
    });
}

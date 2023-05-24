{
  description = "gross, an info aggregator for eww";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };

    naersk.url = "github:nix-community/naersk/master";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux"];

      perSystem = {
        config,
        pkgs,
        system,
        ...
      }: {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            cargo
            clippy
            pre-commit
            rust-analyzer
            rustc
            rustfmt
            rustPackages.clippy
            vscode-extensions.llvm-org.lldb-vscode
          ];

          nativeBuildInputs = with pkgs; [pkg-config];
          buildInputs = with pkgs; [dbus];

          RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
        };

        packages = {
          gross = (pkgs.callPackage inputs.naersk {}).buildPackage {
            src = ./.;
            nativeBuildInputs = with pkgs; [pkg-config];
            buildInputs = with pkgs; [dbus];
          };
        } // { default = config.packages.gross; };

        formatter = pkgs.alejandra;
      };
    };
}

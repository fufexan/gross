{
  description = "gross, an info aggregator for eww";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
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
          inputsFrom = [config.packages.gross];
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

          RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
        };

        packages =
          {
            gross = pkgs.rustPlatform.buildRustPackage {
              pname = "gross";
              version = "0.1.0";

              src = ./.;

              cargoLock = {
                lockFile = ./Cargo.lock;
                outputHashes."fastblur-0.1.1" = "sha256-GRZbQn3+R5vkfOzB2F6WoKOf7hSiWO3qCpeir2VZtzM=";
              };

              nativeBuildInputs = with pkgs; [pkg-config];
              buildInputs = with pkgs; [dbus openssl];
            };
          }
          // {default = config.packages.gross;};

        formatter = pkgs.alejandra;
      };
    };
}

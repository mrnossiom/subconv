{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs?ref=nixos-unstable";

    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";

    gitignore.url = "github:hercules-ci/gitignore.nix";
    gitignore.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      gitignore,
    }:
    let
      inherit (nixpkgs.lib) genAttrs getExe;

      forAllSystems = genAttrs [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];
      forAllPkgs = function: forAllSystems (system: function pkgs.${system});

      mkApp = (
        program: {
          type = "app";
          inherit program;
        }
      );

      pkgs = forAllSystems (
        system:
        import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        }
      );
    in
    {
      formatter = forAllPkgs (pkgs: pkgs.nixfmt-tree);

      packages = forAllPkgs (pkgs: rec {
        default = subconv;
        subconv = pkgs.callPackage ./package.nix { inherit gitignore; };
      });

      apps = forAllSystems (system: rec {
        default = subconv;
        subconv = mkApp (getExe self.packages.${system}.subconv);
      });

      devShells = forAllPkgs (
        pkgs:
        let
          file-rust-toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          rust-toolchain = file-rust-toolchain.override { extensions = [ "rust-analyzer" ]; };
        in
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              pkg-config
              rust-toolchain

              stdenv.cc.cc
              cmake

              libuchardet
            ];

            RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;

            RUST_LOG = "info";
          };
        }
      );
    };
}

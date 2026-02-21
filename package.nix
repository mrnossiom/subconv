{
  lib,

  gitignore,
  rustPlatform,

  cmake,
  libuchardet,
  stdenv,
}:

let
  inherit (gitignore.lib) gitignoreSource;

  src = gitignoreSource ./.;
  cargo-toml = lib.importTOML "${src}/Cargo.toml";
in
rustPlatform.buildRustPackage {
  pname = cargo-toml.package.name;
  version = cargo-toml.package.version;

  inherit src;

  cargoLock.lockFile = "${src}/Cargo.lock";

  nativeBuildInputs = [
    stdenv.cc.cc
    cmake
  ];

  buildInputs = [
    libuchardet
  ];

  meta = {
    inherit (cargo-toml.package) description homepage license;
    mainProgram = "subconv";
  };
}

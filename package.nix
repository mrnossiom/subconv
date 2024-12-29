{ lib

, gitignore
, rustPlatform

, cmake
, libuchardet
, stdenv
}:

let
  inherit (gitignore.lib) gitignoreSource;

  src = gitignoreSource ./.;
  cargoTOML = lib.importTOML "${src}/Cargo.toml";
in
rustPlatform.buildRustPackage {
  pname = cargoTOML.package.name;
  version = cargoTOML.package.version;

  inherit src;

  cargoLock = { lockFile = "${src}/Cargo.lock"; };

  nativeBuildInputs = [
    stdenv.cc.cc
    cmake
  ];

  buildInputs = [
    libuchardet
  ];

  meta = {
    inherit (cargoTOML.package) description homepage license;
    maintainers = cargoTOML.package.authors;
    mainProgram = "subconv";
  };
}

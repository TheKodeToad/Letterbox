{
  lib,
  rustPlatform,
  pkg-config,
  openssl,
}:
rustPlatform.buildRustPackage rec {
  pname = "letterbox";
  inherit (passthru.cargoToml.package) version;

  src = lib.fileset.toSource {
    root = ../../.;
    fileset = lib.fileset.intersection (lib.fileset.gitTracked ../../.) (
      lib.fileset.unions [
        ../../migrations
        ../../src
        ../../Cargo.lock
        ../../Cargo.toml
      ]
    );
  };

  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ openssl ];

  cargoLock.lockFile = ../../Cargo.lock;

  passthru.cargoToml = lib.importTOML ../../Cargo.toml;

  meta = with lib; {
    homepage = "https://github.com/TheKodeToad/Letterbox";
    description = "Simplistic, lightweight ModMail bot app for Discord using modern features like forums";
    license = licenses.mit;
    maintainers = with maintainers; [ Scrumplex ];
    mainProgram = "letterbox";
  };
}

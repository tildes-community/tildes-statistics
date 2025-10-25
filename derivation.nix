{ lib, rustPlatform }:

rustPlatform.buildRustPackage rec {
  pname = "tildes-statistics";
  version = "0.1.0";

  src = ./.;

  cargoHash = "sha256-1cEyaDGbXvWJyzIhG6f6z2SfoPhSAoqzuHd4Jfy3PPc=";

  meta = with lib; {
    description = "Statistics for Tildes.net.";
    homepage = "https://ts.bauke.xyz";
    license = licenses.agpl3Plus;
    maintainers = with maintainers; [ Bauke ];
  };
}

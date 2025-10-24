{ lib, rustPlatform }:

rustPlatform.buildRustPackage rec {
  pname = "tildes-statistics";
  version = "0.1.0";

  src = ./.;

  cargoHash = "sha256-4bdkOeb/NOhMKhUjji5Tjv8HZZGn65vJm0h+Tm/eZ50=";

  meta = with lib; {
    description = "Statistics for Tildes.net.";
    homepage = "https://ts.bauke.xyz";
    license = licenses.agpl3Plus;
    maintainers = with maintainers; [ Bauke ];
  };
}

{ lib, rustPlatform }:

rustPlatform.buildRustPackage rec {
  pname = "tildes-statistics";
  version = "0.1.0";

  src = ./.;

  cargoHash = "sha256-z3OkYWvK8n7QYL4EuVudgSufGJzzZP3Q3lctySt3Fcs=";

  meta = with lib; {
    description = "Statistics for Tildes.net.";
    homepage = "https://ts.bauke.xyz";
    license = licenses.mit;
    maintainers = with maintainers; [ Bauke ];
  };
}

{
  lib,
  rustPlatform,
}: let
  version = "0.1.0";
  owner = "ynqa";
  repo = "sig";
in
  rustPlatform.buildRustPackage {
    pname = "sig";
    inherit version;

    src = ./.;

		cargoHash = "sha256-yz/DPJJxtsHpJLpTMAYfsq9miIs48F+FeSnmkQ707uA=";

    meta = {
      description = "Interactive grep (for streaming)";
      homepage = "https://github.com/${owner}/${repo}";
      license = [lib.licenses.mit];
      mainProgram = "sig";
    };
  }

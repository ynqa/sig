{
  lib,
  rustPlatform,
  fetchFromGitHub,
}: let
  version = "0.1.0";
  owner = "ynqa";
  repo = "sig";
in
  rustPlatform.buildRustPackage {
    pname = "sig";
    inherit version;

    src = fetchFromGitHub {
      inherit owner repo;
      rev = "v${version}";
      hash = "sha256-KHXBeQFmuA3YO9AN5dkY/fl/z2RdbR6AqSSEGUNrxt4=";
    };

    cargoHash = "sha256-3nNdCz6Yjnnle/sG6kqGCAcoVRNhc4BOteWGDDl5oWs=";

    meta = {
      description = "Interactive grep (for streaming)";
      homepage = "https://github.com/${owner}/${repo}";
      license = [lib.licenses.mit];
      mainProgram = "sig";
    };
  }

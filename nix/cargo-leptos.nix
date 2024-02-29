{pkgs, cargo-leptos}: 
	pkgs.rustPlatform.buildRustPackage rec {
    pname = "cargo-leptos";
    version = "0.2.7";
    buildFeatures = ["no_downloads"]; # cargo-leptos will try to download Ruby and other things without this feature

    src = cargo-leptos;

    cargoSha256 = "sha256-jy1bWIXOjVwUhT5GAkIvTw/lPrvw6szAQflnfmhfgow=";
    # cargoSha256 = "";

    nativeBuildInputs = [pkgs.pkg-config pkgs.openssl];

    buildInputs = with pkgs;
      [openssl pkg-config]
      ++ lib.optionals stdenv.isDarwin [
      darwin.Security darwin.apple_sdk.frameworks.CoreServices darwin.apple_sdk.frameworks.SystemConfiguration
    ];

    doCheck = false; # integration tests depend on changing cargo config

    meta = with pkgs.lib; {
      description = "A build tool for the Leptos web framework";
      homepage = "https://github.com/leptos-rs/cargo-leptos";
      changelog = "https://github.com/leptos-rs/cargo-leptos/blob/v${version}/CHANGELOG.md";
      license = with licenses; [mit];
      maintainers = with maintainers; [benwis];
    };
  }

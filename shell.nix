
with import ./pin-unstable.nix { };

stdenv.mkDerivation {
  name = "rust-env";
  # RUST_SRC_PATH = import ./rust-src.nix {
  # inherit stdenv;
  # inherit rustc;
  # };
  nativeBuildInputs = [
    rustc cargo
    inotify-tools
    # Example Build-time Additional Dependencies
    pkgconfig
  ];
  buildInputs = [
    # Example Run-time Additional Dependencies
    openssl
  ];

  # Set Environment Variables
  RUST_BACKTRACE = 1;
}


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
    xorg.libX11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    libGL
    libGLU
    freeglut
  ];
  buildInputs = [
    # Example Run-time Additional Dependencies
    openssl
  ];

  # Set Environment Variables
  RUST_BACKTRACE = 1;
  LD_LIBRARY_PATH="/run/opengl-driver/lib;${xorg.libX11}/lib/;${libGL}/lib/;${libGLU}/lib;${freeglut}/lib;${xorg.libXcursor}/lib;${xorg.libXrandr}/lib;${xorg.libXi}/lib";
  WINIT_UNIX_BACKEND="x11";
}

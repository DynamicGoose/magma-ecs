with import <nixpkgs> {};

stdenv.mkDerivation rec {
  name = "magma_ecs";
  buildInputs = [
    
  ];

  # WINIT_UNIX_BACKEND=wayland/x11

  LD_LIBRARY_PATH = builtins.foldl'
    (a: b: "${a}:${b}/lib") "${vulkan-loader}/lib" buildInputs;
}

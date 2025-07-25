let
  shell =
    pkgs:
    pkgs.mkShell rec {
      nativeBuildInputs = [ pkgs.rust-bin.stable.latest.default ];
      buildInputs = [ pkgs.vulkan-loader ];
      env = {
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
      };
    };
  havRust = (
    import (
      builtins.fetchTarball {
        url = "https://github.com/oxalica/rust-overlay/archive/cf608fb54d8854f31d7f7c499e2d2c928af48036.tar.gz";
        sha256 = "sha256:02m960sjdga9d3vsl10z33n0gsqx1272a3jfg9nnybbznsj1w3zc";
      }
    )
  );
  pinnedPkgs = import (builtins.fetchTarball {
    url = "https://github.com/NixOS/nixpkgs/archive/3ff0e34b1383648053bba8ed03f201d3466f90c9.tar.gz";
    sha256 = "sha256:1mg2rlxz4q2sq94pg9bi0hpqibxsk0cw0v7g383qzr79vd3gdm89";
  }) { };
in
{
  pkgs ? pinnedPkgs,
}:
shell (pkgs.appendOverlays [ havRust ])

{ pkgs ? import <nixpkgs> {}
  , fetchzip ? pkgs.fetchzip
  , fetchgit ? pkgs.fetchgit
  , fetchurl ? pkgs.fetchurl
  , unzip ? pkgs.unzip
}:

pkgs.mkShell {
  buildInputs = [
    pkgs.rustup
    pkgs.rustfmt
    pkgs.libclang
    pkgs.gnumake
    pkgs.python39
    pkgs.gcc-arm-embedded
    pkgs.poetry
    pkgs.cacert
    #(pkgs.callPackage custom_nix_pkgs/analog_openocd.nix { })
  ];

  LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
  GCC_ARM_PATH = "${pkgs.gcc-arm-embedded}";

  shellHook =
    ''
      rustup default nightly
      rustup target add thumbv7em-none-eabi
    '';
}

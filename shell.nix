/**
 * @file "shell.nix"
 * @author Ben Janis
 * @brief Nix shell environment file
 * @date 2024
 *
 * This source file is part of an example system for MITRE's 2024 Embedded System CTF (eCTF).
 * This code is being provided only for educational purposes for the 2024 MITRE eCTF competition,
 * and may not meet MITRE standards for quality. Use this code at your own risk!
 *
 * @copyright Copyright (c) 2024 The MITRE Corporation
 */

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
    (pkgs.callPackage custom_nix_pkgs/analog_openocd.nix { })
    pkgs.minicom
  ];

  LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
  GCC_ARM_PATH = "${pkgs.gcc-arm-embedded}";

  shellHook =
    ''
      chmod -R u+rwX,go+rX,go-w $PWD/msdk
      export MAXIM_PATH=$PWD/msdk

      rustup default nightly
      rustup target add thumbv7em-none-eabihf
    '';
}

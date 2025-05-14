# SPDX-FileCopyrightText: Olivier Le Doeuff <olivier.ldff@gmail.com>
# SPDX-License-Identifier: MIT
{
  description = "cxx-qt-experiments";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    nix-gl-host = {
      url = "github:numtide/nix-gl-host";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs =
    { flake-utils
    , ...
    }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import inputs.nixpkgs {
        inherit system;
        overlays = [
          (import inputs.rust-overlay)
          (_: prev: {
            latestMakeRustPlatform = prev.makeRustPlatform {
              rustc = prev.rust-bin.stable.latest.default;
              cargo = prev.rust-bin.stable.latest.default;
            };
          })
        ];
      };
      nixglhost =
        if pkgs.stdenv.isLinux then inputs.nix-gl-host.packages.${system}.default else null;
      nativeBuildInputs = with pkgs; [
        qt6.wrapQtAppsHook
        makeWrapper
        gcc
        cmake
        cpm-cmake
        ninja
        pkg-config
        (lib.hiPrio rust-bin.nightly."2025-03-28".rustfmt)
        (rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src" # for rust-analyzer
            "rust-analyzer"
            "clippy"
          ];
        })
        latestMakeRustPlatform.bindgenHook
      ];
      # See https://github.com/NixOS/nixpkgs/pull/248149
      NIX_QMLIMPORTSCANNER = if pkgs.stdenv.isDarwin then "${pkgs.qt6.qtdeclarative}/libexec/qmlimportscanner" else null;
      buildInputs = with pkgs; [
        qt6.qtbase
        qt6.full
      ];
      shellHook = ''
        # Crazy shell hook to set up Qt environment, from:
        # https://discourse.nixos.org/t/python-qt-woes/11808/12
        setQtEnvironment=$(mktemp --suffix .setQtEnvironment.sh)
        echo "shellHook: setQtEnvironment = $setQtEnvironment"
        makeWrapper "/bin/sh" "$setQtEnvironment" "''${qtWrapperArgs[@]}"
        sed "/^exec/d" -i "$setQtEnvironment"
        source "$setQtEnvironment"

        # Make sure cxx-qt uses qmake from qt6.full, so qmake -query return folder with all of qt's libraries
        export QMAKE=${pkgs.qt6.full}/bin/qmake
        # TODO: make it only on darwin
        # While https://github.com/NixOS/nixpkgs/issues/395191 is opened, quick fix from:
        # https://github.com/andresilva/polkadot.nix/issues/36#issuecomment-2807556409
        export CRATE_CC_NO_DEFAULTS=1
      '';
      devShellHook = pkgs.lib.concatStringsSep "\n" (
        [ shellHook ]
        # See https://github.com/NixOS/nixpkgs/pull/248149
        ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [ "export QML2_IMPORT_PATH=$NIXPKGS_QT6_QML_IMPORT_PATH" ]
      );
      fullDevBuildInputs = nativeBuildInputs
        ++ (with pkgs;[
        nixpkgs-fmt
        cmake-format
        clang-tools
        git
        nixglhost
        sccache
        nil
        glslang
      ]);
      devShells = {
        default = pkgs.mkShell {
          name = "cxx-qt-dev-shell";
          inherit buildInputs;
          inherit NIX_QMLIMPORTSCANNER;
          # Somehow the cache miss some recompilation leaving me with wrong qml files
          # Doc: https://doc.qt.io/qt-5/qmldiskcache.html
          QML_DISABLE_DISK_CACHE = 1;
          shellHook = devShellHook;
          nativeBuildInputs = fullDevBuildInputs;
        };
      };
    in
    {
      inherit devShells;
      formatter = pkgs.nixpkgs-fmt;
    });
}

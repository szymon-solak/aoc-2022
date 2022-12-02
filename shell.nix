let
    # pin nixpkgs - https://status.nixos.org/
    pkgs = import (fetchTarball "https://github.com/nixos/nixpkgs/archive/596a8e828c5dfa504f91918d0fa4152db3ab5502.tar.gz") {};
in
    pkgs.mkShell.override { stdenv = pkgs.clangStdenv; } {
        buildInputs = [
            pkgs.rustc
            pkgs.cargo
            pkgs.rustfmt
            pkgs.libclang
            pkgs.libiconv
        ];

        # Point bindgen to where the clang library would be
        LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";

        # For Rust language server and rust-analyzer
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

        shellHook = ''
        '';
    }


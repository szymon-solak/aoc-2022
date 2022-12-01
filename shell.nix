let
    # pin nixpkgs
    # pkgs = import (fetchTarball "https://github.com/nixos/nixpkgs/archive/5658fadedb748cb0bdbcb569a53bd6065a5704a9.tar.gz") {};
    pkgs = import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/refs/tags/22.05.zip") {};
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


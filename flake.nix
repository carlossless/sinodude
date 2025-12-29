{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, utils, naersk, rust-overlay }:
    utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];

        pkgs = import nixpkgs {
          inherit system overlays;
        };

        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        naersk' = pkgs.callPackage naersk {
          cargo = toolchain;
          rustc = toolchain;
          clippy = toolchain;
        };

        buildInputs = with pkgs; [
          pkg-config
          libusb1
          binutils
        ] ++
        (lib.optionals (stdenv.hostPlatform.isLinux) [
          udev
        ]) ++
        (lib.optionals (stdenv.hostPlatform.isDarwin) [
          iconv
        ]);
      in
      {
        formatter = pkgs.nixpkgs-fmt;

        packages = {
          # For `nix build` `nix run`, & `nix profile install`:
          default = naersk'.buildPackage {
            pname = "sinodude";
            version = "latest";

            src = ./.;
            doCheck = true; # run `cargo test` on build

            inherit buildInputs;

            meta = with pkgs.lib; {
              description = "A utility for interfacing with sinowealth programming tools e.g. sinolink";
              homepage = "https://github.com/carlossless/sinodude";
              license = licenses.mit;
              mainProgram = "sinodude";
              maintainers = with maintainers; [ carlossless ];
            };
          };
        };

        devShells.default = pkgs.mkShell {
          inherit buildInputs;
          nativeBuildInputs = with pkgs; [ rustup toolchain ];
        };
      }
    );
}

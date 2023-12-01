{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux"];

      perSystem = {
        config,
        pkgs,
        system,
        lib,
        self',
        ...
      }: {
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [(import inputs.rust-overlay)];
        };

        formatter = pkgs.alejandra;

        packages = {
          stable-rust-toolchain = pkgs.rust-bin.stable.latest.minimal.override {
            extensions = ["rust-src" "clippy"];
          };

          nightly-rust-toolchain = pkgs.rust-bin.selectLatestNightlyWith (toolchain:
            toolchain.minimal.override {
              extensions = ["rust-src" "clippy"];
            });

          nightly-rustfmt = pkgs.rust-bin.selectLatestNightlyWith (toolchain:
            toolchain.minimal.override {
              extensions = ["rustfmt"];
            });

          nightly-rust-analyzer = pkgs.rust-bin.selectLatestNightlyWith (toolchain:
            toolchain.minimal.override {
              extensions = ["rust-analyzer"];
            });

          # Wrap cargo udeps to use the nightly toolchain
          cargo-udeps = pkgs.stdenv.mkDerivation {
            name = "cargo-udeps";
            buildInputs = [pkgs.makeWrapper];
            buildCommand = ''
              mkdir -p $out/bin
              ln -s ${pkgs.cargo-udeps}/bin/cargo-udeps $out/bin/cargo-udeps-unwrapped
              wrapProgram $out/bin/cargo-udeps-unwrapped \
                --prefix PATH ":" "${config.packages.nightly-rust-toolchain}/bin"
              mv $out/bin/cargo-udeps-unwrapped $out/bin/cargo-udeps
            '';
          };
        };

        devShells.default = pkgs.mkShell {
          packages = [
            config.formatter
            config.packages.stable-rust-toolchain
            config.packages.nightly-rust-analyzer
            config.packages.nightly-rustfmt
            config.packages.cargo-udeps
            pkgs.cargo-nextest
            pkgs.fd
            pkgs.hyperfine
            pkgs.just
            pkgs.nodePackages.prettier
            pkgs.cargo-outdated
            pkgs.taplo
          ];
        };
      };
    };
}

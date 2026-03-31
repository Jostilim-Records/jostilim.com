{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs@{ self, nixpkgs, flake-parts, fenix, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" "aarch64-darwin" ];
      perSystem = { pkgs, system, ... }:
      let 
        f = with fenix.packages.${system}; combine [
          default.toolchain
          targets.wasm32-unknown-unknown.latest.rust-std
        ];
      in {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            f
            trunk
            leptosfmt
          ];
        };
      };
    };
}

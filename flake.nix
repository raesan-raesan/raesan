{
  description = "a flake for raesan dev environment";
  inputs = {
    nixpkgs.url =
      "github:nixos/nixpkgs/6c90912761c43e22b6fb000025ab96dd31c971ff";
    deno_2_1_4-pkgs.url =
      "github:nixos/nixpkgs/4989a246d7a390a859852baddb1013f825435cee";
	  rust_1_78_0-pkgs.url = "github:nixos/nixpkgs/b60793b86201040d9dee019a05089a9150d08b5b";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-utils, ... }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = import nixpkgs { inherit system; };
      in {
        formatter =
          pkgs.nixfmt-classic; # formatter for .nix files, just run `nix fmt .` to format the entire directory
        devShell = pkgs.mkShell {
          packages = [
            inputs.deno_2_1_4-pkgs.legacyPackages.${system}.deno
			inputs.rust_1_78_0-pkgs.legacyPackages.${system}.rustc
			inputs.rust_1_78_0-pkgs.legacyPackages.${system}.cargo
			inputs.rust_1_78_0-pkgs.legacyPackages.${system}.rustfmt
			inputs.rust_1_78_0-pkgs.legacyPackages.${system}.diesel-cli
			inputs.rust_1_78_0-pkgs.legacyPackages.${system}.sqlite
          ];
        };
      });
}

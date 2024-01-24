{
    description = "A flake for holding some workflow shortcuts with sf client";

    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    };

    outputs = { self, nixpkgs, ... }: 
    let
        system = "x86_64-linux";
        pkgs = import nixpkgs {
            inherit system;
        };
    in {
        packages.${system}.default = pkgs.stdenv.mkDerivation {
            name = "force";
            src = ./salesforce_interface;
            phases = [ "installPhase" ];
            installPhase = ''
                mkdir -p $out/bin
                cp $src $out/bin/force
                chmod +x $out/bin/force
            '';
        };
    };
}

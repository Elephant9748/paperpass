{
  description = "Paperpass password manager flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        formattedDate = builtins.readFile (
                pkgs.runCommand "date" {
                        buildInputs = [ pkgs.coreutils ];
                }
                "echo -n $(date -d @${builtins.toString self.lastModified} +'%Y-%m-%d %H:%M:%S') > $out"
        );
        paperpass = pkgs.callPackage ./nix/default.nix {
                gitRev = self.rev or self.dirtyShortRev or "unknown";
                gitLastModified = toString (formattedDate);
        };
      in
      {
        packages.default = paperpass;
        devShells.default = pkgs.mkShell {
          buildInputs = [ paperpass ];
        };
        apps.default = flake-utils.lib.mkApp {
          drv = paperpass;
        };
      }
    );
}


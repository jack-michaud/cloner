{
  description = "Cloner";

  outputs = { self, nixpkgs }: {
    devShell.x86_64-linux = import ./shell.nix {
      pkgs = import nixpkgs {
        system = "x86_64-linux";
      };
    };

    defaultPackage.x86_64-linux = import ./build.nix {
      pkgs = import nixpkgs {
        system = "x86_64-linux";
      };
    };

  };
}

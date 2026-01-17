{
  description = "Simple ModMail bot for Discord";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    {
      self,
      nixpkgs,
    }:
    let
      inherit (nixpkgs) lib;
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      forAllSystems = lib.genAttrs systems;
      nixpkgsFor = nixpkgs.legacyPackages;
    in
    {
      devShells = forAllSystems (
        system:
        let
          pkgs = nixpkgsFor.${system};
        in
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              # rust tools
              clippy
              rustfmt
              rust-analyzer

              # nix tools
              self.formatter.${system}
            ];

            inputsFrom = [ self.packages.${system}.letterbox ];
            RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
          };
        }
      );

      formatter = forAllSystems (system: nixpkgsFor.${system}.nixfmt-tree);

      nixosModules = {
        letterbox = import ./nix/module.nix self;
        letterbox-with-overlay = import ./nix/module-with-overlay.nix self;
      };

      checks = forAllSystems (
        system:
        let
          pkgs = nixpkgsFor.${system};
        in
        {
          module-test = pkgs.testers.nixosTest (
            import ./nix/vm-test.nix { module = self.nixosModules.letterbox; }
          );
        }
      );

      packages = forAllSystems (
        system:
        let
          pkgs = nixpkgsFor.${system};
          packages' = self.packages.${system};

          letterboxPackages = lib.makeScope pkgs.newScope (lib.flip self.overlays.default pkgs);
        in
        {
          inherit (letterboxPackages) letterbox;

          default = packages'.letterbox;
        }
      );

      overlays.default = final: _: {
        letterbox = final.callPackage ./nix/pkgs/letterbox.nix { };
      };
    };
}

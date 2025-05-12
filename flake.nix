{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    crane.url = "github:ipetkov/crane";

    pre-commit-hooks-nix = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    crane,
    pre-commit-hooks-nix,
  }: let
    systems = [
      "x86_64-linux"
      "aarch64-linux"
    ];
  in
    utils.lib.eachSystem systems (system: let
      pkgs = nixpkgs.legacyPackages."${system}";
      craneLib = crane.mkLib pkgs;
      lib = pkgs.lib;

      blueprintFilter = path: _type: builtins.match ".*blp$" path != null;
      xmlFilter = path: _type: builtins.match ".*xml$" path != null;
      jsonFilter = path: _type: builtins.match ".*json$" path != null;
      graphqlFilter = path: _type: builtins.match ".*graphql$" path != null;
      resOrCargo = path: type:
        (graphqlFilter path type) || (jsonFilter path type) || (xmlFilter path type) || (blueprintFilter path type) || (craneLib.filterCargoSources path type);

      src = lib.cleanSourceWith {
        src = ./.;
        filter = resOrCargo;
        name = "source";
      };

      common-args = {
        inherit src;
        strictDeps = true;

        nativeBuildInputs = with pkgs; [
          installShellFiles
          pkg-config
          wrapGAppsHook4
          blueprint-compiler
        ];

        buildInputs = with pkgs; [
          gtk4
          libadwaita
        ];

        postInstall = ''
          installShellCompletion --cmd ppd-gtk \
            --bash ./target/release/build/ppd-gtk-*/out/ppd-gtk.bash \
            --fish ./target/release/build/ppd-gtk-*/out/ppd-gtk.fish \
            --zsh ./target/release/build/ppd-gtk-*/out/_ppd-gtk
          installManPage ./target/release/build/ppd-gtk-*/out/ppd-gtk.1
        '';
      };

      cargoArtifacts = craneLib.buildDepsOnly common-args;

      ppd-gtk = craneLib.buildPackage (common-args
        // {
          inherit cargoArtifacts;
        });

      pre-commit-check = hooks:
        pre-commit-hooks-nix.lib.${system}.run {
          src = ./.;

          inherit hooks;
        };
    in rec {
      checks = {
        inherit ppd-gtk;

        ppd-gtk-clippy = craneLib.cargoClippy (common-args
          // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });

        ppd-gtk-fmt = craneLib.cargoFmt {
          inherit src;
        };

        #ppd-gtk-deny = craneLib.cargoDeny {
        #  inherit src;
        #};

        pre-commit-check = pre-commit-check {
          alejandra.enable = true;
        };
      };
      packages.ppd-gtk = ppd-gtk;
      packages.default = packages.ppd-gtk;

      apps.ppd-gtk = utils.lib.mkApp {
        drv = packages.ppd-gtk;
      };
      apps.default = apps.ppd-gtk;

      formatter = pkgs.alejandra;

      devShells.default = let
        checks = pre-commit-check {
          alejandra.enable = true;
          rustfmt.enable = true;
          clippy.enable = true;
        };
      in
        craneLib.devShell {
          packages = with pkgs; [
            rustfmt
            clippy
            cargo-deny
            cargo-about
            pkg-config
            udev
            libadwaita
            gtk4
            blueprint-compiler
            adwaita-icon-theme
            graphql-client
          ];
          shellHook = ''
            ${checks.shellHook}
          '';
        };
    })
    // {
      hydraJobs = {
        inherit (self) checks packages devShells;
      };
    };
}

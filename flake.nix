{
    inputs = {
        fenix = {
            url = "github:nix-community/fenix";
            inputs.nixpkgs.follows = "nixpkgs";
        };
        nixpkgs.url = "github:NixOS/nixpkgs";
    };

    outputs = {self, fenix, nixpkgs}:
        let pkgs = nixpkgs.legacyPackages.x86_64-linux;
            # rustt = ((builtins.trace fenix.outputs.packages.x86_64-linux fenix).complete.withComponents [
            rustt = (fenix.outputs.packages.x86_64-linux.complete.withComponents [
                        "cargo"
                        "clippy"
                        "rustc"
                        "rustfmt"
                        "rust-src"
                    ]);
        in {
            defaultPackage.x86_64-linux = pkgs.hello;

            devShell.x86_64-linux =
                pkgs.mkShell {
                    buildInputs = [
                        rustt
                        # Helpers
                        pkgs.httpie
                        pkgs.jq
                        pkgs.yq
                        pkgs.dig
                    ];
                };
        };
}

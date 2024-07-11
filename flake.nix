{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };
  outputs =
    { nixpkgs, ... }:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
    in
    {
      devShells.x86_64-linux.default = pkgs.mkShell {
        packages = with pkgs; [
          protobuf
          (python3.withPackages (py-pkgs: [
            py-pkgs.flask
            py-pkgs.protobuf
          ]))
        ];
      };
    };
}

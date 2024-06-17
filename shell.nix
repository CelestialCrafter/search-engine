with import <nixpkgs> {};
mkShell {
  packages = [
		sentencepiece
    (python3.withPackages (python-pkgs: with python-pkgs; [
      torchWithCuda
      yapf
			sentencepiece
    ]))
  ];
}

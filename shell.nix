{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    nodejs
    pnpm
    svelte-language-server
    typescript-language-server
  ];
}

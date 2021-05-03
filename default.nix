let
  pkgs = import <nixpkgs> {};
in pkgs.mkShell {
  #buildInputs = [ pkgs.cargo pkgs.rustfmt pkgs.rust-analyzer pkgs.rustc];
  buildInputs = [ pkgs.cargo pkgs.rustfmt pkgs.rls pkgs.rustc];
}

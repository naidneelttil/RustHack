with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "rust-env";
  nativeBuildInputs = [
     rustc
     cargo
     rustfmt
     rust-analyzer


     ];

  buildInputs = [

  ];
  
  RUST_BACKTRACE = 1;

  shellHook =
    ''
      echo "entering env"
      export GH_TOKEN="$(cat ~/Projects/ghtoken.txt)"
    '';
}

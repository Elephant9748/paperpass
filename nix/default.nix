# nix-build -E 'with import <nixpkgs> {}; callPackage ./nix/default.nix {}'
# with flake just: nix build . --impure

# Guide:
# https://github.com/NixOS/nixpkgs/blob/master/doc/languages-frameworks/rust.section.md

{
  lib,
  fetchFromGitHub,
  rustPlatform,
  git,
  stdenv,
  gitRev ? null,
  gitLastModified ? null,
}:
let
  rev = if gitRev == "" || gitRev == null then "rev" else gitRev;
  date = if gitLastModified == "" || gitLastModified == null then "date" else gitLastModified;
  lockfile = if builtins.pathExists ./Cargo.lock then  ./Cargo.lock else if builtins.pathExists ../Cargo.lock then ../Cargo.lock else null;
in
rustPlatform.buildRustPackage (finalAttrs: {
  pname = "paperpass";
  version = "1.1.12";

  # cargoLock.lockFile = ./Cargo.lock;

  src = ./..;

  # src = fetchFromGitHub {
  #   owner = "Elephant9748";
  #   repo = "paperpass";
  #   # tag = "${finalAttrs.version}";
  #   rev = "refs/heads/main";
  #   hash = "sha256-W0bIsEnA/kmnJTEUeTpTDCEdbNCWawM1tQdGdijvJuY=";
  # };

  # cargoPatches = [ ./0001-cargo-lock.patch ];

  # postPatch = ''
  #       if ! [ -f Cargo.lock ]; then
  #               cargo generate-lockfile
  #       fi
  #       # ln -sf ${./Cargo.lock} Cargo.lock
  #       ls -la
  # '';

  cargoLock.lockFile =  lockfile;

  # inherit gitRev gitLastModified;

  nativeBuildInputs = [ git ];

  preConfigure = ''
        export GIT_HASH="${rev}"
        export DATE="${date}"
        echo "DEBUG: GIT_HASH=$GIT_HASH"
        echo "DEBUG: DATE=$DATE"
        echo "DEBUG: SOURCE_DATE_EPOCH=$SOURCE_DATE_EPOCH"
  '';

  cargoHash = "sha256-94wUp3BG+ZC2quUQtAxbiv3WgAWB2eHvW89n2T6iCKs=";

  meta = {
    description = "paperpass-git password manager in terminal";
    homepage = "https://codeberg.org/rigel254/paperpass";
    changelog = "https://codeberg.org/rigel254/paperpass/releases/tag/${finalAttrs.version}";
    license = lib.licenses.mit;
    maintainers = with lib.maintainers; [
      rigel
    ];
    mainProgram = "${finalAttrs.pname}";
  };
})


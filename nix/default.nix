# nix-build -E 'with import <nixpkgs> {}; callPackage ./paperpass-bin.nix {}'

# Guide:
# https://github.com/NixOS/nixpkgs/blob/master/doc/languages-frameworks/rust.section.md

{
  lib,
  fetchFromGitHub,
  rustPlatform,
  git,
  gitRev,
  gitLastModified
}:

let
        gitInfo = builtins.fetchGit ../.;
        rev = gitInfo.rev or "unkknown";
        buildTime = lib.formatDate "%Y-%m-%d %H:%M:%S" (builtins.currentTime);
in
rustPlatform.buildRustPackage (finalAttrs: {
  pname = "paperpass";
  version = "1.1.11";

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

  nativeBuildInputs = [ git ];

  GIT_HASH = rev;
  DATE =  builtins.toString gitLastModified;
  # env = {
  #       PAPERPASS_GIT_HASH = rev;
  #       PAPERPASS_DATE =  builtins.toString gitLastModified;
  # };

  cargoHash = "sha256-94wUp3BG+ZC2quUQtAxbiv3WgAWB2eHvW89n2T6iCKs=";

  meta = {
    description = "paperpass-git password manager in terminal";
    homepage = "https://github.com/Elephant9748/paperpass";
    changelog = "https://github.com/Elephant9748/paperpass/releases/tag/${finalAttrs.version}";
    license = lib.licenses.mit;
    maintainers = with lib.maintainers; [
      rigel
    ];
    mainProgram = "${finalAttrs.pname}";
  };
})



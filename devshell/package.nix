{
pkgs,
  name,
  version,
  lib,
  rustPlatform,
  installShellFiles,
}:
rustPlatform.buildRustPackage rec {
  pname = name;
  inherit version;

  src = lib.cleanSource ./..;

  cargoLock.lockFile = ../Cargo.lock;

  nativeBuildInputs = with pkgs [
    # pkgs.bacon
    # pkgs.bunyan-rs.out
    # pkgs.cargo-make
    openssl
    glibc

    nixpkgs-fmt
    shellcheck
    rnix-lsp
    # nodePackages.bash-language-server
  # installShellFiles
  ];

  # postInstall = ''
  #   installShellCompletion --cmd seeking-edge \
  #     --bash <($out/bin/tunelo completions bash) \
  #     --fish <($out/bin/tunelo completions fish) \
  #     --zsh  <($out/bin/tunelo completions zsh)
  # '';

  doCheck = false;

  # meta = with lib; {
  #   homepage = "https://github.com/xrelkd/tunelo";
  #   license = with licenses; [gpl3Only];
  #   maintainers = with maintainers; [xrelkd];
  # };
}

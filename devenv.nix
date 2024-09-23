{ pkgs, lib, ... }:

{
  languages.rust = {
    enable = true;
    # https://devenv.sh/reference/options/#languagesrustchannel
    channel = "nixpkgs";

    components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" ];
  };
}
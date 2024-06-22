default:
  @just --list --justfile {{justfile()}}

build:
  cargo build --bin=portunus-authkeys-cmd
  cargo build --bin=portunus-keycached

#!/usr/bin/env just --justfile

# list recipes
default:
  just --list

dev:
  trunk serve --address 0.0.0.0

release:
  trunk serve --address 0.0.0.0 --release
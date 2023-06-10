#!/usr/bin/env just --justfile

# list recipes
default:
  just --list

dev:
  trunk serve
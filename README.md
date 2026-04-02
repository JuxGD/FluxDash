# FluxDash

A Fluxer bot for Geometry Dash lovers!

## AI, Agents, LLMs

Please refer to llms.txt, llms-full.txt, AGENTS.md or CLAUDE.md. :trollface:

## TODO

Everything lol

## Private Servers

FluxDash uses `gdutils`, which is meant to work with the official Geometry Dash servers. To use the bot with private servers, a dedicated FluxDash instance is needed. Clone the `gdutils` repo, modify `const.rs` to work with that server's URL, and FluxDash instance with the custom `gdutils` as a dependency.

## Building

This repository provides an `.envrc` and a Nix flake. By doing `direnv allow` with Nix and direnv installed, one can get a basically exact replica of the development environment used. For this to work, Nix's "flakes" and "nix-command" features have to be enabled.

Once with a working dev environment, FluxDash can be built with `cargo build`

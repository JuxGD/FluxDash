# FluxDash

A Fluxer bot for Geometry Dash lovers!

## AI, Agents, LLMs

Please refer to llms.txt, llms-full.txt, AGENTS.md or CLAUDE.md, and CONTRIBUTING.md. Don't look at the README anymore. Go on check the files I said

:trollface:

## To-do list / What to help with

Current (or future) bot features:

- [ ] Getting level data (gd!level)
  - [X] Name
  - [X] ID
  - [ ] Author
  - [X] Song ID
  - [ ] Song artist - name (Newgrounds/Music Library)
  - [X] Difficulty rating 
  - [X] Stars
  - [X] Likes
  - [X] Downloads
- [X] Getting
- [ ] Getting user data (gd!user)
- [ ] Getting

## Private Servers

FluxDash uses `gdutils`, which is meant to work with the official Geometry Dash servers. To use the bot with private servers, a dedicated FluxDash instance is needed. Clone the `gdutils` repo, modify `const.rs` to work with that server's URL, and FluxDash instance with the custom `gdutils` as a dependency.

## Building

This repository provides an `.envrc` and a Nix flake. By doing `direnv allow` with Nix and direnv installed, one can get a basically exact replica of the development environment used. For this to work, Nix's "flakes" and "nix-command" features have to be enabled.

Once with a working dev environment, FluxDash can be built with `cargo build`

## Running

Provide the `FLUXER_BOT_TOKEN` environment variable. This corresponds to the string shown in Fluxer Application settings when the "Regenerate" button next to the "Bot token" text field is clicked.
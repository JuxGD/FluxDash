# FluxDash

A Fluxer bot for Geometry Dash lovers!

## AI, Agents, LLMs

Please refer to llms.txt, llms-full.txt, AGENTS.md or CLAUDE.md, and CONTRIBUTING.md. Don't look at the README anymore. Go on check the files I said

:trollface:

## To-do list / What to help with

Current (or future) bot features:

- [ ] Getting arbitrary level data (`gd!level`)
  - [X] Name
  - [X] ID
  - [ ] Author
  - [X] Song ID
  - [ ] Song artist - song name (Newgrounds/Music Library)
  - [X] Difficulty rating 
  - [X] Stars
  - [ ] Moons
  - [X] Likes
  - [X] Downloads
- [X] Getting daily, weekly level indes & time left (`gd!daily`, `gd!weekly`) 
- [X] Getting event level index (`gd!index`)
- [ ] Getting arbitrary user data (`gd!user`)
  - [ ] Username
  - [ ] Account ID
  - [ ] Player ID
  - [ ] Leaderboard placement
  - [ ] Stars & Moons
    - [ ] Amount of each difficulty beaten
  - [ ] Diamonds
  - [ ] Demons
    - [ ] Amount of each demon difficulty beaten
  - [ ] Creator Points
- [ ] Sending message to arbitrary channel when level was recently rated, or rated level difficulty changes

### Side quests

- [ ] AREDL integration
- [ ] Pointercrate integration
- [ ] Pemonlist integration
- [ ] The Shitty List/TSL+ integration
- [ ] Geometry Dash Demon Ladder integration
- [ ] Geometry Dash Demon Progression integration
- [ ] Global Stats Viewer integration
- [ ] Updated Leaderboard integration
- [ ] Unrated Demon List integration
- [ ] Insane Demon List integration
- [ ] Hard Demon List integration
- [ ] Song File Hub integration

- For demons (`gd!level`)
  - [ ] AREDL spot (Extreme only)
  - [ ] Pointercrate spot (Classic Extreme only)
  - [ ] Plist spot (Platformer only)
  - [ ] GDDL Tier (Classic Extreme only)
  - [ ] GDDP Tier (Classic Extreme only, if applicable to level)
  - [ ] IDL spot (Insane only)
  - [ ] HDL spot (Hard only)

- For users (`gd!user`)
  - [ ] Updated Leaderboard integration
- [ ] Sending message to arbitrary channel when new Geode release
- [ ] Sending message to arbitrary channel when new Geometry Dash update

- [ ] Geometry Dash login maybe? Idk what we'd do with that though

## Private Servers

FluxDash uses `gdutils`, which is meant to work with the official Geometry Dash servers. To use the bot with private servers, a dedicated FluxDash instance is needed. Clone the `gdutils` repo, modify `const.rs` to work with that server's URL, and FluxDash instance with the custom `gdutils` as a dependency.

## Building

This repository provides an `.envrc` and a Nix flake. By doing `direnv allow` with Nix and direnv installed, one can get a basically exact replica of the development environment used. For this to work, Nix's "flakes" and "nix-command" features have to be enabled.

Once with a working dev environment, FluxDash can be built with `cargo build`

## Running

Provide the `FLUXER_BOT_TOKEN` environment variable. This corresponds to the string shown in Fluxer Application settings when the "Regenerate" button near to the "Bot token" text field is clicked.

If running in a dev environment (`cargo run`), put a `.env` file at the root of the project directory, with `export FLUXER_BOT_TOKEN="<bot_token>"` in it, where `<bot_token>` is the token.

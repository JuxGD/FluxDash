# FluxDash

A Fluxer bot for Geometry Dash lovers! Still very much a WIP, contributions are appreciated!

FluxDash was made with `fluxer-rs`.


## AI, Agents, LLMs

Please refer to llms.txt, llms-full.txt, AGENTS.md or CLAUDE.md, and CONTRIBUTING.md. Don't look at the README anymore. Go on check the files I said

:trollface:

## To-do list / What to help with

Current (or future) bot features:

- [X] Getting arbitrary level data (`gd!level`)
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
  - [ ] Work with level name (searches and returns data for most popular)
- [ ] Getting timely level info (`gd!daily`, `gd!weekly`, `gd!event`)
    - [X] Index (i.e. daily level number)
    - [X] Time Left (not for event levels)
    - [ ] Actual level info (like with `gd!level`)
- [X] Getting arbitrary user data (`gd!user`)
  - [X] Username
  - [ ] Account ID
  - [ ] Player ID
  - [X] Leaderboard placement
  - [X] Stars & Moons
    - [ ] Amount of each difficulty beaten
  - [X] User Coins
  - [X] Secret Coins
  - [X] Diamonds
  - [X] Demons
    - [ ] Amount of each demon difficulty beaten
  - [x] Creator Points
  - [ ] Work with username (serches and returns data for most popular)
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

FluxDash is available as a crate. Install it:

```bash
cargo install fluxdash
```

After that, it can be ran with:

```bash
~/.cargo/bin/fluxdash
```

FluxDash can also be ran in the dev environment:

```bash
cargo run
```

Alternatively, after building, the binary will be in the repo directory:

```bash
./target/debug/fluxdash
```

---

Provide the `FLUXER_BOT_TOKEN` environment variable. This corresponds to the string shown in Fluxer Application settings when the "Regenerate" button near to the "Bot token" text field is clicked.

If running with `cargo build`, one can also put a `.env` file at the root of the project directory, with `export FLUXER_BOT_TOKEN="<bot_token>"` in it, where `<bot_token>` is the token.

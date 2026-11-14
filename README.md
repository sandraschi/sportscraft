# SportsCraft

![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)
![Rust 1.90+](https://img.shields.io/badge/rust-1.90%2B-orange?logo=rust)
![MCP stdio](https://img.shields.io/badge/MCP-stdio-purple)
![ArtCraft harness](https://img.shields.io/badge/ArtCraft-harness-green)

**SportsCraft** is a local spectator + personal-trainer sports studio. It covers
European soccer as a spectator (demo tables, fixtures, match predictor) and
works as a personal trainer in the gym or at home (exercise library, weekly
plans, coached single workouts) — on top of the Banister fitness/fatigue/form
lab, race pacing guidance, and IMU gait kinematics. Everything runs on your
machine: no account, no cloud, no network port.

## What works

- **Spectator soccer (European, demo)** — five leagues (Premier League, La Liga,
  Bundesliga, Serie A, Ligue 1) with standings, upcoming fixtures, and an
  Elo-style match predictor. All output says `source: "demo"`: no live feed.
- **Personal trainer (gym or home, demo)** — 23-exercise catalog filtered by
  location, weekly plans for 2-5 days across strength / hypertrophy /
  conditioning / general goals and beginner / intermediate / advanced levels,
  and single coached workouts (warm-up, main, cooldown, TRIMP estimate, cues).
  Guidance is general fitness information, not medical advice.

- **Banister impulse response** — CTL (fitness, 42 d), ATL (fatigue, 7 d) and
  TSB (form = CTL - ATL) folded from a daily TRIMP series, all in TRIMP points.
- **Race pacing prediction** — finish time and min/km pace for 800 m to 100 km
  from current chronic fitness, with the race-ready TSB window (`+5 .. +18`).
- **Gait kinematics** — cadence, vertical oscillation, ground contact time, leg
  spring stiffness and asymmetry. **Demo fixture**: no sensor or file importer
  is attached yet, and the output says `source: "demo"`.
- **MCP server over stdio** — eleven tools, no TCP port:
  `status`, `athlete.profile`, `athlete.predict`, `gait.analyze`,
  `soccer.leagues`, `soccer.standings`, `soccer.fixtures`, `soccer.predict`,
  `trainer.catalog`, `trainer.plan`, `trainer.session`.
- **Desktop studio** (`eframe`/`egui`) — **Spectator** section (league tables,
  fixtures, predictor) and **Personal** section (**Trainer** for gym/home plans
  and workouts, **Lab** for Banister, gait, pacing simulator), theme toggle,
  and a locale selector for EN / DE / FR / ES / JA / KO / ZH.

## What is not implemented

Be aware before installing:

- No FIT/GPX/TCX import. The training history is a deterministic demo block
  (`history_source: "demo"`).
- No persistence: sessions are not stored, and there is no workout queue.
- No PDF export. The File menu exports JSON and copies JSON to the clipboard.
- No authentication. There is no account or password.

## Workspace

| Crate | Role |
|---|---|
| `crates/sportscraft-kinematics` | Banister solver, gait model (no CLI/MCP/UI deps) |
| `crates/sportscraft-core` | Pacing prediction, soccer + trainer engines, i18n, `Command::dispatch` bus |
| `crates/sportscraft-mcp` | MCP `2024-11-05` stdio server (`sportscraft-mcp.exe`) |
| `crates/sportscraft-app` | Desktop studio (`sportscraft.exe`) and CLI (`sportscraft-cli.exe`) |

The desktop app calls core in-process. There is no sidecar, no HTTP server, and
no webapp.

## Build and run

Requires Rust (stable) only. No Python, Node, or `uv`.

```powershell
# Full gate set: clippy, fmt, tests, MCP stdio probe, app smoke
just ci

# Desktop app
cargo run -p sportscraft-app --bin sportscraft

# CLI
cargo run -p sportscraft-app --bin sportscraft-cli -- status
cargo run -p sportscraft-app --bin sportscraft-cli -- profile ATHLETE-PRO
cargo run -p sportscraft-app --bin sportscraft-cli -- predict ATHLETE-PRO 42195
cargo run -p sportscraft-app --bin sportscraft-cli -- soccer-leagues
cargo run -p sportscraft-app --bin sportscraft-cli -- soccer-standings ENG1
cargo run -p sportscraft-app --bin sportscraft-cli -- trainer-catalog home

# MCP server (stdio; logs on stderr, JSON-RPC on stdout)
cargo run -p sportscraft-mcp
```

`start.ps1` wraps the desktop build/run; `start.bat` is the double-click
shim.

## MCP client configuration

```json
{
  "mcpServers": {
    "sportscraft": {
      "command": "D:\\Dev\\repos\\sportscraft\\target\\release\\sportscraft-mcp.exe",
      "args": []
    }
  }
}
```

See [`docs/TOOLS.md`](docs/TOOLS.md) for the tool schemas and
[`docs/DEVELOPMENT.md`](docs/DEVELOPMENT.md) for the release convention.

## Notes on the model

TSB is `CTL - ATL` in TRIMP points. Both impulses use weight 1.0 so the
race-ready window (`+5 .. +18`) is meaningful; a 21-day taper from a hard block
reaches it. With no taper, a hard 14-day demo block sits around `-53`, and the
UI shows the target range so the gap is visible rather than hidden.

## Status — does today / next week / non-goals

**Does today:** Spectator soccer (5 European leagues, tables, fixtures,
predictor) + Personal trainer (gym/home catalog, weekly plans, coached
workouts) + Banister CTL/ATL/TSB lab + race pacing (800 m–100 km) + IMU gait;
all demo output labeled `source: demo`; trainer carries the not-medical-advice
line; NSIS installer posture; 30 test fns.

**Next week:** golden Banister fixtures (known TRIMP → exact CTL/ATL); CSV
importer decision + source labeling; banister-sim recipe.

**Non-goals:** not medical advice (pacing + trainer guidance carry the line); no
live scores feed, no wearable auto-sync yet.

## Lineage — the Storytold craft-app pattern

Storytold ArtCraft ([storytold/artcraft](https://github.com/storytold/artcraft) — "the IDE for artists") was the starting point. The 16 craft apps in the Storytold stable each ship one binary that is both the human CLI and the agent MCP server (stdio), headless-first, with `run.json` + report artifacts. That modus operandi and harness is what this repo implements; the code here is independent, the pattern is shared.

# Changelog

All notable changes to SportsCraft. Version numbers live in `Cargo.toml`
(`[workspace.package] version`) and are mirrored by `installer/installer.nsi`;
see `docs/DEVELOPMENT.md` for the release convention.

## 0.2.0 - 2026-11-06

### Added
- **Spectator section (European soccer, demo):** five leagues (ENG1, ESP1, GER1,
  ITA1, FRA1) with standings, fixtures, and an Elo-style match predictor. New
  `Command` arms `SoccerLeagues`, `SoccerStandings`, `SoccerFixtures`,
  `SoccerPredict`; new MCP tools `soccer.leagues`, `soccer.standings`,
  `soccer.fixtures`, `soccer.predict` (plus underscore aliases); matching CLI
  subcommands; Spectator tab in the desktop studio with league selector,
  standings grid, fixtures, and predictor. All output carries
  `"source": "demo"`.
- **Personal → Trainer section (gym or home, demo):** 23-exercise catalog with
  equipment and gym/home/both locations, weekly plans (2-5 days) for strength /
  hypertrophy / conditioning / general across three levels, and single coached
  workouts with warm-up, cooldown, TRIMP estimate, and cues. New `Command` arms
  `TrainerCatalog`, `TrainerPlan`, `TrainerSession`; new MCP tools
  `trainer.catalog`, `trainer.plan`, `trainer.session` (plus underscore
  aliases); matching CLI subcommands; Trainer tab with goal/location/level/day
  controls, plan + session views, and the exercise library. Every trainer
  payload carries the general-fitness disclaimer (not medical advice).
- Desktop studio now opens on two top-level sections: **Spectator** and
  **Personal** (Trainer + Banister/Gait/Pacing lab). About dialog and `README`
  describe what exists; `--smoke` also asserts the soccer and trainer demos.
- 11 new tests (soccer leagues/tables/errors/predictor, trainer catalog/plan/
  session/dispatch labels); suite grows 16 → 30 test fns.

## 0.1.1 - 2026-11-05

### Fixed
- `sportscraft-mcp` is now a real MCP server: MCP `2024-11-05` JSON-RPC over
  stdio with `initialize`, `notifications/initialized`, `tools/list`,
  `tools/call`, and `ping`. It previously printed a status banner and exited
  without reading stdin, while `README.md` advertised "FastMCP on port 14131".
  There is no TCP port; the ArtCraft standard runs MCP over stdio.
- Four tools are registered and backed by real code: `status`,
  `athlete.profile`, `athlete.predict`, `gait.analyze` (dotted and underscore
  aliases route to the same arm).
- Banister form (TSB) is on the same TRIMP-point scale as CTL/ATL: the default
  fatigue impulse weight was 1.8, which made TSB permanently around
  `-0.8 * CTL` and made the advertised `+5 .. +18` race-ready window
  unreachable (`-116.9` on the demo block). It is now 1.0, so a taper can reach
  the window; a regression test covers it.
- `predict_race_pacing` returns `Result` and rejects zero, negative, or
  non-finite distances instead of emitting nonsense.
- Desktop UI no longer has fake actions: "Export Pacing Plan (JSON)" writes a
  real file and reports the path, "Copy Pacing Plan" puts real JSON on the
  clipboard, and the unimplemented "Export to PDF" / "Sync FIT/GPX Telemetry"
  entries are gone.
- The login modal ("Basic Auth") accepted any non-empty username with any
  password and gated nothing; it has been removed rather than left as a fake
  security boundary.
- The i18n tables are now actually used: every UI string goes through
  `i18n::t`, so the locale selector changes the interface (EN, DE, FR, ES, JA,
  KO, ZH).
- `README.md` and the About dialog claim only what exists (no port, no
  database persistence, no PDF export, no FIT/GPX import).

### Changed
- Added `Command::dispatch` in `sportscraft-core`; the CLI and MCP surfaces now
  share one command bus.
- Demo data is self-declared: `history_source: "demo"` on profiles/status and
  `source: "demo"` on gait output.
- Dropped unused dependencies (`rusqlite`, `chrono`, `tokio`, and the
  `databasecraft-store` / `databasecraft-core` path dependencies). The workspace
  now builds standalone; before this change a fresh clone failed because it
  required the sibling `databasecraft` checkout for crates nothing imported.
- Workspace `[workspace.lints.clippy]` denies `unwrap_used`, `expect_used`,
  `panic`, `todo`, `unimplemented`, mirrored per crate.

### Added
- `justfile` with `check`, `test`, `fmt-check`, `stdio-probe`, `app-smoke`,
  `mcp-smoke`, `ci`, and `installer` recipes.
- `Cargo.lock`-backed release build and a current-user `installer/installer.nsi`.
- `AGENTS.md`, `docs/DEVELOPMENT.md`, `docs/TOOLS.md`, `LICENSE-MIT`,
  `LICENSE-APACHE`.

## 0.1.0 - 2026-11-04

- Initial scaffold: kinematics (Banister, demo gait), core (pacing, i18n),
  stdio placeholder, `eframe` desktop UI, CLI.

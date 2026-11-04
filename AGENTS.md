# SportsCraft - Agent Map

Pure-Rust ArtCraft workspace. Four crates, two binaries for humans, one MCP
server. Read this before changing anything; the standards that govern it live in
`D:\Dev\repos\mcp-central-docs\standards\RUST_ARTCRAFT_STANDARD.md`.

## Shape

| Path | What it is | May depend on |
|---|---|---|
| `crates/sportscraft-kinematics` | Banister solver + gait fixture | `serde` only |
| `crates/sportscraft-core` | pacing, i18n, `Command::dispatch` | kinematics, serde, serde_json, thiserror |
| `crates/sportscraft-mcp` | MCP stdio server | core, serde_json |
| `crates/sportscraft-app` | `eframe`/`egui` studio + CLI | core, eframe, egui, clap, serde_json |

The seam matters: core never imports `clap`, `eframe`, or a transport crate, and
the app never shells out to the MCP binary or opens a socket.

## Rules that bite

- **No `unwrap()`, `expect()`, `panic!`, `todo!`, `unimplemented!`.** Enforced by
  `[workspace.lints.clippy]` and per-crate `#![deny(...)]`. This includes tests.
- **One capability = one `Command` variant + one dispatch arm + one test.** The
  CLI, MCP `tools/call`, and any future UI surface all go through
  `Command::dispatch`; do not add a second path.
- **stdout is JSON-RPC only** in `sportscraft-mcp`. Logs go to `stderr`.
- **No TCP port.** The fleet port registry row for 14131 is stale; Rust ArtCraft
  repos run MCP over stdio and the app in-process.
- **Demo data must say so.** Profiles carry `history_source: "demo"`; gait
  carries `source: "demo"`. Never let a fixture look live.
- **No author-machine paths in runtime code.** Derive paths from
  `std::env::current_dir()` / `dirs` / env overrides.
- **Scores and gates:** `just ci` is the single entry point. A release that has
  not run it is uncertified.

## Gates

```powershell
just check        # clippy -D warnings
just fmt-check    # cargo fmt --all --check
just test         # 16 tests
just stdio-probe  # byte-exact MCP child-process probe
just app-smoke    # desktop headless smoke
just ci           # all of the above
```

## Versioning

`[workspace.package] version` is the release number; `installer/installer.nsi`
mirrors it; `CHANGELOG.md` gets a new top section per release. See
`docs/DEVELOPMENT.md`.

# SportsCraft

**SportsCraft** is a local, Pure-Rust biomechanics and training-load studio. It
models Banister fitness/fatigue/form, turns current chronic fitness into race
pacing guidance, and shows IMU gait kinematics. Everything runs on your machine:
no account, no cloud, no network port.

## What works

- **Banister impulse response** — CTL (fitness, 42 d), ATL (fatigue, 7 d) and
  TSB (form = CTL - ATL) folded from a daily TRIMP series, all in TRIMP points.
- **Race pacing prediction** — finish time and min/km pace for 800 m to 100 km
  from current chronic fitness, with the race-ready TSB window (`+5 .. +18`).
- **Gait kinematics** — cadence, vertical oscillation, ground contact time, leg
  spring stiffness and asymmetry. **Demo fixture**: no sensor or file importer
  is attached yet, and the output says `source: "demo"`.
- **MCP server over stdio** — four tools, no TCP port:
  `status`, `athlete.profile`, `athlete.predict`, `gait.analyze`.
- **Desktop studio** (`eframe`/`egui`) — Banister panel, gait panel, pacing
  simulator, theme toggle, and a locale selector for EN / DE / FR / ES / JA / KO
  / ZH.

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
| `crates/sportscraft-core` | Pacing prediction, i18n, `Command::dispatch` bus |
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

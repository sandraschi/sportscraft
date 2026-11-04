# Tools

SportsCraft exposes the same capability surface three ways: the MCP stdio server,
the CLI, and the desktop UI. All of them call `sportscraft_core::Command::dispatch`.

## MCP server (`sportscraft-mcp.exe`)

Transport: MCP `2024-11-05` JSON-RPC over **stdio**. No TCP port. Logs go to
stderr; stdout carries only JSON-RPC frames.

### `status`

No arguments. Engine version, Banister time constants, the race-ready TSB window,
the command list, and the demo-data marker.

```json
{"status":"ready","engine":"sportscraft","version":"0.1.1",
 "fitness_tau_days":42.0,"fatigue_tau_days":7.0,
 "optimal_tsb_window":[5.0,18.0],"history_source":"demo"}
```

### `athlete.profile`

| Argument | Type | Required | Default |
|---|---|---|---|
| `athlete` | string | no | `ATHLETE-ELITE-1` |

Returns the Banister profile: CTL (`current_fitness`), ATL (`current_fatigue`),
TSB (`form_tsb`), the time constants and impulse weights, and
`history_source: "demo"`.

### `athlete.predict`

| Argument | Type | Required |
|---|---|---|
| `distance_m` | number (`> 0`) | yes |
| `athlete` | string | no (default `ATHLETE-ELITE-1`) |

Returns `target_distance_m`, `predicted_time_seconds`,
`recommended_pace_min_km`, and `optimal_form_tsb_window`. A zero, negative, or
non-finite distance returns `success: false` with an error string.

### `gait.analyze`

No arguments. Cadence, vertical oscillation, ground contact time, leg spring
stiffness, asymmetry, and `source: "demo"` (the values are a fixture; no sensor
is attached).

Underscore aliases (`athlete_profile`, `athlete_predict`, `gait_analyze`) route to
the same handlers as the dotted names.

## CLI (`sportscraft-cli.exe`)

```
sportscraft-cli status
sportscraft-cli profile  <athlete>
sportscraft-cli predict  <athlete> <distance_m>
sportscraft-cli gait
```

Each command prints the same JSON payload as the MCP tool (pretty-printed).

## Desktop UI

- **File** — export the pacing plan as JSON (real file, real path reported) or
  copy it to the clipboard.
- **Distances** — 5K / 10K / half / marathon / 100K presets.
- **View** — light/dark theme.
- **Help** — About (version, transport, what is and is not implemented).
- **Locale selector** — EN / DE / FR / ES / JA / KO / ZH; every UI string is
  localized through `sportscraft_core::i18n`.

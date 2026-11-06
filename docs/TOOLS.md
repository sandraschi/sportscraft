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
{"status":"ready","engine":"sportscraft","version":"0.2.0",
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

### `soccer.leagues`

No arguments. The five demo European leagues: Premier League (`ENG1`),
La Liga (`ESP1`), Bundesliga (`GER1`), Serie A (`ITA1`), Ligue 1 (`FRA1`).
Returns `source: "demo"`.

### `soccer.standings`

| Argument | Type | Required |
|---|---|---|
| `league` | string (`ENG1`, `ESP1`, `GER1`, `ITA1`, `FRA1`) | yes |

Demo table: position, played/won/drawn/lost, goals, points (`3*W+D`), last-five
form. Unknown codes return `success: false`. Returns `source: "demo"`.

### `soccer.fixtures`

| Argument | Type | Required |
|---|---|---|
| `league` | string (same codes) | yes |

Demo next-matchday fixtures with kickoff labels and venues. Unknown codes
return `success: false`. Returns `source: "demo"`.

### `soccer.predict`

| Argument | Type | Required |
|---|---|---|
| `home` | string (team name, e.g. `Man City`) | yes |
| `away` | string (team name, e.g. `Arsenal`) | yes |

Elo-style demo predictor from the bundled ratings plus home edge: home/draw/away
percentages, expected goals, and a pick. Unknown team names return
`success: false`. Returns `source: "demo"`.

### `trainer.catalog`

| Argument | Type | Required |
|---|---|---|
| `location` | string (`gym` or `home`) | yes |

Exercise library filtered by location (`gym` sees `gym`+`both`, `home` sees
`home`+`both`): muscle, equipment, sets/reps/rest, coaching cues. Returns
`source: "demo"` plus the general-fitness disclaimer.

### `trainer.plan`

| Argument | Type | Required |
|---|---|---|
| `goal` | string (`strength`, `hypertrophy`, `conditioning`, `general`) | yes |
| `location` | string (`gym` or `home`) | yes |
| `days_per_week` | integer (2-5) | yes |
| `level` | string (`beginner`, `intermediate`, `advanced`) | yes |

Weekly plan: one focused day per training day with exercises, estimated minutes
and TRIMP. Out-of-range day counts return `success: false`. Returns
`source: "demo"` plus the general-fitness disclaimer (not medical advice).

### `trainer.session`

| Argument | Type | Required |
|---|---|---|
| `goal` | string (same four) | yes |
| `location` | string (`gym` or `home`) | yes |
| `level` | string (same three) | yes |
| `minutes` | integer (15-120) | yes |

One coached workout: warm-up, main sets, cooldown, TRIMP estimate, coaching
cues. Out-of-range minutes return `success: false`. Returns `source: "demo"`
plus the general-fitness disclaimer (not medical advice).

Underscore aliases (`athlete_profile`, `athlete_predict`, `gait_analyze`,
`soccer_leagues`, `soccer_standings`, `soccer_fixtures`, `soccer_predict`,
`trainer_catalog`, `trainer_plan`, `trainer_session`) route to the same handlers
as the dotted names.

## CLI (`sportscraft-cli.exe`)

```
sportscraft-cli status
sportscraft-cli profile  <athlete>
sportscraft-cli predict  <athlete> <distance_m>
sportscraft-cli gait
sportscraft-cli soccer-leagues
sportscraft-cli soccer-standings  <league>
sportscraft-cli soccer-fixtures   <league>
sportscraft-cli soccer-predict    <home> <away>
sportscraft-cli trainer-catalog   <location>
sportscraft-cli trainer-plan      <goal> <location> <days_per_week> <level>
sportscraft-cli trainer-session   <goal> <location> <level> <minutes>
```

Each command prints the same JSON payload as the MCP tool (pretty-printed).

## Desktop UI

- **Spectator** — league selector, standings table, fixtures, match predictor
  (all demo data, labeled `source: demo`).
- **Personal → Trainer** — goal / gym-or-home / level / days / minutes controls,
  weekly plan builder, single-workout builder, exercise library. Guidance is
  general fitness information, not medical advice.
- **Personal → Lab** — Banister panel, gait panel, pacing simulator.
- **File** — export the pacing plan as JSON (real file, real path reported) or
  copy it to the clipboard.
- **Distances** — 5K / 10K / half / marathon / 100K presets.
- **View** — light/dark theme.
- **Help** — About (version, transport, what is and is not implemented).
- **Locale selector** — EN / DE / FR / ES / JA / KO / ZH; every UI string is
  localized through `sportscraft_core::i18n`.

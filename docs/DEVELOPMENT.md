# Development

## Prerequisites

Rust stable (`cargo`, `rustc`) only. The release binaries have no runtime
dependencies: no Python, no `uv`, no Node, no WebView2.

Optional: NSIS (`winget install NSIS.NSIS`) to build the installer, and `just`
for the recipe wrappers.

## Gates

```powershell
just check        # cargo clippy --workspace --all-targets --all-features -- -D warnings
just fmt-check    # cargo fmt --all --check
just test         # cargo test --workspace (16 tests)
just stdio-probe  # mcp-central-docs/scripts/Test-RustMcpStdio.ps1 over a real child process
just app-smoke    # target\debug\sportscraft.exe --smoke
just ci           # check + fmt-check + test + stdio-probe + app-smoke
```

The clippy gate denies `unwrap_used`, `expect_used`, `panic`, `todo`, and
`unimplemented`, through both `[workspace.lints.clippy]` and a per-crate
`#![deny(...)]` block. Tests are not exempt.

`stdio-probe` needs `MCD_ROOT` (default `D:\Dev\repos\mcp-central-docs`).

## Version convention

**`[workspace.package] version` in the root `Cargo.toml` owns the release
number.** Nothing else invents a number:

1. bump the workspace version;
2. mirror it in `installer/installer.nsi` (`Name`, `OutFile`, `DisplayName`);
3. add a `CHANGELOG.md` section at the top;
4. `cargo build --release --workspace` updates `Cargo.lock` — commit it.

`installer/installer.nsi` and `Cargo.toml` drifting apart is a defect; the
`assess` Rust checklist compares them.

## Layout rules

- `sportscraft-kinematics` is pure math: `serde` only, no CLI/MCP/UI crates.
- `sportscraft-core` owns the capability surface (`Command` + `dispatch`) and
  must stay transport-agnostic.
- `sportscraft-mcp` is the only place that parses JSON-RPC; it delegates every
  call to `Command::dispatch`.
- `sportscraft-app` renders and calls core in-process. Do not add HTTP.

## Adding a capability

1. add a `Command` variant in `crates/sportscraft-core/src/lib.rs`;
2. add its dispatch arm (return JSON, never panic);
3. add a unit test for the arm;
4. add the MCP tool entry in `tools_list()` and the `call_tool` match arm
   (dotted name plus an underscore alias);
5. add a CLI subcommand mapping to the variant;
6. update `docs/TOOLS.md`, `README.md`, and `CHANGELOG.md`.

## Demo data

There is no FIT/GPX importer. `SportsEngine::demo_trimp_history()` is a fixed
14-day block and `BiomechanicsModel::demo_gait_profile()` is a fixed gait
fixture. Both must be labelled demo in every output (`history_source`,
`source`). Never present them as measured.

## Release

```powershell
just installer
```

Builds `target\release\` and then `dist\SportsCraft-Setup-<version>.exe` with
`makensis /NOCD installer\installer.nsi`. The `/NOCD` flag matters: without it
NSIS resolves `File` paths relative to `installer\` and ships an empty bundle.

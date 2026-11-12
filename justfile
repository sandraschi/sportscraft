set windows-shell := ["powershell.exe", "-NoProfile", "-Command"]

# SportsCraft automation. Every recipe prefixes cargo via USERPROFILE so IDE and
# NSSM shells work even when .cargo\bin is not on the inherited PATH.
mcd := env_var_or_default("MCD_ROOT", "D:\\Dev\\repos\\mcp-central-docs")

default:
    @just --list

# Strict ArtCraft lint gate (deny unwrap/expect/panic/todo/unimplemented)
check:
    $env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"; cargo clippy --workspace --all-targets --all-features -- -D warnings

# Unit + integration + catalog/dispatch tests
test:
    $env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"; cargo test --workspace

# Formatting gate
fmt-check:
    $env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"; cargo fmt --all --check

# Debug build of every binary
build:
    $env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"; cargo build --workspace

# Release build of every binary
build-release:
    $env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"; cargo build --release --workspace

# Byte-exact MCP stdio probe: initialize -> tools/list -> tools/call (fleet script)
stdio-probe: build
    powershell -NoProfile -ExecutionPolicy Bypass -File "{{mcd}}\scripts\Test-RustMcpStdio.ps1" -Exe target\debug\sportscraft-mcp.exe -McpArgs @("--stdio") -ProbeTool status

# Desktop app headless parity check
app-smoke: build
    .\target\debug\sportscraft.exe --smoke

# MCP catalog self-check (no stdio session)
mcp-smoke: build
    .\target\debug\sportscraft-mcp.exe --smoke

# The full 6-gate bar
ci: check fmt-check test stdio-probe app-smoke
    @echo "SportsCraft: all gates green"

# Start the MCP server over stdio (JSON-RPC on stdout, logs on stderr)
run-mcp: build
    .\target\debug\sportscraft-mcp.exe

# CLI examples
run-cli: build
    .\target\debug\sportscraft-cli.exe status
    .\target\debug\sportscraft-cli.exe profile ATHLETE-PRO
    .\target\debug\sportscraft-cli.exe predict ATHLETE-PRO 42195

# Launch the desktop app
run-app: build
    .\target\debug\sportscraft.exe

# Build the current-user NSIS installer (requires NSIS)
installer: build-release
    $nsis = "${env:ProgramFiles(x86)}\NSIS\makensis.exe"; if (-not (Test-Path $nsis)) { throw "makensis not found (winget install NSIS.NSIS)" }; & $nsis /NOCD installer\installer.nsi

# Headless check: app and CLI smoke from scripts/craft.config.psd1 (throwaway database).
smoke:
    powershell -NoProfile -ExecutionPolicy Bypass -File start.ps1 -Smoke

# Build and stage dist/ (plain exes, installer, launcher zip, SHA256SUMS). Nothing is published.
release-dry:
    powershell -NoProfile -ExecutionPolicy Bypass -File scripts/release.ps1

# Same, then tag, push the tag and upload to GitHub (clean, pushed trunk required).
release:
    powershell -NoProfile -ExecutionPolicy Bypass -File scripts/release.ps1 -Publish

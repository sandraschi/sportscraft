# SportsCraft — Tiered Execution Plan

**Parent Standard:** [`RUST_ARTCRAFT_STANDARD.md`](../../mcp-central-docs/standards/RUST_ARTCRAFT_STANDARD.md)  
**Menu Bar Standard:** [`CRAFT_MENU_BAR_STANDARD.md`](../../mcp-central-docs/standards/rust-artcraft/CRAFT_MENU_BAR_STANDARD.md)  
**Architecture Model:** 4-Tier ArtCraft Incremental Delivery  
**Status:** Active  

---

## Architecture Flow

```
Tier 1: Core Engine ──> Tier 2: MCP / CLI ──> Tier 3: Desktop UI ──> Tier 4: Menu Bar, Help & Packaging
(sportscraft-core)     (sportscraft-mcp)     (sportscraft-app)     (Hardened Workstation)
```

---

## Tier Specifications

### Tier 1 — Core Athletic Physiology & Kinetic Engine (`sportscraft-core`)
- [x] L1 `Command::dispatch` universal athletic execution bus.
- [x] FIT, GPX, TCX file parser with elevation, cadence, HR, and power streams.
- [x] Banister Impulse-Response model (CTL, ATL, TSB curves).
- [x] Monod-Scherrer Critical Power and $W'$ anaerobic modeling.
- [x] Biomechanical video pose landmark analysis (21 joint angles).
- [x] `#![forbid(unsafe_code)]` and zero-panic clippy lints.

### Tier 2 — Headless Driver & FastMCP (`sportscraft-mcp`)
- [x] Unified CLI `sportscraft-cli` with `--json` output.
- [x] FastMCP stdio server communicating over JSON-RPC 2.0.
- [x] Activity export and metric calculation tools.
- [x] Headless `--smoke` verification without window initialization.

### Tier 3 — Desktop Workstation UI (`sportscraft-app`)
- [x] Immediate-mode `egui` 0.37 / `wgpu` workstation.
- [x] Video pose playback canvas with joint angle tracking overlays.
- [x] Interactive Banister fatigue graphs and power duration curves.

### Tier 4 — Craft Menu Bar Standard, Help Suite & Hardened Verification
- [ ] Universal 8-menu bar (`File`, `Edit`, `View`, `Insert`, `Modify`, `Commands`, `Window`, `Help`) with `switch_on_hover`.
- [ ] Mandatory 5-dialog Help suite (`Command Palette...`, `Keyboard Shortcuts...`, `Project Files & Docs...`, `Licenses & Notices...`, `About SportsCraft`).
- [ ] Fleet UI/UX Pattern 2 adherence: viewport-centric navigation, top button row mapped to View toggles, panel visibility managed via `View` / `Window`.
- [ ] Tofu Guard verification (100% font glyph coverage, zero tofu placeholder boxes).
- [ ] Headless `--smoke` parity validation.
- [ ] Standalone release binary packaging and NSIS installer.

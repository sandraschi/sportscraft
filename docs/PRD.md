# SportsCraft — Product Requirements Document (PRD)

**Document Identifier:** `CRAFT-PRD-2026-SPORT-001`  
**Version:** 1.1.0  
**Status:** Active  
**Domain:** Biomechanical Computer Vision, Athletic Telemetry, Sovereign Endurance Coaching  
**Standard Port:** 14127  
**Parent Standard:** [`RUST_ARTCRAFT_STANDARD.md`](../../mcp-central-docs/standards/RUST_ARTCRAFT_STANDARD.md)  
**Menu Bar Standard:** [`CRAFT_MENU_BAR_STANDARD.md`](../../mcp-central-docs/standards/rust-artcraft/CRAFT_MENU_BAR_STANDARD.md)  

---

## 1. Executive Summary & Problem Statement

Modern athletic tools (Strava, TrainingPeaks, Whoop, Garmin Connect) have become predatory walled gardens:
- **Paywalled Physiology:** Basic fatigue curves (CTL/ATL/TSB) and power duration models are locked behind monthly subscriptions.
- **Privacy Intrusion:** Sensitive GPS coordinates, home addresses, and daily routes are mined and monetized.
- **Zero Computer Vision Integration:** Wearables track heart rate, but fail to analyze lifting technique, running gait, or kinetic angles.

**The SportsCraft Solution:**
SportsCraft is a local-first, pure-Rust endurance performance lab and computer vision biomechanics atelier:
- Ingests standard FIT, GPX, and TCX activity files with 100% offline parsing.
- Computes the Banister Impulse-Response fatigue model (Fitness CTL, Fatigue ATL, Form TSB) without cloud fees.
- Critical Power modeling (Monod-Scherrer & 3-parameter $W'$) for functional threshold power (FTP) determination.
- Computer vision pose estimation for 21 3D joint landmarks, bar paths, and cadence mechanics.
- Direct integration with DatabaseCraft's `club` blueprint for race brackets, team rosters, and seasonal leaderboards.

---

## 2. Core Functional Requirements

1. **Endurance Physiology Engine (`sportscraft-core`):**
   - FIT/GPX/TCX file parser with elevation, heart rate, cadence, and power streams.
   - Banister Impulse-Response model calculating chronic training load (CTL) and acute training load (ATL).
   - Monod-Scherrer Critical Power and $W'$ anaerobic work capacity analyzer.
2. **Kinetic Biomechanics HUD:**
   - Video pose estimation extracting joint angles, bar velocity, and cadence timing.
   - Form alert triggers for excessive lumbar flexion or asymmetrical pedal stroke.
3. **Athletic Archive & Club Blueprints:**
   - Local SQLite WAL storage keeping GPS traces private.
   - Regatta brackets, race timing, and division rosters.
4. **Universal Command Bus (`Command::dispatch`):**
   - 100% of CLI, MCP, and UI actions dispatch through `sportscraft-core::dispatch`.

---

## 3. UI/UX Architecture & Menu Bar Standard

In accordance with [`CRAFT_MENU_BAR_STANDARD.md`](../../mcp-central-docs/standards/rust-artcraft/CRAFT_MENU_BAR_STANDARD.md):
- **Pattern 2 (Viewport-Centric Athletic Workstation):** Persistent kinetic biomechanics stage, video pose overlay canvas, and endurance fatigue curves (CTL/ATL/TSB). Top quick button row acts as accelerators into the `View` menu. To add a new athletic analysis tool or club inspector:
  1. Register a visibility toggle in the `View` or `Window` menu.
  2. Add an entry in the sidebar Athletes / Workouts toolbox.
  3. Wire the action directly into the universal `Command::dispatch` bus.
- **Top Menu Bar Hierarchy:**
  - `File`: New Session (`Ctrl+N`), Open Activity FIT/GPX (`Ctrl+O`), Open Recent ▸, Save Session (`Ctrl+S`), Save As... (`Ctrl+Shift+S`), Export Analytics / CSV (`Ctrl+E`), Close (`Ctrl+W`), Quit (`Ctrl+Q`).
  - `Edit`: Undo (`Ctrl+Z`), Redo (`Ctrl+Y`), Cut, Copy, Paste, Select All, Preferences (`Ctrl+,`).
  - `View`: Biomechanics HUD, Pose Overlay Viewport, Banister Fatigue Curves, Power Duration Curve, Race Bracket Grid, Theme (`Dark`/`Light`).
  - `Insert`: Add Workout Activity, Insert Video Recording, Add Athlete Profile, Create Race Event.
  - `Modify`: Crop Activity Data, Adjust FTP / Weight, Recalculate Banister Weights, Trim Video Clip.
  - `Commands`: Run Biomechanics Pose Analysis, Compute Critical Power Curve, Audit Form Safety, Sync Local Archive.
  - `Window`: Reset Layout, Video Viewport, Physiology Drawer, Club Roster Dock.
  - `Help` (Mandatory 5-Dialog Suite): `Command Palette...` (`Ctrl+K`), `Keyboard Shortcuts...` (`F1`), `Project Files & Docs...`, `Licenses & Notices...`, `About SportsCraft`.
- **Behavioral Mechanics:** Smooth `switch_on_hover` between top-level menus, zero-unsafe immediate-mode rendering, and headless `--smoke` parity.

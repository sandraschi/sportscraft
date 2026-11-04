# SportsCraft

**SportsCraft** is an ArtCraft-tier sovereign biomechanics, physiological impulse modeling, and athletic training studio that replaces extractive cloud fitness trackers with mathematical precision.

Features:
- **Banister Impulse-Response Engine**: Rigorous modeling of Chronic Training Load (CTL fitness, $\tau_1=42$d), Acute Training Load (ATL fatigue, $\tau_2=7$d), and Training Stress Balance (TSB form).
- **Critical Velocity Pacing Predictor**: Translates current chronic fitness into non-linear pacing guidance for 800m to 100km ultramarathons.
- **IMU Gait Kinematics**: Tracks ground contact time, vertical oscillation, leg spring stiffness, and bilateral stride asymmetry.
- **FastMCP Protocol**: Exposed on port `14131` for multi-agent athletic coaching swarms.

---

## Workspace Structure

1. `crates/sportscraft-kinematics`: Banister differential equation solver, TRIMP load integration, and IMU biomechanical models.
2. `crates/sportscraft-core`: Race pacing regression, database session storage, and workout queue.
3. `crates/sportscraft-mcp`: FastMCP JSON-RPC server on port `14131`.
4. `crates/sportscraft-app`: Native GUI (`eframe`/`egui`) and CLI tool (`sportscraft-cli`).

---

## Verification & Usage

```powershell
# Run test suite
cargo test

# Headless smoke test
./target/debug/sportscraft.exe --smoke

# CLI Telemetry & Pacing
./target/debug/sportscraft-cli.exe profile ATHLETE-PRO
./target/debug/sportscraft-cli.exe predict ATHLETE-PRO 42195.0
./target/debug/sportscraft-cli.exe gait
```

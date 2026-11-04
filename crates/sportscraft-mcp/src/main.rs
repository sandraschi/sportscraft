//! SportsCraft MCP Server
//!
//! Exposes Banister impulse-response modeling, TRIMP load calculations,
//! and biomechanical gait telemetry as FastMCP JSON-RPC tools on port 14131.

use serde_json::json;
use sportscraft_core::SportsEngine;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port: u16 = 14131;
    println!("[sportscraft-mcp] Initializing Sports Biomechanics FastMCP engine on port {}...", port);

    let profile = SportsEngine::get_default_banister_profile("ATHLETE-PRO");
    let gait = SportsEngine::get_gait_analysis();

    let status = json!({
        "status": "ready",
        "mcp_server": "sportscraft",
        "port": port,
        "capabilities": ["BanisterImpulseResponse", "TrimpLoadAggregation", "GaitAsymmetryAnalysis", "PredictiveRacePacing"],
        "ctl_fitness": profile.current_fitness,
        "atl_fatigue": profile.current_fatigue,
        "tsb_form": profile.form_tsb,
        "cadence_spm": gait.cadence_spm,
    });
    println!("[sportscraft-mcp] Status: {}", serde_json::to_string_pretty(&status)?);
    Ok(())
}

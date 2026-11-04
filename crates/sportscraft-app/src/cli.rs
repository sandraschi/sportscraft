use clap::{Parser, Subcommand};
use sportscraft_core::SportsEngine;

#[derive(Parser)]
#[command(name = "sportscraft-cli", about = "SportsCraft Biomechanics & Banister Training Studio CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Inspect athlete Banister fitness, fatigue, and form
    Profile {
        athlete: String,
    },
    /// Predict race finish time and pacing
    Predict {
        athlete: String,
        distance_m: f64,
    },
    /// Inspect IMU gait kinematics
    Gait,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Profile { athlete } => {
            let profile = SportsEngine::get_default_banister_profile(&athlete);
            println!("{}", serde_json::to_string_pretty(&profile)?);
        }
        Commands::Predict { athlete, distance_m } => {
            let profile = SportsEngine::get_default_banister_profile(&athlete);
            let pacing = SportsEngine::predict_race_pacing(&profile, distance_m);
            println!("{}", serde_json::to_string_pretty(&pacing)?);
        }
        Commands::Gait => {
            let gait = SportsEngine::get_gait_analysis();
            println!("{}", serde_json::to_string_pretty(&gait)?);
        }
    }
    Ok(())
}

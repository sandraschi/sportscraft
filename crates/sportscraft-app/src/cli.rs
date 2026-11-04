//! SportsCraft CLI — one subcommand per [`Command`] variant.
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unimplemented)]
#![deny(clippy::todo)]

use clap::{Parser, Subcommand};
use sportscraft_core::Command;

#[derive(Parser)]
#[command(
    name = "sportscraft-cli",
    about = "SportsCraft Biomechanics & Banister Training Studio CLI"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Engine status and constants
    Status,
    /// Inspect athlete Banister fitness, fatigue, and form
    Profile { athlete: String },
    /// Predict race finish time and pacing
    Predict { athlete: String, distance_m: f64 },
    /// Inspect IMU gait kinematics (demo fixture)
    Gait,
}

impl From<Commands> for Command {
    fn from(value: Commands) -> Self {
        match value {
            Commands::Status => Command::Status,
            Commands::Profile { athlete } => Command::Profile { athlete },
            Commands::Predict {
                athlete,
                distance_m,
            } => Command::Predict {
                athlete,
                distance_m,
            },
            Commands::Gait => Command::Gait,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let payload = Command::from(cli.command).dispatch();
    println!("{}", serde_json::to_string_pretty(&payload)?);
    Ok(())
}

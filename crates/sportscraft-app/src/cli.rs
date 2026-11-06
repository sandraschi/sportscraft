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
    /// List European soccer leagues (demo)
    SoccerLeagues,
    /// Show demo standings for a league (ENG1, ESP1, GER1, ITA1, FRA1)
    SoccerStandings { league: String },
    /// Show demo fixtures for a league
    SoccerFixtures { league: String },
    /// Predict a match from demo ratings
    SoccerPredict { home: String, away: String },
    /// List trainer exercises for a location (gym, home)
    TrainerCatalog { location: String },
    /// Build a weekly plan: goal location days_per_week level
    TrainerPlan {
        goal: String,
        location: String,
        days_per_week: u32,
        level: String,
    },
    /// Build one coached workout: goal location level minutes
    TrainerSession {
        goal: String,
        location: String,
        level: String,
        minutes: u32,
    },
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
            Commands::SoccerLeagues => Command::SoccerLeagues,
            Commands::SoccerStandings { league } => Command::SoccerStandings { league },
            Commands::SoccerFixtures { league } => Command::SoccerFixtures { league },
            Commands::SoccerPredict { home, away } => Command::SoccerPredict { home, away },
            Commands::TrainerCatalog { location } => Command::TrainerCatalog { location },
            Commands::TrainerPlan {
                goal,
                location,
                days_per_week,
                level,
            } => Command::TrainerPlan {
                goal,
                location,
                days_per_week,
                level,
            },
            Commands::TrainerSession {
                goal,
                location,
                level,
                minutes,
            } => Command::TrainerSession {
                goal,
                location,
                level,
                minutes,
            },
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let payload = Command::from(cli.command).dispatch();
    println!("{}", serde_json::to_string_pretty(&payload)?);
    Ok(())
}

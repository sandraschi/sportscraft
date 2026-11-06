//! SportsCraft Core Engine
//!
//! Physiological load aggregation, race pacing prediction, and the universal
//! `Command::dispatch` bus shared by the CLI, the MCP server, and the desktop UI.
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unimplemented)]
#![deny(clippy::todo)]

pub mod i18n;
pub mod soccer;
pub mod trainer;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sportscraft_kinematics::{BanisterProfile, BiomechanicsModel, GaitAnalysis};
use thiserror::Error;

/// Athlete "form" (TSB, TRIMP points) the pacing engine considers race-ready.
pub const OPTIMAL_TSB_WINDOW: (f64, f64) = (5.0, 18.0);

#[derive(Error, Debug)]
pub enum SportsError {
    #[error("calculation error: {0}")]
    Calculation(String),
    #[error("invalid input: {0}")]
    InvalidInput(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RacePacingPrediction {
    pub target_distance_m: f64,
    pub predicted_time_seconds: f64,
    pub recommended_pace_min_km: f64,
    pub optimal_form_tsb_window: (f64, f64),
}

pub struct SportsEngine;

impl SportsEngine {
    /// Deterministic demo training block (14 days, ~100 TRIMP average).
    ///
    /// This is a fixture, not imported athlete data: there is no FIT/GPX
    /// importer yet. Callers must label it as demo data in output.
    pub fn demo_trimp_history() -> Vec<f64> {
        vec![
            95.0, 110.0, 80.0, 120.0, 0.0, 140.0, 85.0, 105.0, 115.0, 90.0, 130.0, 0.0, 150.0, 75.0,
        ]
    }

    /// Default Banister profile with the demo history already applied.
    ///
    /// Both impulses use weight 1.0 so that CTL, ATL and TSB are all in the same
    /// TRIMP-point scale and `OPTIMAL_TSB_WINDOW` is reachable after a taper.
    pub fn get_default_banister_profile(athlete_id: &str) -> BanisterProfile {
        let mut profile = BanisterProfile {
            athlete_id: athlete_id.to_string(),
            fitness_tau_days: 42.0,
            fatigue_tau_days: 7.0,
            fitness_weight_k1: 1.0,
            fatigue_weight_k2: 1.0,
            current_fitness: 0.0,
            current_fatigue: 0.0,
            form_tsb: 0.0,
        };
        BiomechanicsModel::calculate_banister_state(&mut profile, &Self::demo_trimp_history());
        profile
    }

    /// Demo gait fixture (see [`BiomechanicsModel::demo_gait_profile`]).
    pub fn get_gait_analysis() -> GaitAnalysis {
        BiomechanicsModel::demo_gait_profile()
    }

    /// Critical-velocity style pacing prediction from current chronic fitness.
    pub fn predict_race_pacing(
        profile: &BanisterProfile,
        distance_m: f64,
    ) -> Result<RacePacingPrediction, SportsError> {
        if !distance_m.is_finite() || distance_m <= 0.0 {
            return Err(SportsError::InvalidInput(
                "distance_m must be a positive number of metres".to_string(),
            ));
        }
        let base_speed_m_s = 3.8 + (profile.current_fitness / 100.0) * 0.8;
        if !base_speed_m_s.is_finite() || base_speed_m_s <= 0.0 {
            return Err(SportsError::Calculation(
                "non-finite base speed from fitness profile".to_string(),
            ));
        }
        let predicted_time = distance_m / base_speed_m_s;
        let pace_min_km = (predicted_time / 60.0) / (distance_m / 1000.0);

        Ok(RacePacingPrediction {
            target_distance_m: distance_m,
            predicted_time_seconds: predicted_time,
            recommended_pace_min_km: pace_min_km,
            optimal_form_tsb_window: OPTIMAL_TSB_WINDOW,
        })
    }
}

/// One capability, addressed identically from the CLI and `tools/call`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "command", rename_all = "snake_case")]
pub enum Command {
    Status,
    Profile {
        athlete: String,
    },
    Predict {
        athlete: String,
        distance_m: f64,
    },
    Gait,
    SoccerLeagues,
    SoccerStandings {
        league: String,
    },
    SoccerFixtures {
        league: String,
    },
    SoccerPredict {
        home: String,
        away: String,
    },
    TrainerCatalog {
        location: String,
    },
    TrainerPlan {
        goal: String,
        location: String,
        days_per_week: u32,
        level: String,
    },
    TrainerSession {
        goal: String,
        location: String,
        level: String,
        minutes: u32,
    },
}

impl Command {
    /// Execute the command and return a JSON payload (never panics).
    pub fn dispatch(&self) -> Value {
        match self {
            Command::Status => json!({
                "status": "ready",
                "engine": "sportscraft",
                "version": env!("CARGO_PKG_VERSION"),
                "fitness_tau_days": 42.0,
                "fatigue_tau_days": 7.0,
                "optimal_tsb_window": [OPTIMAL_TSB_WINDOW.0, OPTIMAL_TSB_WINDOW.1],
                "commands": ["status", "profile", "predict", "gait",
                    "soccer_leagues", "soccer_standings", "soccer_fixtures", "soccer_predict",
                    "trainer_catalog", "trainer_plan", "trainer_session"],
                "history_source": "demo",
            }),
            Command::Profile { athlete } => {
                let profile = SportsEngine::get_default_banister_profile(athlete);
                match serde_json::to_value(&profile) {
                    Ok(mut value) => {
                        if let Some(object) = value.as_object_mut() {
                            object.insert("history_source".to_string(), json!("demo"));
                        }
                        value
                    }
                    Err(error) => json!({"success": false, "error": error.to_string()}),
                }
            }
            Command::Predict {
                athlete,
                distance_m,
            } => {
                let profile = SportsEngine::get_default_banister_profile(athlete);
                match SportsEngine::predict_race_pacing(&profile, *distance_m) {
                    Ok(prediction) => match serde_json::to_value(&prediction) {
                        Ok(mut value) => {
                            if let Some(object) = value.as_object_mut() {
                                object.insert("success".to_string(), json!(true));
                            }
                            value
                        }
                        Err(error) => json!({"success": false, "error": error.to_string()}),
                    },
                    Err(error) => json!({"success": false, "error": error.to_string()}),
                }
            }
            Command::Gait => match serde_json::to_value(SportsEngine::get_gait_analysis()) {
                Ok(mut value) => {
                    if let Some(object) = value.as_object_mut() {
                        object.insert("source".to_string(), json!("demo"));
                    }
                    value
                }
                Err(error) => json!({"success": false, "error": error.to_string()}),
            },
            Command::SoccerLeagues => json!({
                "success": true,
                "source": "demo",
                "leagues": soccer::leagues(),
            }),
            Command::SoccerStandings { league } => match soccer::standings(league) {
                Some(rows) => json!({
                    "success": true, "source": "demo",
                    "league": league.to_uppercase(), "standings": rows,
                }),
                None => json!({"success": false,
                        "error": format!("unknown league: {league} (try ENG1, ESP1, GER1, ITA1, FRA1)")}),
            },
            Command::SoccerFixtures { league } => match soccer::fixtures(league) {
                Some(rows) => json!({
                    "success": true, "source": "demo",
                    "league": league.to_uppercase(), "fixtures": rows,
                }),
                None => json!({"success": false,
                        "error": format!("unknown league: {league} (try ENG1, ESP1, GER1, ITA1, FRA1)")}),
            },
            Command::SoccerPredict { home, away } => match soccer::predict(home, away) {
                Ok(prediction) => match serde_json::to_value(&prediction) {
                    Ok(mut value) => {
                        if let Some(object) = value.as_object_mut() {
                            object.insert("success".to_string(), json!(true));
                            object.insert("source".to_string(), json!("demo"));
                        }
                        value
                    }
                    Err(error) => json!({"success": false, "error": error.to_string()}),
                },
                Err(error) => json!({"success": false, "error": error}),
            },
            Command::TrainerCatalog { location } => match trainer::parse_location(location) {
                Ok(loc) => json!({
                    "success": true, "source": "demo",
                    "location": location.to_lowercase(),
                    "exercises": trainer::catalog(loc),
                    "disclaimer": trainer::DISCLAIMER,
                }),
                Err(error) => json!({"success": false, "error": error}),
            },
            Command::TrainerPlan {
                goal,
                location,
                days_per_week,
                level,
            } => {
                let parsed = trainer::parse_goal(goal)
                    .and_then(|g| trainer::parse_location(location).map(|l| (g, l)))
                    .and_then(|(g, l)| trainer::parse_level(level).map(|v| (g, l, v)));
                match parsed {
                    Ok((g, l, v)) => match trainer::plan(g, l, *days_per_week, v) {
                        Ok(plan) => match serde_json::to_value(&plan) {
                            Ok(mut value) => {
                                if let Some(object) = value.as_object_mut() {
                                    object.insert("success".to_string(), json!(true));
                                    object.insert("source".to_string(), json!("demo"));
                                }
                                value
                            }
                            Err(error) => json!({"success": false, "error": error.to_string()}),
                        },
                        Err(error) => json!({"success": false, "error": error}),
                    },
                    Err(error) => json!({"success": false, "error": error}),
                }
            }
            Command::TrainerSession {
                goal,
                location,
                level,
                minutes,
            } => {
                let parsed = trainer::parse_goal(goal)
                    .and_then(|g| trainer::parse_location(location).map(|l| (g, l)))
                    .and_then(|(g, l)| trainer::parse_level(level).map(|v| (g, l, v)));
                match parsed {
                    Ok((g, l, v)) => match trainer::session(g, l, v, *minutes) {
                        Ok(session) => match serde_json::to_value(&session) {
                            Ok(mut value) => {
                                if let Some(object) = value.as_object_mut() {
                                    object.insert("success".to_string(), json!(true));
                                    object.insert("source".to_string(), json!("demo"));
                                }
                                value
                            }
                            Err(error) => json!({"success": false, "error": error.to_string()}),
                        },
                        Err(error) => json!({"success": false, "error": error}),
                    },
                    Err(error) => json!({"success": false, "error": error}),
                }
            }
        }
    }
}

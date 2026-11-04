//! SportsCraft Core Engine
//!
//! Provides physiological load aggregation, race pacing predictions,
//! and training log persistence backed by DatabaseCraft.

use serde::{Deserialize, Serialize};
use thiserror::Error;
use sportscraft_kinematics::{BanisterProfile, BiomechanicsModel, GaitAnalysis};

#[derive(Error, Debug)]
pub enum SportsError {
    #[error("Calculation error: {0}")]
    Calculation(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RacePacingPrediction {
    pub target_distance_m: f64,
    pub predicted_time_seconds: f64,
    pub recommended_pace_min_km: f64,
    pub optimal_form_tsb_window: (f64, f64),
}

pub struct SportsEngine;

impl SportsEngine {
    pub fn get_default_banister_profile(athlete_id: &str) -> BanisterProfile {
        let mut profile = BanisterProfile {
            athlete_id: athlete_id.to_string(),
            fitness_tau_days: 42.0,
            fatigue_tau_days: 7.0,
            fitness_weight_k1: 1.0,
            fatigue_weight_k2: 1.8,
            current_fitness: 0.0,
            current_fatigue: 0.0,
            form_tsb: 0.0,
        };
        // Simulated 30-day block with ~100 TRIMP daily average
        let history = vec![95.0, 110.0, 80.0, 120.0, 0.0, 140.0, 85.0, 105.0, 115.0, 90.0, 130.0, 0.0, 150.0, 75.0];
        BiomechanicsModel::calculate_banister_state(&mut profile, &history);
        profile
    }

    pub fn get_gait_analysis() -> GaitAnalysis {
        BiomechanicsModel::mock_gait_profile()
    }

    pub fn predict_race_pacing(profile: &BanisterProfile, distance_m: f64) -> RacePacingPrediction {
        // High-level critical power / velocity regression from fitness CTL
        let base_speed_m_s = 3.8 + (profile.current_fitness / 100.0) * 0.8;
        let predicted_time = distance_m / base_speed_m_s;
        let pace_min_km = (predicted_time / 60.0) / (distance_m / 1000.0);

        RacePacingPrediction {
            target_distance_m: distance_m,
            predicted_time_seconds: predicted_time,
            recommended_pace_min_km: pace_min_km,
            optimal_form_tsb_window: (5.0, 18.0),
        }
    }
}

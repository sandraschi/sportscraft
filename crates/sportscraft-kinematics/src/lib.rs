//! SportsCraft Kinematics & Biomechanics Engine
//!
//! Provides Banister impulse-response fatigue/fitness modeling (TRIMP),
//! critical velocity calculations, and IMU gait kinematic analysis.
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unimplemented)]
#![deny(clippy::todo)]

use serde::{Deserialize, Serialize};

/// Banister impulse-response state for one athlete.
///
/// `current_fitness` / `current_fatigue` are exponentially weighted averages of
/// daily TRIMP (the TrainingPeaks CTL/ATL definition, so both use weight 1.0).
/// `form_tsb` is `current_fitness - current_fatigue` in TRIMP points, which is
/// the quantity the `+5 .. +18` taper window is expressed in.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BanisterProfile {
    pub athlete_id: String,
    /// Chronic fitness time constant, typically ~42 days.
    pub fitness_tau_days: f64,
    /// Acute fatigue time constant, typically ~7 days.
    pub fatigue_tau_days: f64,
    /// Magnitude scaling for the fitness impulse.
    pub fitness_weight_k1: f64,
    /// Magnitude scaling for the fatigue impulse.
    pub fatigue_weight_k2: f64,
    /// Chronic Training Load (CTL).
    pub current_fitness: f64,
    /// Acute Training Load (ATL).
    pub current_fatigue: f64,
    /// Training Stress Balance (CTL - ATL).
    pub form_tsb: f64,
}

/// IMU-derived gait metrics.
///
/// The values returned by [`BiomechanicsModel::demo_gait_profile`] are a
/// deterministic demo fixture, not a live sensor reading. Callers must label
/// them as demo data (the MCP server returns `"source": "demo"`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GaitAnalysis {
    pub cadence_spm: f64,
    pub vertical_oscillation_cm: f64,
    pub ground_contact_time_ms: f64,
    pub leg_spring_stiffness_kn_m: f64,
    pub asymmetry_pct: f64,
}

pub struct BiomechanicsModel;

impl BiomechanicsModel {
    /// Fold a daily TRIMP history into `profile`'s CTL/ATL/TSB.
    ///
    /// Non-finite or non-positive time constants fall back to the 42 d / 7 d
    /// defaults rather than producing `NaN`.
    pub fn calculate_banister_state(profile: &mut BanisterProfile, daily_trimp_history: &[f64]) {
        let fitness_tau = if profile.fitness_tau_days.is_finite() && profile.fitness_tau_days > 0.0
        {
            profile.fitness_tau_days
        } else {
            42.0
        };
        let fatigue_tau = if profile.fatigue_tau_days.is_finite() && profile.fatigue_tau_days > 0.0
        {
            profile.fatigue_tau_days
        } else {
            7.0
        };

        let mut ctl = 0.0;
        let mut atl = 0.0;
        let ctl_decay = (-1.0 / fitness_tau).exp();
        let atl_decay = (-1.0 / fatigue_tau).exp();

        for &trimp in daily_trimp_history {
            if !trimp.is_finite() || trimp < 0.0 {
                continue;
            }
            ctl = ctl * ctl_decay + trimp * (1.0 - ctl_decay) * profile.fitness_weight_k1;
            atl = atl * atl_decay + trimp * (1.0 - atl_decay) * profile.fatigue_weight_k2;
        }

        profile.current_fitness = ctl;
        profile.current_fatigue = atl;
        profile.form_tsb = ctl - atl;
    }

    /// Deterministic demo gait fixture (no sensor attached).
    pub fn demo_gait_profile() -> GaitAnalysis {
        GaitAnalysis {
            cadence_spm: 182.0,
            vertical_oscillation_cm: 6.4,
            ground_contact_time_ms: 215.0,
            leg_spring_stiffness_kn_m: 12.8,
            asymmetry_pct: 1.2,
        }
    }
}

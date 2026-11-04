//! SportsCraft Kinematics & Biomechanics Engine
//!
//! Provides Banister impulse-response fatigue/fitness modeling (TRIMP),
//! critical velocity calculations, and IMU gait kinematic analysis.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BanisterProfile {
    pub athlete_id: String,
    pub fitness_tau_days: f64,   // Typically ~42 days
    pub fatigue_tau_days: f64,   // Typically ~7 days
    pub fitness_weight_k1: f64,  // Magnitude scaling for fitness
    pub fatigue_weight_k2: f64,  // Magnitude scaling for fatigue
    pub current_fitness: f64,    // Chronic Training Load (CTL)
    pub current_fatigue: f64,    // Acute Training Load (ATL)
    pub form_tsb: f64,           // Training Stress Balance (CTL - ATL)
}

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
    pub fn calculate_banister_state(
        profile: &mut BanisterProfile,
        daily_trimp_history: &[f64],
    ) {
        let mut ctl = 0.0;
        let mut atl = 0.0;
        let ctl_decay = (-1.0 / profile.fitness_tau_days).exp();
        let atl_decay = (-1.0 / profile.fatigue_tau_days).exp();

        for &trimp in daily_trimp_history {
            ctl = ctl * ctl_decay + trimp * (1.0 - ctl_decay) * profile.fitness_weight_k1;
            atl = atl * atl_decay + trimp * (1.0 - atl_decay) * profile.fatigue_weight_k2;
        }

        profile.current_fitness = ctl;
        profile.current_fatigue = atl;
        profile.form_tsb = ctl - atl;
    }

    pub fn mock_gait_profile() -> GaitAnalysis {
        GaitAnalysis {
            cadence_spm: 182.0,
            vertical_oscillation_cm: 6.4,
            ground_contact_time_ms: 215.0,
            leg_spring_stiffness_kn_m: 12.8,
            asymmetry_pct: 1.2,
        }
    }
}

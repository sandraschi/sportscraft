use sportscraft_core::SportsEngine;

#[test]
fn test_sportscraft_banister_modeling() {
    let profile = SportsEngine::get_default_banister_profile("ATHLETE-ELITE-1");
    assert!(profile.current_fitness > 0.0);
    assert!(profile.current_fatigue > 0.0);
    // TSB = CTL - ATL
    assert_eq!(profile.form_tsb, profile.current_fitness - profile.current_fatigue);
}

#[test]
fn test_sportscraft_pacing_prediction() {
    let profile = SportsEngine::get_default_banister_profile("ATHLETE-ELITE-1");
    let marathon = SportsEngine::predict_race_pacing(&profile, 42195.0);
    assert!(marathon.predicted_time_seconds > 7200.0); // > 2 hours
    assert!(marathon.recommended_pace_min_km < 6.0); // Sub 6 min/km pace
}

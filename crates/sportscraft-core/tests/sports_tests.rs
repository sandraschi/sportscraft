//! Regression tests for the Banister engine, pacing prediction, and the
//! `Command::dispatch` bus shared by the CLI and MCP surfaces.
use sportscraft_core::i18n::{t, AppLocale, I18nKey};
use sportscraft_core::{soccer, trainer, Command, SportsEngine, OPTIMAL_TSB_WINDOW};
use sportscraft_kinematics::{BanisterProfile, BiomechanicsModel};

fn profile_with(history: &[f64]) -> BanisterProfile {
    let mut profile = BanisterProfile {
        athlete_id: "TEST".to_string(),
        fitness_tau_days: 42.0,
        fatigue_tau_days: 7.0,
        fitness_weight_k1: 1.0,
        fatigue_weight_k2: 1.0,
        current_fitness: 0.0,
        current_fatigue: 0.0,
        form_tsb: 0.0,
    };
    BiomechanicsModel::calculate_banister_state(&mut profile, history);
    profile
}

#[test]
fn banister_tsb_is_ctl_minus_atl() {
    let profile = SportsEngine::get_default_banister_profile("ATHLETE-ELITE-1");
    assert!(profile.current_fitness > 0.0);
    assert!(profile.current_fatigue > 0.0);
    assert!((profile.form_tsb - (profile.current_fitness - profile.current_fatigue)).abs() < 1e-9);
}

#[test]
fn taper_raises_form_out_of_the_negative_trap() {
    // 30 days of hard training, then a 21-day taper at 30% load.
    let mut history = vec![100.0; 30];
    history.extend(vec![30.0; 21]);
    let tapered = profile_with(&history);
    let mut hard_only = vec![100.0; 30];
    hard_only.extend(vec![0.0; 21]);
    let detrained = profile_with(&hard_only);

    assert!(tapered.form_tsb > 0.0, "taper must produce positive form");
    assert!(
        tapered.current_fitness > detrained.current_fitness,
        "taper keeps more fitness than total rest"
    );
    assert!(
        tapered.form_tsb >= OPTIMAL_TSB_WINDOW.0,
        "a 21-day taper should reach the advertised TSB window"
    );
}

#[test]
fn optimal_window_constant_is_reachable_shape() {
    assert!(OPTIMAL_TSB_WINDOW.0 < OPTIMAL_TSB_WINDOW.1);
    let status = Command::Status.dispatch();
    let window = status
        .get("optimal_tsb_window")
        .and_then(|value| value.as_array());
    assert!(window.is_some());
}

#[test]
fn pacing_prediction_scales_with_distance() {
    let profile = SportsEngine::get_default_banister_profile("ATHLETE-ELITE-1");
    let ten_k = SportsEngine::predict_race_pacing(&profile, 10000.0);
    let marathon = SportsEngine::predict_race_pacing(&profile, 42195.0);
    assert!(ten_k.is_ok());
    assert!(marathon.is_ok());
    if let (Ok(short), Ok(long)) = (ten_k, marathon) {
        assert!(long.predicted_time_seconds > short.predicted_time_seconds);
        assert!(short.recommended_pace_min_km > 0.0);
        assert!(long.recommended_pace_min_km < 10.0);
    }
}

#[test]
fn pacing_rejects_nonpositive_distance() {
    let profile = SportsEngine::get_default_banister_profile("ATHLETE-ELITE-1");
    assert!(SportsEngine::predict_race_pacing(&profile, 0.0).is_err());
    assert!(SportsEngine::predict_race_pacing(&profile, -5.0).is_err());
    assert!(SportsEngine::predict_race_pacing(&profile, f64::NAN).is_err());
}

#[test]
fn dispatch_profile_marks_demo_history() {
    let payload = Command::Profile {
        athlete: "ATHLETE-X".to_string(),
    }
    .dispatch();
    assert_eq!(
        payload
            .get("history_source")
            .and_then(|value| value.as_str()),
        Some("demo")
    );
    assert_eq!(
        payload.get("athlete_id").and_then(|value| value.as_str()),
        Some("ATHLETE-X")
    );
}

#[test]
fn dispatch_gait_marks_demo_source() {
    let payload = Command::Gait.dispatch();
    assert_eq!(
        payload.get("source").and_then(|value| value.as_str()),
        Some("demo")
    );
}

#[test]
fn dispatch_predict_invalid_distance_is_an_error_object() {
    let payload = Command::Predict {
        athlete: "ATHLETE-X".to_string(),
        distance_m: 0.0,
    }
    .dispatch();
    assert_eq!(
        payload.get("success").and_then(|value| value.as_bool()),
        Some(false)
    );
}

#[test]
fn i18n_ui_strings_are_real_per_locale() {
    assert_eq!(t(I18nKey::MenuFile, AppLocale::En), "File");
    assert_eq!(t(I18nKey::MenuFile, AppLocale::De), "Datei");
    assert_eq!(t(I18nKey::MenuFile, AppLocale::Ja), "ファイル");
    assert_ne!(
        t(I18nKey::AppTitle, AppLocale::En),
        t(I18nKey::AppTitle, AppLocale::De)
    );
    for locale in AppLocale::all() {
        assert!(!t(I18nKey::LabelForm, *locale).is_empty());
        assert!(!t(I18nKey::PanelPacing, *locale).is_empty());
    }
}

#[test]
fn soccer_has_five_european_leagues() {
    let leagues = soccer::leagues();
    assert_eq!(leagues.len(), 5);
    assert!(leagues.iter().any(|l| l.code == "ENG1"));
    assert!(leagues.iter().any(|l| l.code == "GER1"));
}

#[test]
fn soccer_standings_points_are_consistent() {
    let rows = soccer::standings("ENG1").unwrap_or_default();
    assert_eq!(rows.len(), 6);
    for row in &rows {
        assert_eq!(row.points, row.won * 3 + row.drawn);
        assert_eq!(row.played, row.won + row.drawn + row.lost);
    }
}

#[test]
fn soccer_unknown_league_is_none() {
    assert!(soccer::standings("XX9").is_none());
    assert!(soccer::fixtures("XX9").is_none());
}

#[test]
fn soccer_predict_favours_the_stronger_home_side() {
    let result = soccer::predict("Man City", "Nice");
    assert!(result.is_ok());
    if let Ok(prediction) = result {
        assert!(prediction.home_win_pct > prediction.away_win_pct);
        assert!(
            (prediction.home_win_pct + prediction.draw_pct + prediction.away_win_pct - 100.0).abs()
                < 1.0
        );
    }
}

#[test]
fn soccer_predict_rejects_unknown_teams() {
    assert!(soccer::predict("Man City", "Atlantis FC").is_err());
}

#[test]
fn trainer_home_catalog_has_no_gym_only_machines() {
    let home = trainer::catalog(trainer::Location::Home);
    assert!(!home.is_empty());
    assert!(home.iter().all(|item| item.location != "gym"));
    assert!(home.iter().any(|item| item.id == "pushup"));
}

#[test]
fn trainer_plan_builds_requested_days_with_trimp() {
    let result = trainer::plan(
        trainer::Goal::General,
        trainer::Location::Home,
        3,
        trainer::Level::Beginner,
    );
    assert!(result.is_ok());
    if let Ok(plan) = result {
        assert_eq!(plan.days.len(), 3);
        assert!(plan.days.iter().all(|day| !day.exercises.is_empty()));
        assert!(plan.days.iter().all(|day| day.estimated_trimp > 0.0));
    }
}

#[test]
fn trainer_plan_rejects_day_counts_outside_two_to_five() {
    assert!(trainer::plan(
        trainer::Goal::Strength,
        trainer::Location::Gym,
        1,
        trainer::Level::Intermediate,
    )
    .is_err());
    assert!(trainer::plan(
        trainer::Goal::Strength,
        trainer::Location::Gym,
        6,
        trainer::Level::Intermediate,
    )
    .is_err());
}

#[test]
fn trainer_session_carries_disclaimer_and_coaching() {
    let result = trainer::session(
        trainer::Goal::Conditioning,
        trainer::Location::Gym,
        trainer::Level::Intermediate,
        45,
    );
    assert!(result.is_ok());
    if let Ok(session) = result {
        assert!(!session.main.is_empty());
        assert!(!session.coaching.is_empty());
        assert!(session.disclaimer.contains("not medical advice"));
        assert!(session.estimated_trimp > 0.0);
    }
}

#[test]
fn dispatch_soccer_and_trainer_arms_are_demo_labeled() {
    let standings = Command::SoccerStandings {
        league: "ENG1".to_string(),
    }
    .dispatch();
    assert_eq!(
        standings.get("source").and_then(|value| value.as_str()),
        Some("demo")
    );
    let catalog = Command::TrainerCatalog {
        location: "home".to_string(),
    }
    .dispatch();
    assert_eq!(
        catalog.get("source").and_then(|value| value.as_str()),
        Some("demo")
    );
    let bad = Command::SoccerStandings {
        league: "XX9".to_string(),
    }
    .dispatch();
    assert_eq!(
        bad.get("success").and_then(|value| value.as_bool()),
        Some(false)
    );
}

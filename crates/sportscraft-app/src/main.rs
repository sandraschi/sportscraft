//! SportsCraft desktop studio (`eframe` / `egui`).
//!
//! Two top-level sections: **Spectator** (European soccer tables, fixtures,
//! predictor) and **Personal** (gym/home trainer + Banister lab). Every visible
//! string goes through `sportscraft_core::i18n::t`, so the locale selector
//! actually localizes the UI. Data is produced by `sportscraft_core` in-process;
//! there is no backend port.
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unimplemented)]
#![deny(clippy::todo)]

use clap::Parser;
use serde_json::json;
use sportscraft_core::i18n::{t, AppLocale, I18nKey};
use sportscraft_core::soccer::MatchPrediction;
use sportscraft_core::trainer::{WorkoutPlan, WorkoutSession};
use sportscraft_core::{soccer, trainer, SportsEngine, OPTIMAL_TSB_WINDOW};

#[derive(Parser, Debug)]
#[command(
    name = "sportscraft",
    about = "SportsCraft Spectator & Personal Trainer Studio"
)]
struct Args {
    #[arg(long)]
    smoke: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Section {
    Spectator,
    Personal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PersonalTab {
    Trainer,
    Lab,
}

const GOALS: [&str; 4] = ["general", "strength", "hypertrophy", "conditioning"];
const LEVELS: [&str; 3] = ["beginner", "intermediate", "advanced"];

fn goal_of(index: usize) -> trainer::Goal {
    match index {
        1 => trainer::Goal::Strength,
        2 => trainer::Goal::Hypertrophy,
        3 => trainer::Goal::Conditioning,
        _ => trainer::Goal::General,
    }
}

fn level_of(index: usize) -> trainer::Level {
    match index {
        1 => trainer::Level::Intermediate,
        2 => trainer::Level::Advanced,
        _ => trainer::Level::Beginner,
    }
}

pub struct SportsCraftApp {
    athlete_name: String,
    locale: AppLocale,
    target_distance: f64,
    status_msg: String,
    is_dark_theme: bool,
    show_about_dialog: bool,
    section: Section,
    personal_tab: PersonalTab,
    leagues: Vec<(String, String)>,
    league_idx: usize,
    home_team: String,
    away_team: String,
    prediction: Option<MatchPrediction>,
    prediction_error: String,
    goal_idx: usize,
    at_gym: bool,
    level_idx: usize,
    days_per_week: u32,
    session_minutes: u32,
    plan: Option<WorkoutPlan>,
    session: Option<WorkoutSession>,
    trainer_error: String,
}

impl SportsCraftApp {
    pub fn new() -> Self {
        let leagues = soccer::leagues()
            .into_iter()
            .map(|league| (league.code, league.name))
            .collect::<Vec<(String, String)>>();
        Self {
            athlete_name: "ATHLETE-ELITE-1".to_string(),
            locale: AppLocale::default(),
            target_distance: 10000.0,
            status_msg: "SportsCraft Studio Ready".to_string(),
            is_dark_theme: true,
            show_about_dialog: false,
            section: Section::Spectator,
            personal_tab: PersonalTab::Trainer,
            leagues,
            league_idx: 0,
            home_team: "Man City".to_string(),
            away_team: "Arsenal".to_string(),
            prediction: None,
            prediction_error: String::new(),
            goal_idx: 0,
            at_gym: false,
            level_idx: 0,
            days_per_week: 3,
            session_minutes: 45,
            plan: None,
            session: None,
            trainer_error: String::new(),
        }
    }

    fn label(&self, key: I18nKey) -> &'static str {
        t(key, self.locale)
    }

    fn league_code(&self) -> String {
        self.leagues
            .get(self.league_idx)
            .map(|(code, _)| code.clone())
            .unwrap_or_else(|| "ENG1".to_string())
    }

    /// Profile + prediction as one JSON payload (used by export and clipboard).
    fn plan_payload(&self) -> serde_json::Value {
        let profile = SportsEngine::get_default_banister_profile(&self.athlete_name);
        let prediction = SportsEngine::predict_race_pacing(&profile, self.target_distance);
        let (prediction_value, prediction_error) = match prediction {
            Ok(value) => (
                serde_json::to_value(value).unwrap_or(serde_json::Value::Null),
                None,
            ),
            Err(error) => (serde_json::Value::Null, Some(error.to_string())),
        };
        json!({
            "athlete": self.athlete_name,
            "target_distance_m": self.target_distance,
            "banister": profile,
            "history_source": "demo",
            "prediction": prediction_value,
            "prediction_error": prediction_error,
            "generated_by": format!("sportscraft {}", env!("CARGO_PKG_VERSION")),
        })
    }

    /// Write the pacing plan next to the working directory; report the real path.
    fn export_plan(&mut self) {
        let file_name = format!("sportscraft-pacing-{:.0}m.json", self.target_distance);
        let path = std::env::current_dir()
            .map(|dir| dir.join(&file_name))
            .unwrap_or_else(|_| std::path::PathBuf::from(&file_name));
        match serde_json::to_string_pretty(&self.plan_payload()) {
            Ok(text) => match std::fs::write(&path, text) {
                Ok(()) => self.status_msg = format!("Exported pacing plan to {}", path.display()),
                Err(error) => self.status_msg = format!("Export failed: {error}"),
            },
            Err(error) => self.status_msg = format!("Export failed: {error}"),
        }
    }

    fn copy_plan(&mut self, ctx: &egui::Context) {
        match serde_json::to_string_pretty(&self.plan_payload()) {
            Ok(text) => {
                ctx.copy_text(text);
                self.status_msg = "Pacing plan copied to clipboard".to_string();
            }
            Err(error) => self.status_msg = format!("Copy failed: {error}"),
        }
    }

    fn run_prediction(&mut self) {
        match soccer::predict(&self.home_team, &self.away_team) {
            Ok(value) => {
                self.prediction = Some(value);
                self.prediction_error.clear();
                self.status_msg = "Match prediction ready (demo ratings)".to_string();
            }
            Err(error) => {
                self.prediction = None;
                self.prediction_error = error.clone();
                self.status_msg = format!("Prediction failed: {error}");
            }
        }
    }

    fn build_plan(&mut self) {
        let location = if self.at_gym {
            trainer::Location::Gym
        } else {
            trainer::Location::Home
        };
        match trainer::plan(
            goal_of(self.goal_idx),
            location,
            self.days_per_week,
            level_of(self.level_idx),
        ) {
            Ok(value) => {
                self.plan = Some(value);
                self.trainer_error.clear();
                self.status_msg = "Weekly plan built (demo catalog)".to_string();
            }
            Err(error) => {
                self.plan = None;
                self.trainer_error = error.clone();
                self.status_msg = format!("Plan failed: {error}");
            }
        }
    }

    fn build_session(&mut self) {
        let location = if self.at_gym {
            trainer::Location::Gym
        } else {
            trainer::Location::Home
        };
        match trainer::session(
            goal_of(self.goal_idx),
            location,
            level_of(self.level_idx),
            self.session_minutes,
        ) {
            Ok(value) => {
                self.session = Some(value);
                self.trainer_error.clear();
                self.status_msg = "Workout built (demo catalog)".to_string();
            }
            Err(error) => {
                self.session = None;
                self.trainer_error = error.clone();
                self.status_msg = format!("Workout failed: {error}");
            }
        }
    }

    fn render_spectator(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(self.label(I18nKey::FieldLeague));
            let names = self
                .leagues
                .iter()
                .map(|(_, name)| name.clone())
                .collect::<Vec<String>>();
            let current = self
                .leagues
                .get(self.league_idx)
                .map(|(_, name)| name.clone())
                .unwrap_or_else(|| "Premier League".to_string());
            egui::ComboBox::from_id_salt("spectator_league")
                .selected_text(current)
                .show_ui(ui, |ui| {
                    for (index, name) in names.iter().enumerate() {
                        let _ = ui.selectable_value(&mut self.league_idx, index, name);
                    }
                });
            ui.label("source: demo (no live feed)");
        });
        ui.add_space(4.0);

        let code = self.league_code();
        ui.columns(2, |cols| {
            cols[0].heading(t(I18nKey::LabelStandings, self.locale));
            if let Some(rows) = soccer::standings(&code) {
                egui::Grid::new("standings_grid")
                    .striped(true)
                    .show(&mut cols[0], |ui| {
                        ui.label("#");
                        ui.label("Team");
                        ui.label("P");
                        ui.label("W-D-L");
                        ui.label("Pts");
                        ui.label("Form");
                        ui.end_row();
                        for row in &rows {
                            ui.label(format!("{}", row.position));
                            ui.label(&row.team);
                            ui.label(format!("{}", row.played));
                            ui.label(format!("{}-{}-{}", row.won, row.drawn, row.lost));
                            ui.label(format!("{}", row.points));
                            ui.label(&row.form);
                            ui.end_row();
                        }
                    });
            } else {
                cols[0].label("No table for this league code.");
            }

            cols[1].heading(t(I18nKey::LabelFixtures, self.locale));
            if let Some(fixtures) = soccer::fixtures(&code) {
                for game in &fixtures {
                    cols[1].group(|ui| {
                        ui.label(format!("{} vs {}", game.home, game.away));
                        ui.weak(format!("{} · {}", game.kickoff, game.venue));
                    });
                }
            } else {
                cols[1].label("No fixtures for this league code.");
            }
        });

        ui.add_space(8.0);
        ui.heading(self.label(I18nKey::LabelMatchPrediction));
        ui.horizontal(|ui| {
            ui.label(self.label(I18nKey::FieldHomeTeam));
            ui.text_edit_singleline(&mut self.home_team);
            ui.label(self.label(I18nKey::FieldAwayTeam));
            ui.text_edit_singleline(&mut self.away_team);
            if ui.button(self.label(I18nKey::ActionPredictMatch)).clicked() {
                self.run_prediction();
            }
        });
        if let Some(prediction) = &self.prediction {
            ui.group(|ui| {
                ui.label(format!(
                    "{} {:.0}% · Draw {:.0}% · {} {:.0}%",
                    prediction.home,
                    prediction.home_win_pct,
                    prediction.draw_pct,
                    prediction.away,
                    prediction.away_win_pct
                ));
                ui.label(format!(
                    "xG {:.2} - {:.2} · Pick: {}",
                    prediction.predicted_home_goals,
                    prediction.predicted_away_goals,
                    prediction.pick
                ));
            });
        }
        if !self.prediction_error.is_empty() {
            ui.colored_label(egui::Color32::from_rgb(220, 80, 50), &self.prediction_error);
        }
    }

    fn render_trainer(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(self.label(I18nKey::FieldGoal));
            egui::ComboBox::from_id_salt("trainer_goal")
                .selected_text(GOALS[self.goal_idx.min(GOALS.len() - 1)])
                .show_ui(ui, |ui| {
                    for (index, name) in GOALS.iter().enumerate() {
                        let _ = ui.selectable_value(&mut self.goal_idx, index, *name);
                    }
                });
            ui.label(self.label(I18nKey::FieldLocation));
            let gym_label = self.label(I18nKey::LocationGym);
            let home_label = self.label(I18nKey::LocationHome);
            ui.selectable_value(&mut self.at_gym, true, gym_label);
            ui.selectable_value(&mut self.at_gym, false, home_label);
            ui.label(self.label(I18nKey::FieldLevel));
            egui::ComboBox::from_id_salt("trainer_level")
                .selected_text(LEVELS[self.level_idx.min(LEVELS.len() - 1)])
                .show_ui(ui, |ui| {
                    for (index, name) in LEVELS.iter().enumerate() {
                        let _ = ui.selectable_value(&mut self.level_idx, index, *name);
                    }
                });
        });
        ui.horizontal(|ui| {
            ui.label(self.label(I18nKey::FieldDaysPerWeek));
            ui.add(egui::DragValue::new(&mut self.days_per_week).range(2..=5));
            if ui.button(self.label(I18nKey::ActionGeneratePlan)).clicked() {
                self.build_plan();
            }
            ui.label(self.label(I18nKey::FieldMinutes));
            ui.add(egui::DragValue::new(&mut self.session_minutes).range(15..=120));
            if ui
                .button(self.label(I18nKey::ActionGenerateSession))
                .clicked()
            {
                self.build_session();
            }
        });
        if !self.trainer_error.is_empty() {
            ui.colored_label(egui::Color32::from_rgb(220, 80, 50), &self.trainer_error);
        }

        ui.columns(2, |cols| {
            cols[0].heading(t(I18nKey::LabelWorkoutPlan, self.locale));
            if let Some(plan) = &self.plan {
                for day in &plan.days {
                    cols[0].group(|ui| {
                        ui.label(format!(
                            "Day {} · {} · ~{} min · TRIMP {:.0}",
                            day.day, day.focus, day.estimated_minutes, day.estimated_trimp
                        ));
                        for item in &day.exercises {
                            ui.weak(format!(
                                "{} — {} × {} ({}s rest)",
                                item.name, item.sets, item.reps, item.rest_seconds
                            ));
                        }
                    });
                }
                cols[0].weak(&plan.disclaimer);
            } else {
                cols[0].weak("No plan yet — pick gym/home, goal, days, then build.");
            }

            cols[1].heading(t(I18nKey::LabelWorkoutSession, self.locale));
            if let Some(session) = &self.session {
                cols[1].group(|ui| {
                    ui.label(format!(
                        "{} min · TRIMP ~{:.0}",
                        session.minutes, session.estimated_trimp
                    ));
                    ui.label("Warm-up:");
                    for line in &session.warmup {
                        ui.weak(format!("· {line}"));
                    }
                    ui.label("Main:");
                    for item in &session.main {
                        ui.label(format!("{} — {} × {}", item.name, item.sets, item.reps));
                        ui.weak(&item.cues);
                    }
                    ui.label("Cooldown:");
                    for line in &session.cooldown {
                        ui.weak(format!("· {line}"));
                    }
                    ui.label("Coaching:");
                    for line in &session.coaching {
                        ui.weak(format!("· {line}"));
                    }
                });
                cols[1].weak(&session.disclaimer);
            } else {
                cols[1].weak("No workout yet — set minutes, then build.");
            }
        });

        ui.add_space(4.0);
        ui.heading(self.label(I18nKey::LabelExerciseLibrary));
        let location = if self.at_gym {
            trainer::Location::Gym
        } else {
            trainer::Location::Home
        };
        egui::ScrollArea::vertical()
            .max_height(180.0)
            .show(ui, |ui| {
                for item in trainer::catalog(location) {
                    ui.horizontal(|ui| {
                        ui.label(format!("{} ({})", item.name, item.equipment));
                        ui.weak(format!("{} × {} · {}", item.sets, item.reps, item.muscle));
                    });
                }
            });
    }

    fn render_lab(&mut self, ui: &mut egui::Ui) {
        ui.columns(2, |cols| {
            cols[0].heading(self.label(I18nKey::PanelBanister));
            let profile = SportsEngine::get_default_banister_profile(&self.athlete_name);
            cols[0].group(|ui| {
                ui.label(format!("Athlete: {}", profile.athlete_id));
                ui.label(format!(
                    "{}: {:.1}",
                    self.label(I18nKey::LabelFitness),
                    profile.current_fitness
                ));
                ui.label(format!(
                    "{}: {:.1}",
                    self.label(I18nKey::LabelFatigue),
                    profile.current_fatigue
                ));
                let tsb_in_window = profile.form_tsb >= OPTIMAL_TSB_WINDOW.0
                    && profile.form_tsb <= OPTIMAL_TSB_WINDOW.1;
                let tsb_color = if tsb_in_window {
                    egui::Color32::from_rgb(50, 200, 50)
                } else {
                    egui::Color32::from_rgb(220, 80, 50)
                };
                ui.colored_label(
                    tsb_color,
                    format!(
                        "{}: {:.1} (target +{:.0} .. +{:.0})",
                        self.label(I18nKey::LabelForm),
                        profile.form_tsb,
                        OPTIMAL_TSB_WINDOW.0,
                        OPTIMAL_TSB_WINDOW.1
                    ),
                );
            });

            cols[0].add_space(8.0);
            cols[0].heading(self.label(I18nKey::PanelGait));
            let gait = SportsEngine::get_gait_analysis();
            cols[0].group(|ui| {
                ui.label(format!(
                    "{}: {:.0} spm",
                    self.label(I18nKey::LabelCadence),
                    gait.cadence_spm
                ));
                ui.label(format!(
                    "{}: {:.1} cm",
                    self.label(I18nKey::LabelVerticalOsc),
                    gait.vertical_oscillation_cm
                ));
                ui.label(format!(
                    "{}: {:.0} ms",
                    self.label(I18nKey::LabelGroundContact),
                    gait.ground_contact_time_ms
                ));
                ui.label(format!(
                    "{}: {:.1} kN/m",
                    self.label(I18nKey::LabelLegStiffness),
                    gait.leg_spring_stiffness_kn_m
                ));
                ui.label(format!(
                    "{}: {:.1}%",
                    self.label(I18nKey::LabelAsymmetry),
                    gait.asymmetry_pct
                ));
                ui.weak("source: demo fixture (no sensor attached)");
            });

            cols[1].heading(self.label(I18nKey::PanelPacing));
            cols[1].horizontal(|ui| {
                ui.label(self.label(I18nKey::FieldTargetDistance));
                ui.add(
                    egui::DragValue::new(&mut self.target_distance)
                        .speed(100.0)
                        .range(800.0..=100000.0),
                );
            });
            let profile = SportsEngine::get_default_banister_profile(&self.athlete_name);
            cols[1].add_space(8.0);
            cols[1].group(|ui| {
                ui.label(egui::RichText::new(self.label(I18nKey::LabelPredictionResults)).strong());
                match SportsEngine::predict_race_pacing(&profile, self.target_distance) {
                    Ok(pacing) => {
                        let total_min = pacing.predicted_time_seconds / 60.0;
                        let hrs = (total_min / 60.0).floor();
                        let mins = total_min % 60.0;
                        ui.label(format!(
                            "{}: {:.0}h {:.1}m ({:.0} sec)",
                            self.label(I18nKey::LabelEstimatedFinish),
                            hrs,
                            mins,
                            pacing.predicted_time_seconds
                        ));
                        ui.label(format!(
                            "{}: {:.2} min/km",
                            self.label(I18nKey::LabelRecommendedPace),
                            pacing.recommended_pace_min_km
                        ));
                        ui.label(format!(
                            "{}: +{:.0} .. +{:.0}",
                            self.label(I18nKey::LabelTsbWindow),
                            pacing.optimal_form_tsb_window.0,
                            pacing.optimal_form_tsb_window.1
                        ));
                    }
                    Err(error) => {
                        ui.colored_label(
                            egui::Color32::from_rgb(220, 80, 50),
                            format!("Prediction unavailable: {error}"),
                        );
                    }
                }
            });
        });
    }
}

impl Default for SportsCraftApp {
    fn default() -> Self {
        Self::new()
    }
}

impl eframe::App for SportsCraftApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if self.is_dark_theme {
            ui.ctx().set_visuals(egui::Visuals::dark());
        } else {
            ui.ctx().set_visuals(egui::Visuals::light());
        }

        // 0. Desktop Menu Bar
        egui::Panel::top("desktop_menu_bar").show(ui, |ui| {
            ui.horizontal(|ui| {
                let file_label = self.label(I18nKey::MenuFile);
                ui.menu_button(file_label, |ui| {
                    let export_label = self.label(I18nKey::ActionExportPlan);
                    if ui.button(export_label).clicked() {
                        self.export_plan();
                        ui.close();
                    }
                    let copy_label = self.label(I18nKey::ActionCopyPlan);
                    if ui.button(copy_label).clicked() {
                        self.copy_plan(ui.ctx());
                        ui.close();
                    }
                    ui.separator();
                    let exit_label = self.label(I18nKey::ActionExit);
                    if ui.button(exit_label).clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                let distance_label = self.label(I18nKey::MenuDistances);
                ui.menu_button(distance_label, |ui| {
                    for (text, metres) in [
                        ("5,000 m (5K)", 5000.0),
                        ("10,000 m (10K)", 10000.0),
                        ("21,097 m (Half Marathon)", 21097.5),
                        ("42,195 m (Marathon)", 42195.0),
                        ("100,000 m (100K)", 100000.0),
                    ] {
                        if ui.button(text).clicked() {
                            self.target_distance = metres;
                            self.status_msg = format!("Target set to {text}");
                            ui.close();
                        }
                    }
                });

                let view_label = self.label(I18nKey::MenuView);
                ui.menu_button(view_label, |ui| {
                    let theme_label = if self.is_dark_theme {
                        self.label(I18nKey::ActionThemeLight)
                    } else {
                        self.label(I18nKey::ActionThemeDark)
                    };
                    if ui.button(theme_label).clicked() {
                        self.is_dark_theme = !self.is_dark_theme;
                        ui.close();
                    }
                });

                let help_label = self.label(I18nKey::MenuHelp);
                ui.menu_button(help_label, |ui| {
                    let about_label = self.label(I18nKey::ActionAbout);
                    if ui.button(about_label).clicked() {
                        self.show_about_dialog = true;
                        ui.close();
                    }
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let cur_loc = self.locale;
                    let label = format!("{} {}", cur_loc.flag(), cur_loc.code().to_uppercase());
                    egui::ComboBox::from_id_salt("topbar_locale_selector")
                        .selected_text(label)
                        .show_ui(ui, |ui| {
                            for loc_item in AppLocale::all() {
                                let text =
                                    format!("{} {}", loc_item.flag(), loc_item.display_name());
                                let _ = ui.selectable_value(&mut self.locale, *loc_item, text);
                            }
                        });

                    let (icon, label) = if self.is_dark_theme {
                        ("\u{2600}", self.label(I18nKey::ActionThemeLight))
                    } else {
                        ("\u{1F319}", self.label(I18nKey::ActionThemeDark))
                    };
                    if ui.small_button(format!("{icon} {label}")).clicked() {
                        self.is_dark_theme = !self.is_dark_theme;
                    }
                });
            });
        });

        // 1. Top Panel: status + section tabs
        egui::Panel::top("top_panel").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading(self.label(I18nKey::AppTitle));
                ui.separator();
                let spectator = self.label(I18nKey::TabSpectator);
                let personal = self.label(I18nKey::TabPersonal);
                ui.selectable_value(&mut self.section, Section::Spectator, spectator);
                ui.selectable_value(&mut self.section, Section::Personal, personal);
                ui.separator();
                ui.label(&self.status_msg);
            });
            if self.section == Section::Personal {
                ui.horizontal(|ui| {
                    let trainer = self.label(I18nKey::TabTrainer);
                    let lab = self.label(I18nKey::TabLab);
                    ui.selectable_value(&mut self.personal_tab, PersonalTab::Trainer, trainer);
                    ui.selectable_value(
                        &mut self.personal_tab,
                        PersonalTab::Lab,
                        format!("{lab} (Banister/Gait/Pacing)"),
                    );
                });
            }
        });

        egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| match self.section {
                Section::Spectator => self.render_spectator(ui),
                Section::Personal => match self.personal_tab {
                    PersonalTab::Trainer => self.render_trainer(ui),
                    PersonalTab::Lab => self.render_lab(ui),
                },
            });
        });

        // Modal: About Dialog
        if self.show_about_dialog {
            egui::Window::new(self.label(I18nKey::ActionAbout))
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    ui.heading("SportsCraft Studio");
                    ui.label("Spectator soccer + personal gym/home trainer, Banister model.");
                    ui.label(format!("Version: v{}", env!("CARGO_PKG_VERSION")));
                    ui.label("Transport: MCP over stdio (no TCP port)");
                    ui.separator();
                    ui.label("Implemented: soccer tables/fixtures/predictor, trainer plans/sessions, Banister CTL/ATL/TSB, pacing, gait fixture.");
                    ui.label("Not implemented: live scores feeds, FIT/GPX import, persistence, PDF export.");
                    ui.label("Trainer guidance is general fitness info, not medical advice.");
                    ui.add_space(8.0);
                    if ui.button("Close").clicked() {
                        self.show_about_dialog = false;
                    }
                });
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    if args.smoke {
        let profile = SportsEngine::get_default_banister_profile("ATHLETE-ELITE-1");
        let prediction = SportsEngine::predict_race_pacing(&profile, 42195.0);
        let (marathon_seconds, prediction_error) = match prediction {
            Ok(value) => (json!(value.predicted_time_seconds), None),
            Err(error) => (serde_json::Value::Null, Some(error.to_string())),
        };
        let league_count = soccer::leagues().len();
        let table_rows = soccer::standings("ENG1")
            .map(|rows| rows.len())
            .unwrap_or(0);
        let demo_prediction = soccer::predict("Man City", "Arsenal").is_ok();
        let demo_plan = trainer::plan(
            trainer::Goal::General,
            trainer::Location::Home,
            3,
            trainer::Level::Beginner,
        )
        .is_ok();
        let demo_session = trainer::session(
            trainer::Goal::General,
            trainer::Location::Home,
            trainer::Level::Beginner,
            30,
        )
        .is_ok();
        let smoke_payload = json!({
            "app": "sportscraft",
            "version": env!("CARGO_PKG_VERSION"),
            "status": "ready",
            "transport": "stdio",
            "success": prediction_error.is_none() && demo_prediction && demo_plan && demo_session,
            "ctl_fitness": profile.current_fitness,
            "atl_fatigue": profile.current_fatigue,
            "tsb_form": profile.form_tsb,
            "marathon_predicted_seconds": marathon_seconds,
            "marathon_prediction_error": prediction_error,
            "soccer_league_count": league_count,
            "soccer_eng_table_rows": table_rows,
            "soccer_demo_prediction_ok": demo_prediction,
            "trainer_demo_plan_ok": demo_plan,
            "trainer_demo_session_ok": demo_session,
            "history_source": "demo",
        });
        println!("[sportscraft] Smoke OK: {smoke_payload}");
        if smoke_payload
            .get("success")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
        {
            return Ok(());
        }
        return Err("smoke prediction failed".into());
    }

    let app = SportsCraftApp::new();
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 800.0])
            .with_title("SportsCraft - Spectator Soccer & Personal Trainer"),
        ..Default::default()
    };

    eframe::run_native(
        "SportsCraft",
        native_options,
        Box::new(|_cc| Ok(Box::new(app))),
    )?;
    Ok(())
}

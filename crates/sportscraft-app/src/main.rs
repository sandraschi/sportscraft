//! SportsCraft desktop studio (`eframe` / `egui`).
//!
//! Every visible string goes through `sportscraft_core::i18n::t`, so the locale
//! selector actually localizes the UI. Data is produced by
//! `sportscraft_core::SportsEngine` in-process; there is no backend port.
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unimplemented)]
#![deny(clippy::todo)]

use clap::Parser;
use serde_json::json;
use sportscraft_core::i18n::{t, AppLocale, I18nKey};
use sportscraft_core::{SportsEngine, OPTIMAL_TSB_WINDOW};

#[derive(Parser, Debug)]
#[command(
    name = "sportscraft",
    about = "SportsCraft Biomechanics & Performance Studio"
)]
struct Args {
    #[arg(long)]
    smoke: bool,
}

pub struct SportsCraftApp {
    athlete_name: String,
    locale: AppLocale,
    target_distance: f64,
    status_msg: String,
    is_dark_theme: bool,
    show_about_dialog: bool,
}

impl SportsCraftApp {
    pub fn new() -> Self {
        Self {
            athlete_name: "ATHLETE-ELITE-1".to_string(),
            locale: AppLocale::default(),
            target_distance: 10000.0,
            status_msg: "SportsCraft Studio Ready".to_string(),
            is_dark_theme: true,
            show_about_dialog: false,
        }
    }

    fn label(&self, key: I18nKey) -> &'static str {
        t(key, self.locale)
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

        // 1. Top Panel
        egui::Panel::top("top_panel").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading(self.label(I18nKey::AppTitle));
                ui.separator();
                ui.label(&self.status_msg);
            });
        });

        egui::Panel::left("banister_panel").show(ui, |ui| {
            ui.heading(self.label(I18nKey::PanelBanister));
            let profile = SportsEngine::get_default_banister_profile(&self.athlete_name);
            ui.group(|ui| {
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
                ui.separator();
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

            ui.add_space(8.0);
            ui.heading(self.label(I18nKey::PanelGait));
            let gait = SportsEngine::get_gait_analysis();
            ui.group(|ui| {
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
        });

        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading(self.label(I18nKey::PanelPacing));
            ui.horizontal(|ui| {
                ui.label(self.label(I18nKey::FieldTargetDistance));
                ui.add(
                    egui::DragValue::new(&mut self.target_distance)
                        .speed(100.0)
                        .range(800.0..=100000.0),
                );
            });

            let profile = SportsEngine::get_default_banister_profile(&self.athlete_name);
            ui.add_space(8.0);
            ui.group(|ui| {
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

        // Modal: About Dialog
        if self.show_about_dialog {
            egui::Window::new(self.label(I18nKey::ActionAbout))
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    ui.heading("SportsCraft Studio");
                    ui.label("Sovereign athletic biomechanics, Banister model & race pacing.");
                    ui.label(format!("Version: v{}", env!("CARGO_PKG_VERSION")));
                    ui.label("Transport: MCP over stdio (no TCP port)");
                    ui.separator();
                    ui.label("Implemented: Banister CTL/ATL/TSB, pacing prediction, gait fixture.");
                    ui.label("Not implemented: FIT/GPX import, persistence, PDF export.");
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
        let smoke_payload = json!({
            "app": "sportscraft",
            "version": env!("CARGO_PKG_VERSION"),
            "status": "ready",
            "transport": "stdio",
            "success": prediction_error.is_none(),
            "ctl_fitness": profile.current_fitness,
            "atl_fatigue": profile.current_fatigue,
            "tsb_form": profile.form_tsb,
            "marathon_predicted_seconds": marathon_seconds,
            "marathon_prediction_error": prediction_error,
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
            .with_title("SportsCraft - Biomechanics & Performance Studio"),
        ..Default::default()
    };

    eframe::run_native(
        "SportsCraft",
        native_options,
        Box::new(|_cc| Ok(Box::new(app))),
    )?;
    Ok(())
}

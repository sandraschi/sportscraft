use clap::Parser;
use serde_json::json;
use sportscraft_core::SportsEngine;

#[derive(Parser, Debug)]
#[command(name = "sportscraft", about = "SportsCraft Biomechanics & Performance Studio")]
struct Args {
    #[arg(long)]
    smoke: bool,
}

pub struct SportsCraftApp {
    athlete_name: String,
    target_distance: f64,
    status_msg: String,
}

impl SportsCraftApp {
    pub fn new() -> Self {
        Self {
            athlete_name: "ATHLETE-ELITE-1".to_string(),
            target_distance: 10000.0,
            status_msg: "SportsCraft Studio Ready (Port 14131)".to_string(),
        }
    }
}

impl eframe::App for SportsCraftApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("top_panel").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("🏃 SportsCraft — Biomechanics, Banister Impulse & Kinematics");
                ui.separator();
                ui.label(&self.status_msg);
            });
        });

        egui::Panel::left("banister_panel").show(ui, |ui| {
            ui.heading("Banister Training Load");
            let profile = SportsEngine::get_default_banister_profile(&self.athlete_name);
            ui.group(|ui| {
                ui.label(format!("Athlete: {}", profile.athlete_id));
                ui.label(format!("Fitness (CTL, 42d): {:.1}", profile.current_fitness));
                ui.label(format!("Fatigue (ATL, 7d): {:.1}", profile.current_fatigue));
                ui.separator();
                let tsb_color = if profile.form_tsb >= 0.0 {
                    egui::Color32::from_rgb(50, 200, 50)
                } else {
                    egui::Color32::from_rgb(220, 80, 50)
                };
                ui.colored_label(tsb_color, format!("Form (TSB): {:.1}", profile.form_tsb));
            });

            ui.add_space(8.0);
            ui.heading("IMU Gait Kinematics");
            let gait = SportsEngine::get_gait_analysis();
            ui.group(|ui| {
                ui.label(format!("Cadence: {:.0} spm", gait.cadence_spm));
                ui.label(format!("Vertical Osc: {:.1} cm", gait.vertical_oscillation_cm));
                ui.label(format!("Ground Contact: {:.0} ms", gait.ground_contact_time_ms));
                ui.label(format!("Leg Stiffness: {:.1} kN/m", gait.leg_spring_stiffness_kn_m));
                ui.label(format!("Asymmetry: {:.1}%", gait.asymmetry_pct));
            });
        });

        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Race Pacing & Critical Velocity Simulator");
            ui.horizontal(|ui| {
                ui.label("Target Distance (meters):");
                ui.add(egui::DragValue::new(&mut self.target_distance).speed(100.0).range(800.0..=100000.0));
            });

            let profile = SportsEngine::get_default_banister_profile(&self.athlete_name);
            let pacing = SportsEngine::predict_race_pacing(&profile, self.target_distance);

            ui.add_space(8.0);
            ui.group(|ui| {
                ui.label(egui::RichText::new("Prediction Results").strong());
                let total_min = pacing.predicted_time_seconds / 60.0;
                let hrs = (total_min / 60.0).floor();
                let mins = total_min % 60.0;
                ui.label(format!("Estimated Finish: {:.0}h {:.1}m ({:.0} sec)", hrs, mins, pacing.predicted_time_seconds));
                ui.label(format!("Recommended Pace: {:.2} min/km", pacing.recommended_pace_min_km));
                ui.label(format!("Target Form TSB Window: +{:.0} to +{:.0}", pacing.optimal_form_tsb_window.0, pacing.optimal_form_tsb_window.1));
            });
        });
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    if args.smoke {
        println!("[sportscraft] Running headless smoke verification...");
        let profile = SportsEngine::get_default_banister_profile("ATHLETE-ELITE-1");
        let pacing = SportsEngine::predict_race_pacing(&profile, 42195.0);
        let smoke_payload = json!({
            "app": "sportscraft",
            "version": "0.1.0",
            "status": "ready",
            "control_port": 14131,
            "success": true,
            "ctl_fitness": profile.current_fitness,
            "marathon_predicted_seconds": pacing.predicted_time_seconds,
            "capabilities": ["BanisterImpulseResponse", "TrimpLoadAggregation", "GaitAsymmetryAnalysis", "PredictiveRacePacing"]
        });
        println!("[sportscraft] Smoke OK: {}", smoke_payload);
        return Ok(());
    }

    let app = SportsCraftApp::new();
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 800.0])
            .with_title("SportsCraft — Biomechanics & Performance Studio"),
        ..Default::default()
    };

    eframe::run_native(
        "SportsCraft",
        native_options,
        Box::new(|_cc| Ok(Box::new(app))),
    )?;
    Ok(())
}

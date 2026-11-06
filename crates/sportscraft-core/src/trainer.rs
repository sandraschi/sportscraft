//! Personal-trainer engine: gym / home workout catalog, weekly plans, sessions.
//!
//! Everything is deterministic and equipment-aware so the Trainer tab works
//! offline with zero accounts. Payloads carry `"source": "demo"` plus the
//! general-fitness disclaimer (not medical advice).
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unimplemented)]
#![deny(clippy::todo)]

use serde::{Deserialize, Serialize};

pub const DISCLAIMER: &str = "General fitness information only, not medical advice. Stop if you feel pain, dizziness, or unusual shortness of breath.";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Location {
    Gym,
    Home,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Goal {
    Strength,
    Hypertrophy,
    Conditioning,
    General,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    Beginner,
    Intermediate,
    Advanced,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Exercise {
    pub id: String,
    pub name: String,
    pub muscle: String,
    pub equipment: String,
    /// Where it can be done: "gym", "home", or "both".
    pub location: String,
    pub level: String,
    pub sets: u32,
    pub reps: String,
    pub rest_seconds: u32,
    /// Rough TRIMP burn rate used for session estimates.
    pub trimp_per_min: f64,
    pub cues: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlanDay {
    pub day: u32,
    pub focus: String,
    pub exercises: Vec<Exercise>,
    pub estimated_minutes: u32,
    pub estimated_trimp: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkoutPlan {
    pub goal: String,
    pub location: String,
    pub level: String,
    pub days_per_week: u32,
    pub days: Vec<PlanDay>,
    pub disclaimer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkoutSession {
    pub goal: String,
    pub location: String,
    pub level: String,
    pub minutes: u32,
    pub warmup: Vec<String>,
    pub main: Vec<Exercise>,
    pub cooldown: Vec<String>,
    pub estimated_trimp: f64,
    pub coaching: Vec<String>,
    pub disclaimer: String,
}

#[allow(clippy::too_many_arguments)]
fn ex(
    id: &str,
    name: &str,
    muscle: &str,
    equipment: &str,
    location: &str,
    level: &str,
    sets: u32,
    reps: &str,
    rest_seconds: u32,
    trimp_per_min: f64,
    cues: &str,
) -> Exercise {
    Exercise {
        id: id.to_string(),
        name: name.to_string(),
        muscle: muscle.to_string(),
        equipment: equipment.to_string(),
        location: location.to_string(),
        level: level.to_string(),
        sets,
        reps: reps.to_string(),
        rest_seconds,
        trimp_per_min,
        cues: cues.to_string(),
    }
}

pub fn parse_location(raw: &str) -> Result<Location, String> {
    match raw.trim().to_lowercase().as_str() {
        "gym" => Ok(Location::Gym),
        "home" => Ok(Location::Home),
        other => Err(format!("unknown location: {other} (use gym or home)")),
    }
}

pub fn parse_goal(raw: &str) -> Result<Goal, String> {
    match raw.trim().to_lowercase().as_str() {
        "strength" => Ok(Goal::Strength),
        "hypertrophy" | "muscle" => Ok(Goal::Hypertrophy),
        "conditioning" | "cardio" | "fat-loss" | "fatloss" => Ok(Goal::Conditioning),
        "general" | "fitness" => Ok(Goal::General),
        other => Err(format!(
            "unknown goal: {other} (use strength, hypertrophy, conditioning, general)"
        )),
    }
}

pub fn parse_level(raw: &str) -> Result<Level, String> {
    match raw.trim().to_lowercase().as_str() {
        "beginner" | "easy" => Ok(Level::Beginner),
        "intermediate" | "medium" => Ok(Level::Intermediate),
        "advanced" | "hard" => Ok(Level::Advanced),
        other => Err(format!(
            "unknown level: {other} (use beginner, intermediate, advanced)"
        )),
    }
}

fn location_label(location: Location) -> &'static str {
    match location {
        Location::Gym => "gym",
        Location::Home => "home",
    }
}

fn goal_label(goal: Goal) -> &'static str {
    match goal {
        Goal::Strength => "strength",
        Goal::Hypertrophy => "hypertrophy",
        Goal::Conditioning => "conditioning",
        Goal::General => "general",
    }
}

fn level_label(level: Level) -> &'static str {
    match level {
        Level::Beginner => "beginner",
        Level::Intermediate => "intermediate",
        Level::Advanced => "advanced",
    }
}

fn full_catalog() -> Vec<Exercise> {
    vec![
        ex(
            "bb-squat",
            "Barbell Back Squat",
            "legs",
            "barbell",
            "gym",
            "intermediate",
            4,
            "5",
            180,
            9.0,
            "Brace hard, knees track over toes, full depth you own.",
        ),
        ex(
            "deadlift",
            "Conventional Deadlift",
            "back+legs",
            "barbell",
            "gym",
            "advanced",
            4,
            "5",
            210,
            10.0,
            "Hinge, lats tight, bar drags up the shins.",
        ),
        ex(
            "bench",
            "Barbell Bench Press",
            "push/chest",
            "barbell",
            "gym",
            "intermediate",
            4,
            "6",
            150,
            8.0,
            "Feet planted, slight arch, touch chest, press the bar back.",
        ),
        ex(
            "lat-pulldown",
            "Lat Pulldown",
            "pull/back",
            "machine",
            "gym",
            "beginner",
            3,
            "10",
            90,
            7.0,
            "Chest up, pull elbows to ribs, control the return.",
        ),
        ex(
            "leg-press",
            "Leg Press",
            "legs",
            "machine",
            "gym",
            "beginner",
            3,
            "12",
            120,
            7.5,
            "Lower back glued to pad, never lock out hard.",
        ),
        ex(
            "seated-row",
            "Seated Cable Row",
            "pull/back",
            "machine",
            "gym",
            "beginner",
            3,
            "12",
            90,
            7.0,
            "Squeeze shoulder blades, no swinging.",
        ),
        ex(
            "treadmill",
            "Treadmill Intervals",
            "conditioning",
            "treadmill",
            "gym",
            "intermediate",
            6,
            "2-min fast / 2-min easy",
            0,
            11.0,
            "Fast parts hard but speakable-in-phrases; easy parts truly easy.",
        ),
        ex(
            "rower",
            "Rower Intervals",
            "conditioning+pull",
            "rower",
            "gym",
            "intermediate",
            5,
            "500 m",
            120,
            11.5,
            "Legs-body-arms on the drive, arms-body-legs on recovery.",
        ),
        ex(
            "db-press",
            "Dumbbell Shoulder Press",
            "push/shoulders",
            "dumbbell",
            "both",
            "beginner",
            3,
            "10",
            90,
            7.5,
            "Ribs down, press overhead without shrugging.",
        ),
        ex(
            "db-row",
            "Dumbbell Bent-Over Row",
            "pull/back",
            "dumbbell",
            "both",
            "beginner",
            3,
            "10/side",
            90,
            7.5,
            "Flat back, row elbow to hip.",
        ),
        ex(
            "kb-swing",
            "Kettlebell Swing",
            "legs+conditioning",
            "kettlebell",
            "both",
            "intermediate",
            5,
            "15",
            90,
            12.0,
            "Hike the bell, snap hips, bell floats to chest height.",
        ),
        ex(
            "band-pull",
            "Resistance Band Pull-Apart",
            "pull/shoulders",
            "band",
            "both",
            "beginner",
            3,
            "15",
            60,
            5.5,
            "Straight arms, pull band to chest, pinch blades.",
        ),
        ex(
            "pushup",
            "Push-Up",
            "push/chest",
            "bodyweight",
            "both",
            "beginner",
            3,
            "12",
            75,
            7.0,
            "Body in one line, chest to fist height, full lockout.",
        ),
        ex(
            "bw-squat",
            "Bodyweight Squat",
            "legs",
            "bodyweight",
            "both",
            "beginner",
            3,
            "15",
            60,
            6.5,
            "Sit between heels, knees out, chest proud.",
        ),
        ex(
            "lunge",
            "Walking Lunge",
            "legs",
            "bodyweight",
            "both",
            "beginner",
            3,
            "10/leg",
            75,
            7.0,
            "Long step, back knee kisses the floor.",
        ),
        ex(
            "glute-bridge",
            "Glute Bridge",
            "legs/glutes",
            "bodyweight",
            "both",
            "beginner",
            3,
            "15",
            60,
            5.5,
            "Squeeze glutes at top, no back arch.",
        ),
        ex(
            "plank",
            "Front Plank",
            "core",
            "bodyweight",
            "both",
            "beginner",
            3,
            "45 s",
            45,
            5.0,
            "Ribs down, glutes on, breathe behind the brace.",
        ),
        ex(
            "situp",
            "Sit-Up",
            "core",
            "bodyweight",
            "both",
            "beginner",
            3,
            "15",
            60,
            6.0,
            "Curl up segment by segment, no yanking the neck.",
        ),
        ex(
            "stepup",
            "Step-Up",
            "legs",
            "bench/chair",
            "both",
            "beginner",
            3,
            "10/leg",
            75,
            7.0,
            "Drive through the heel, control the way down.",
        ),
        ex(
            "rope",
            "Jump Rope",
            "conditioning",
            "rope",
            "both",
            "beginner",
            5,
            "2 min",
            60,
            12.5,
            "Light on the balls of the feet, wrists turn the rope.",
        ),
        ex(
            "burpee",
            "Burpee",
            "full-body",
            "bodyweight",
            "both",
            "advanced",
            4,
            "10",
            90,
            13.0,
            "Chest to floor, jump and clap overhead, land soft.",
        ),
        ex(
            "climber",
            "Mountain Climber",
            "core+conditioning",
            "bodyweight",
            "both",
            "intermediate",
            4,
            "30 s",
            60,
            11.0,
            "Hips low, knees drive fast and quiet.",
        ),
        ex(
            "jumping-jack",
            "Jumping Jack",
            "conditioning",
            "bodyweight",
            "both",
            "beginner",
            3,
            "60 s",
            45,
            9.0,
            "Warm-up pace first round, crisp pace after.",
        ),
    ]
}

/// Exercises usable at a location ("gym" sees gym+both, "home" sees home+both).
pub fn catalog(location: Location) -> Vec<Exercise> {
    full_catalog()
        .into_iter()
        .filter(|item| match location {
            Location::Gym => item.location == "gym" || item.location == "both",
            Location::Home => item.location == "home" || item.location == "both",
        })
        .collect()
}

fn tune_for_goal(mut item: Exercise, goal: Goal, level: Level) -> Exercise {
    match goal {
        Goal::Strength => {
            item.sets = match level {
                Level::Beginner => 3,
                Level::Intermediate => 4,
                Level::Advanced => 5,
            };
            if item.muscle.contains("conditioning") {
                item.sets = 3;
                item.reps = "90 s".to_string();
            } else {
                item.reps = match level {
                    Level::Beginner => "6".to_string(),
                    Level::Intermediate => "5".to_string(),
                    Level::Advanced => "4".to_string(),
                };
                item.rest_seconds = 180;
            }
        }
        Goal::Hypertrophy => {
            item.sets = 3;
            if !item.muscle.contains("conditioning") {
                item.reps = "10".to_string();
                item.rest_seconds = 90;
            }
        }
        Goal::Conditioning => {
            item.sets = 3;
            if item.muscle.contains("conditioning") || item.equipment == "bodyweight" {
                item.reps = "40 s on / 20 s off".to_string();
                item.rest_seconds = 60;
            }
        }
        Goal::General => {}
    }
    if level == Level::Beginner && item.level == "advanced" {
        item.sets = 2;
    }
    item
}

fn pick_for_focus(pool: &[Exercise], focus: &str, count: usize) -> Vec<Exercise> {
    let keywords: Vec<&str> = match focus {
        f if f.contains("Push") => vec!["push", "chest", "shoulders"],
        f if f.contains("Pull") => vec!["pull", "back"],
        f if f.contains("Lower") || f.contains("Legs") => vec!["legs", "glutes"],
        f if f.contains("Conditioning") => vec!["conditioning", "full-body", "core"],
        f if f.contains("Core") => vec!["core"],
        _ => vec![
            "push",
            "pull",
            "legs",
            "core",
            "conditioning",
            "back",
            "chest",
        ],
    };
    let mut picked: Vec<Exercise> = Vec::new();
    for want in &keywords {
        for item in pool {
            if picked.len() >= count {
                break;
            }
            if item.muscle.to_lowercase().contains(want)
                && !picked.iter().any(|p: &Exercise| p.id == item.id)
            {
                picked.push(item.clone());
            }
        }
    }
    for item in pool {
        if picked.len() >= count {
            break;
        }
        if !picked.iter().any(|p: &Exercise| p.id == item.id) {
            picked.push(item.clone());
        }
    }
    picked
}

fn day_minutes(exercises: &[Exercise]) -> u32 {
    let mut total: u32 = 10; // warm-up allowance
    for item in exercises {
        total += item.sets * 3 + item.rest_seconds / 60 * item.sets;
    }
    total.min(90)
}

fn day_trimp(exercises: &[Exercise], minutes: u32) -> f64 {
    if exercises.is_empty() {
        return 0.0;
    }
    let avg: f64 =
        exercises.iter().map(|item| item.trimp_per_min).sum::<f64>() / exercises.len() as f64;
    (minutes as f64 * avg * 0.55 * 10.0).round() / 10.0
}

/// Build a weekly plan. `days_per_week` outside 2..=5 is an error.
pub fn plan(
    goal: Goal,
    location: Location,
    days_per_week: u32,
    level: Level,
) -> Result<WorkoutPlan, String> {
    if !(2..=5).contains(&days_per_week) {
        return Err(format!("days_per_week must be 2-5 (got {days_per_week})"));
    }
    let pool: Vec<Exercise> = catalog(location)
        .into_iter()
        .map(|item| tune_for_goal(item, goal, level))
        .collect();
    if pool.is_empty() {
        return Err("no exercises available for this location".to_string());
    }
    let foci: Vec<String> = match days_per_week {
        2 => vec!["Full Body A".to_string(), "Full Body B".to_string()],
        3 => vec![
            "Upper Push".to_string(),
            "Lower + Core".to_string(),
            "Full Conditioning".to_string(),
        ],
        4 => vec![
            "Upper Push".to_string(),
            "Lower Strength".to_string(),
            "Upper Pull".to_string(),
            "Conditioning + Core".to_string(),
        ],
        _ => vec![
            "Push".to_string(),
            "Pull".to_string(),
            "Legs".to_string(),
            "Upper Mix".to_string(),
            "Conditioning".to_string(),
        ],
    };
    let mut days: Vec<PlanDay> = Vec::new();
    for (index, focus) in foci.into_iter().enumerate() {
        let exercises = pick_for_focus(&pool, &focus, 6);
        let minutes = day_minutes(&exercises);
        let trimp = day_trimp(&exercises, minutes);
        days.push(PlanDay {
            day: index as u32 + 1,
            focus,
            exercises,
            estimated_minutes: minutes,
            estimated_trimp: trimp,
        });
    }
    Ok(WorkoutPlan {
        goal: goal_label(goal).to_string(),
        location: location_label(location).to_string(),
        level: level_label(level).to_string(),
        days_per_week,
        days,
        disclaimer: DISCLAIMER.to_string(),
    })
}

/// Build a single coached session. `minutes` outside 15..=120 is an error.
pub fn session(
    goal: Goal,
    location: Location,
    level: Level,
    minutes: u32,
) -> Result<WorkoutSession, String> {
    if !(15..=120).contains(&minutes) {
        return Err(format!("minutes must be 15-120 (got {minutes})"));
    }
    let pool: Vec<Exercise> = catalog(location)
        .into_iter()
        .map(|item| tune_for_goal(item, goal, level))
        .collect();
    if pool.is_empty() {
        return Err("no exercises available for this location".to_string());
    }
    let focus = match goal {
        Goal::Strength => "Full Strength",
        Goal::Hypertrophy => "Full Hypertrophy",
        Goal::Conditioning => "Full Conditioning",
        Goal::General => "Full Body",
    };
    let main_count = match minutes {
        15..=29 => 4,
        30..=44 => 5,
        45..=59 => 6,
        _ => 7,
    };
    let main = pick_for_focus(&pool, focus, main_count);
    let estimated_trimp = day_trimp(&main, minutes);
    let coaching = vec![
        "Warm up 5 minutes easy (walk, rope, or bike) before the first set.".to_string(),
        "Leave 1-2 reps in reserve on every set for the first two weeks.".to_string(),
        "Rest as listed; cut rest only if breathing is fully back under control.".to_string(),
        "Progress next time: one more rep per set, then a little more load.".to_string(),
    ];
    Ok(WorkoutSession {
        goal: goal_label(goal).to_string(),
        location: location_label(location).to_string(),
        level: level_label(level).to_string(),
        minutes,
        warmup: vec![
            "5 min easy cardio (walk / rope / bike)".to_string(),
            "Leg swings + arm circles + 10 easy squats".to_string(),
        ],
        main,
        cooldown: vec![
            "3 min easy walk + deep breathing".to_string(),
            "Calves, quads, chest doorway stretch, 30 s each".to_string(),
        ],
        estimated_trimp,
        coaching,
        disclaimer: DISCLAIMER.to_string(),
    })
}

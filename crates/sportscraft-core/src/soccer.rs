//! European soccer spectator data (demo fixtures, no live feed).
//!
//! All tables and fixtures are deterministic demo content so the Spectator
//! section works offline. Every payload leaving `Command::dispatch` carries
//! `"source": "demo"`.
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unimplemented)]
#![deny(clippy::todo)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct League {
    pub code: String,
    pub name: String,
    pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub league_code: String,
    /// Demo strength rating (higher is stronger), used by the predictor.
    pub rating: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StandingRow {
    pub position: u32,
    pub team: String,
    pub played: u32,
    pub won: u32,
    pub drawn: u32,
    pub lost: u32,
    pub goals_for: u32,
    pub goals_against: u32,
    pub goal_diff: i32,
    pub points: u32,
    /// Last-five form, e.g. "WWDWL".
    pub form: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Fixture {
    pub league_code: String,
    pub matchday: u32,
    pub home: String,
    pub away: String,
    /// Demo kickoff label (local time), not a live schedule.
    pub kickoff: String,
    pub venue: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MatchPrediction {
    pub home: String,
    pub away: String,
    pub home_rating: f64,
    pub away_rating: f64,
    pub home_win_pct: f64,
    pub draw_pct: f64,
    pub away_win_pct: f64,
    pub predicted_home_goals: f64,
    pub predicted_away_goals: f64,
    pub pick: String,
}

fn league(code: &str, name: &str, country: &str) -> League {
    League {
        code: code.to_string(),
        name: name.to_string(),
        country: country.to_string(),
    }
}

fn team(id: &str, name: &str, league_code: &str, rating: f64) -> Team {
    Team {
        id: id.to_string(),
        name: name.to_string(),
        league_code: league_code.to_string(),
        rating,
    }
}

#[allow(clippy::too_many_arguments)]
fn row(
    position: u32,
    team: &str,
    played: u32,
    won: u32,
    drawn: u32,
    lost: u32,
    gf: u32,
    ga: u32,
    form: &str,
) -> StandingRow {
    StandingRow {
        position,
        team: team.to_string(),
        played,
        won,
        drawn,
        lost,
        goals_for: gf,
        goals_against: ga,
        goal_diff: gf as i32 - ga as i32,
        points: won * 3 + drawn,
        form: form.to_string(),
    }
}

fn fixture(
    league_code: &str,
    matchday: u32,
    home: &str,
    away: &str,
    kickoff: &str,
    venue: &str,
) -> Fixture {
    Fixture {
        league_code: league_code.to_string(),
        matchday,
        home: home.to_string(),
        away: away.to_string(),
        kickoff: kickoff.to_string(),
        venue: venue.to_string(),
    }
}

/// The five demo leagues (European soccer, v0.2 scope).
pub fn leagues() -> Vec<League> {
    vec![
        league("ENG1", "Premier League", "England"),
        league("ESP1", "La Liga", "Spain"),
        league("GER1", "Bundesliga", "Germany"),
        league("ITA1", "Serie A", "Italy"),
        league("FRA1", "Ligue 1", "France"),
    ]
}

/// Demo clubs with ratings. Unknown league code yields an empty vec (never panics).
pub fn teams_for(league_code: &str) -> Vec<Team> {
    match league_code.to_uppercase().as_str() {
        "ENG1" => vec![
            team("ars", "Arsenal", "ENG1", 89.0),
            team("mci", "Man City", "ENG1", 91.0),
            team("liv", "Liverpool", "ENG1", 90.0),
            team("che", "Chelsea", "ENG1", 84.0),
            team("tot", "Tottenham", "ENG1", 82.0),
            team("new", "Newcastle", "ENG1", 83.0),
        ],
        "ESP1" => vec![
            team("rma", "Real Madrid", "ESP1", 91.0),
            team("bar", "Barcelona", "ESP1", 90.0),
            team("atm", "Atletico Madrid", "ESP1", 86.0),
            team("gir", "Girona", "ESP1", 81.0),
            team("ath", "Athletic Bilbao", "ESP1", 80.0),
            team("sev", "Sevilla", "ESP1", 79.0),
        ],
        "GER1" => vec![
            team("bay", "Bayern Munich", "GER1", 90.0),
            team("bvb", "Borussia Dortmund", "GER1", 85.0),
            team("rbl", "RB Leipzig", "GER1", 84.0),
            team("b04", "Bayer Leverkusen", "GER1", 87.0),
            team("vfb", "VfB Stuttgart", "GER1", 81.0),
            team("sge", "Eintracht Frankfurt", "GER1", 80.0),
        ],
        "ITA1" => vec![
            team("int", "Inter", "ITA1", 89.0),
            team("mil", "AC Milan", "ITA1", 86.0),
            team("juv", "Juventus", "ITA1", 85.0),
            team("nap", "Napoli", "ITA1", 84.0),
            team("rom", "AS Roma", "ITA1", 82.0),
            team("ata", "Atalanta", "ITA1", 83.0),
        ],
        "FRA1" => vec![
            team("psg", "Paris SG", "FRA1", 89.0),
            team("om", "Marseille", "FRA1", 82.0),
            team("asm", "AS Monaco", "FRA1", 83.0),
            team("ol", "Lyon", "FRA1", 81.0),
            team("losc", "Lille", "FRA1", 81.0),
            team("ogcn", "Nice", "FRA1", 79.0),
        ],
        _ => vec![],
    }
}

/// Demo standings (played 9). `None` for unknown league codes.
pub fn standings(league_code: &str) -> Option<Vec<StandingRow>> {
    let rows = match league_code.to_uppercase().as_str() {
        "ENG1" => vec![
            row(1, "Man City", 9, 7, 1, 1, 22, 8, "WWWDW"),
            row(2, "Liverpool", 9, 6, 2, 1, 19, 9, "WDWWW"),
            row(3, "Arsenal", 9, 6, 1, 2, 17, 9, "WWLDW"),
            row(4, "Chelsea", 9, 4, 2, 3, 14, 12, "DLWWL"),
            row(5, "Newcastle", 9, 3, 3, 3, 12, 12, "DDWLD"),
            row(6, "Tottenham", 9, 2, 3, 4, 11, 14, "LDWDL"),
        ],
        "ESP1" => vec![
            row(1, "Real Madrid", 9, 7, 1, 1, 21, 8, "WWWDW"),
            row(2, "Barcelona", 9, 6, 2, 1, 23, 10, "WDWWW"),
            row(3, "Atletico Madrid", 9, 5, 3, 1, 14, 7, "DWWDW"),
            row(4, "Girona", 9, 4, 2, 3, 13, 11, "WLWDW"),
            row(5, "Athletic Bilbao", 9, 3, 3, 3, 10, 10, "DDWLD"),
            row(6, "Sevilla", 9, 2, 2, 5, 9, 14, "LLWDL"),
        ],
        "GER1" => vec![
            row(1, "Bayern Munich", 9, 7, 2, 0, 26, 9, "WWWDW"),
            row(2, "Bayer Leverkusen", 9, 6, 2, 1, 20, 10, "WDWWW"),
            row(3, "Borussia Dortmund", 9, 5, 2, 2, 17, 11, "WWDLW"),
            row(4, "RB Leipzig", 9, 4, 3, 2, 15, 11, "DWDWL"),
            row(5, "VfB Stuttgart", 9, 3, 3, 3, 13, 13, "DDWLW"),
            row(6, "Eintracht Frankfurt", 9, 2, 3, 4, 11, 15, "LDLWD"),
        ],
        "ITA1" => vec![
            row(1, "Inter", 9, 6, 2, 1, 18, 8, "WWWDW"),
            row(2, "AC Milan", 9, 5, 3, 1, 16, 9, "DWWDW"),
            row(3, "Juventus", 9, 5, 2, 2, 13, 8, "WWDWL"),
            row(4, "Napoli", 9, 4, 3, 2, 14, 10, "DWDWL"),
            row(5, "Atalanta", 9, 4, 2, 3, 15, 12, "WLWDW"),
            row(6, "AS Roma", 9, 3, 3, 3, 11, 11, "DDWLD"),
        ],
        "FRA1" => vec![
            row(1, "Paris SG", 9, 7, 1, 1, 22, 7, "WWWDW"),
            row(2, "AS Monaco", 9, 5, 2, 2, 16, 10, "WWDWL"),
            row(3, "Marseille", 9, 4, 3, 2, 15, 11, "DWDWL"),
            row(4, "Lille", 9, 4, 2, 3, 12, 10, "WLWDW"),
            row(5, "Lyon", 9, 3, 4, 2, 13, 11, "DDWDW"),
            row(6, "Nice", 9, 2, 3, 4, 9, 13, "LDLWD"),
        ],
        _ => return None,
    };
    Some(rows)
}

/// Demo fixtures (next matchday). `None` for unknown league codes.
pub fn fixtures(league_code: &str) -> Option<Vec<Fixture>> {
    let rows = match league_code.to_uppercase().as_str() {
        "ENG1" => vec![
            fixture(
                "ENG1",
                10,
                "Arsenal",
                "Chelsea",
                "2026-11-08 17:30",
                "Emirates Stadium",
            ),
            fixture(
                "ENG1",
                10,
                "Liverpool",
                "Tottenham",
                "2026-11-09 15:00",
                "Anfield",
            ),
            fixture(
                "ENG1",
                10,
                "Man City",
                "Newcastle",
                "2026-11-09 17:30",
                "Etihad Stadium",
            ),
        ],
        "ESP1" => vec![
            fixture(
                "ESP1",
                10,
                "Barcelona",
                "Atletico Madrid",
                "2026-11-08 21:00",
                "Olimpic Lluis Companys",
            ),
            fixture(
                "ESP1",
                10,
                "Real Madrid",
                "Sevilla",
                "2026-11-09 18:30",
                "Santiago Bernabeu",
            ),
            fixture(
                "ESP1",
                10,
                "Athletic Bilbao",
                "Girona",
                "2026-11-09 16:15",
                "San Mames",
            ),
        ],
        "GER1" => vec![
            fixture(
                "GER1",
                10,
                "Bayern Munich",
                "RB Leipzig",
                "2026-11-08 18:30",
                "Allianz Arena",
            ),
            fixture(
                "GER1",
                10,
                "Borussia Dortmund",
                "VfB Stuttgart",
                "2026-11-09 15:30",
                "Signal Iduna Park",
            ),
            fixture(
                "GER1",
                10,
                "Bayer Leverkusen",
                "Eintracht Frankfurt",
                "2026-11-09 17:30",
                "BayArena",
            ),
        ],
        "ITA1" => vec![
            fixture(
                "ITA1",
                10,
                "Inter",
                "Juventus",
                "2026-11-08 20:45",
                "San Siro",
            ),
            fixture(
                "ITA1",
                10,
                "Napoli",
                "AC Milan",
                "2026-11-09 20:45",
                "Stadio Maradona",
            ),
            fixture(
                "ITA1",
                10,
                "AS Roma",
                "Atalanta",
                "2026-11-09 18:00",
                "Stadio Olimpico",
            ),
        ],
        "FRA1" => vec![
            fixture(
                "FRA1",
                10,
                "Paris SG",
                "Lyon",
                "2026-11-08 20:45",
                "Parc des Princes",
            ),
            fixture(
                "FRA1",
                10,
                "Marseille",
                "AS Monaco",
                "2026-11-09 20:45",
                "Velodrome",
            ),
            fixture(
                "FRA1",
                10,
                "Lille",
                "Nice",
                "2026-11-09 17:00",
                "Stade Pierre-Mauroy",
            ),
        ],
        _ => return None,
    };
    Some(rows)
}

fn find_rating(name: &str) -> Option<(String, f64)> {
    let needle = name.trim().to_lowercase();
    if needle.is_empty() {
        return None;
    }
    for league in leagues() {
        for t in teams_for(&league.code) {
            if t.name.to_lowercase() == needle {
                return Some((t.name, t.rating));
            }
        }
    }
    // Forgiving prefix match ("man city" vs "city" is exact-only; keep it strict
    // so a typo returns a clean error instead of a wrong team).
    None
}

fn clamp(value: f64, low: f64, high: f64) -> f64 {
    if value < low {
        low
    } else if value > high {
        high
    } else {
        value
    }
}

/// Deterministic demo predictor: Elo-style logistic on ratings + home edge.
///
/// Returns an error string for unknown team names (never panics).
pub fn predict(home: &str, away: &str) -> Result<MatchPrediction, String> {
    let (home_name, home_rating) =
        find_rating(home).ok_or_else(|| format!("unknown home team: {home}"))?;
    let (away_name, away_rating) =
        find_rating(away).ok_or_else(|| format!("unknown away team: {away}"))?;
    let diff = home_rating - away_rating + 2.0; // home edge in rating points
    let home_p = 1.0 / (1.0 + 10.0_f64.powf(-diff / 12.0));
    let draw_p = 0.24 * (1.0 - (diff.abs() / 30.0).min(0.6));
    let home_win = (home_p * (1.0 - draw_p * 0.5)).clamp(0.05, 0.85);
    let away_win = ((1.0 - home_p) * (1.0 - draw_p * 0.5)).max(0.05);
    let mut draw = (1.0 - home_win - away_win).max(0.08);
    let total = home_win + draw + away_win;
    let home_win_pct = home_win / total * 100.0;
    let away_win_pct = away_win / total * 100.0;
    draw = 100.0 - home_win_pct - away_win_pct;

    let home_xg = clamp(1.55 + diff * 0.09, 0.3, 4.2);
    let away_xg = clamp(1.30 - diff * 0.07, 0.2, 3.8);
    let pick = if home_win_pct >= away_win_pct && home_win_pct >= draw {
        format!("{home_name} (home)")
    } else if away_win_pct >= draw {
        format!("{away_name} (away)")
    } else {
        "Draw".to_string()
    };
    Ok(MatchPrediction {
        home: home_name,
        away: away_name,
        home_rating,
        away_rating,
        home_win_pct,
        draw_pct: draw,
        away_win_pct,
        predicted_home_goals: home_xg,
        predicted_away_goals: away_xg,
        pick,
    })
}

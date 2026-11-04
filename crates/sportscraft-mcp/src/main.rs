//! SportsCraft MCP Server
//!
//! Speaks MCP `2024-11-05` JSON-RPC over stdio. Stdout carries only JSON-RPC
//! frames; every log line goes to stderr. There is no network port: the Rust
//! ArtCraft standard runs MCP over stdio and keeps the desktop app in-process.
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unimplemented)]
#![deny(clippy::todo)]

use std::io::{self, BufRead, Write};

use serde_json::{json, Value};
use sportscraft_core::Command;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|arg| arg == "--smoke") {
        run_smoke();
        return;
    }
    run_mcp_stdio();
}

/// Headless parity check: catalog + one dispatch, printed as JSON on stdout.
fn run_smoke() {
    let catalog = tools_list();
    let tool_count = catalog
        .get("tools")
        .and_then(Value::as_array)
        .map(|tools| tools.len())
        .unwrap_or(0);
    let payload = json!({
        "app": "sportscraft-mcp",
        "version": env!("CARGO_PKG_VERSION"),
        "transport": "stdio",
        "tool_count": tool_count,
        "status": Command::Status.dispatch(),
    });
    println!("{payload}");
}

/// The tool catalog. Mirrors `tools/list` exactly; pure so it is unit-testable.
fn tools_list() -> Value {
    json!({
        "tools": [
            {
                "name": "status",
                "description": "Engine status: version, Banister time constants, optimal TSB window, demo-data markers.",
                "inputSchema": {"type": "object", "properties": {}},
            },
            {
                "name": "athlete.profile",
                "description": "Banister CTL (fitness), ATL (fatigue) and TSB (form) for an athlete, folded over the demo TRIMP history. history_source=demo.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "athlete": {"type": "string", "description": "Athlete id (default ATHLETE-ELITE-1)."},
                    },
                },
            },
            {
                "name": "athlete.predict",
                "description": "Predict finish time and recommended pace for a target distance from current chronic fitness.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "athlete": {"type": "string", "description": "Athlete id (default ATHLETE-ELITE-1)."},
                        "distance_m": {"type": "number", "description": "Target distance in metres (must be > 0)."},
                    },
                    "required": ["distance_m"],
                },
            },
            {
                "name": "gait.analyze",
                "description": "IMU gait kinematics fixture: cadence, vertical oscillation, ground contact time, leg spring stiffness, asymmetry. source=demo (no sensor attached).",
                "inputSchema": {"type": "object", "properties": {}},
            },
        ]
    })
}

fn text_result(payload: &Value) -> Value {
    json!({
        "content": [{"type": "text", "text": payload.to_string()}],
        "isError": false,
    })
}

fn error_result(message: &str) -> Value {
    json!({
        "content": [{"type": "text", "text": message}],
        "isError": true,
    })
}

fn arg_string(args: &Value, key: &str, default: &str) -> String {
    args.get(key)
        .and_then(Value::as_str)
        .map(str::to_string)
        .unwrap_or_else(|| default.to_string())
}

/// Dispatch one `tools/call`. Unknown tools become `isError` content, never a panic.
fn call_tool(name: &str, args: &Value) -> Value {
    let athlete = arg_string(args, "athlete", "ATHLETE-ELITE-1");

    let command = match name {
        "status" => Command::Status,
        "athlete.profile" | "athlete_profile" => Command::Profile { athlete },
        "athlete.predict" | "athlete_predict" => {
            match args.get("distance_m").and_then(Value::as_f64) {
                Some(distance_m) => Command::Predict {
                    athlete,
                    distance_m,
                },
                None => {
                    return error_result("`distance_m` is required and must be a number");
                }
            }
        }
        "gait.analyze" | "gait_analyze" => Command::Gait,
        other => {
            return error_result(&format!("unknown tool: {other}"));
        }
    };

    text_result(&command.dispatch())
}

fn run_mcp_stdio() {
    eprintln!(
        "sportscraft-mcp {}: MCP server listening on stdio",
        env!("CARGO_PKG_VERSION")
    );
    let stdin = io::stdin();
    let mut out = io::stdout();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(line) => line,
            Err(_) => break,
        };
        if line.trim().is_empty() {
            continue;
        }

        let request: Value = match serde_json::from_str(&line) {
            Ok(value) => value,
            Err(error) => {
                write_frame(
                    &mut out,
                    &json!({
                        "jsonrpc": "2.0",
                        "id": Value::Null,
                        "error": {"code": -32700, "message": format!("parse error: {error}")}
                    }),
                );
                continue;
            }
        };

        let id = request.get("id").cloned().unwrap_or(Value::Null);
        let method = request.get("method").and_then(Value::as_str).unwrap_or("");
        let is_notification = request.get("id").is_none();

        let result: Option<Value> = match method {
            "initialize" => Some(json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {"tools": {}},
                "serverInfo": {
                    "name": "sportscraft",
                    "version": env!("CARGO_PKG_VERSION")
                },
                "instructions": "SportsCraft biomechanics: Banister CTL/ATL/TSB, race pacing, IMU gait (demo data)."
            })),
            "notifications/initialized" | "initialized" => None,
            "tools/list" => Some(tools_list()),
            "tools/call" => {
                let params = request.get("params").cloned().unwrap_or_else(|| json!({}));
                let name = params.get("name").and_then(Value::as_str).unwrap_or("");
                let arguments = params
                    .get("arguments")
                    .cloned()
                    .unwrap_or_else(|| json!({}));
                Some(call_tool(name, &arguments))
            }
            "ping" => Some(json!({})),
            other => {
                if is_notification {
                    None
                } else {
                    write_frame(
                        &mut out,
                        &json!({
                            "jsonrpc": "2.0",
                            "id": id,
                            "error": {"code": -32601, "message": format!("unknown method: {other}")}
                        }),
                    );
                    continue;
                }
            }
        };

        if is_notification {
            continue;
        }
        let body = result.unwrap_or_else(|| json!({}));
        write_frame(
            &mut out,
            &json!({"jsonrpc": "2.0", "id": id, "result": body}),
        );
    }
}

fn write_frame(out: &mut io::Stdout, frame: &Value) {
    if let Ok(text) = serde_json::to_string(frame) {
        let _ = writeln!(out, "{text}");
        let _ = out.flush();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sportscraft_core::OPTIMAL_TSB_WINDOW;

    fn content_text(result: &Value) -> String {
        result
            .get("content")
            .and_then(Value::as_array)
            .and_then(|items| items.first())
            .and_then(|item| item.get("text"))
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string()
    }

    #[test]
    fn catalog_exposes_four_tools() {
        let catalog = tools_list();
        let tools = catalog.get("tools").and_then(Value::as_array);
        assert!(tools.is_some());
        if let Some(tools) = tools {
            assert_eq!(tools.len(), 4);
        }
    }

    #[test]
    fn dotted_and_underscore_aliases_route_to_the_same_arm() {
        let dotted = call_tool("athlete.profile", &json!({"athlete": "A1"}));
        let underscore = call_tool("athlete_profile", &json!({"athlete": "A1"}));
        assert_eq!(content_text(&dotted), content_text(&underscore));
    }

    #[test]
    fn predict_requires_distance() {
        let result = call_tool("athlete.predict", &json!({}));
        assert_eq!(result.get("isError"), Some(&json!(true)));
    }

    #[test]
    fn predict_rejects_zero_distance() {
        let result = call_tool("athlete.predict", &json!({"distance_m": 0.0}));
        let text = content_text(&result);
        assert!(text.contains("invalid input"));
    }

    #[test]
    fn gait_is_marked_demo() {
        let result = call_tool("gait.analyze", &json!({}));
        assert!(content_text(&result).contains("\"source\":\"demo\""));
    }

    #[test]
    fn unknown_tool_is_error_content_not_panic() {
        let result = call_tool("nope.missing", &json!({}));
        assert_eq!(result.get("isError"), Some(&json!(true)));
    }

    #[test]
    fn optimal_window_is_exported() {
        let status = Command::Status.dispatch();
        let window = status.get("optimal_tsb_window").and_then(Value::as_array);
        assert!(window.is_some());
        if let Some(window) = window {
            assert_eq!(
                window.first().and_then(Value::as_f64),
                Some(OPTIMAL_TSB_WINDOW.0)
            );
        }
    }
}

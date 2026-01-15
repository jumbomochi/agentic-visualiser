use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Event types from Claude Code hooks
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventType {
    PreToolUse,
    PostToolUse,
    SubagentStop,
    #[serde(other)]
    Unknown,
}

/// A tool event captured from Claude Code hooks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolEvent {
    pub timestamp: DateTime<Utc>,
    pub session_id: String,
    pub event_type: EventType,
    pub tool_name: String,
    pub tool_use_id: String,
    #[serde(default)]
    pub cwd: String,
    #[serde(default)]
    pub transcript_path: String,
    #[serde(default)]
    pub subagent_type: String,
    #[serde(default)]
    pub subagent_prompt: String,
    pub tool_input: Option<serde_json::Value>,
    pub tool_response: Option<serde_json::Value>,
}

impl ToolEvent {
    /// Get a short summary of the tool input for display
    pub fn input_summary(&self) -> String {
        match self.tool_name.as_str() {
            "Read" => {
                if let Some(input) = &self.tool_input {
                    input["file_path"]
                        .as_str()
                        .map(|s| truncate_path(s, 40))
                        .unwrap_or_else(|| "?".to_string())
                } else {
                    "?".to_string()
                }
            }
            "Write" | "Edit" => {
                if let Some(input) = &self.tool_input {
                    input["file_path"]
                        .as_str()
                        .map(|s| truncate_path(s, 40))
                        .unwrap_or_else(|| "?".to_string())
                } else {
                    "?".to_string()
                }
            }
            "Bash" => {
                if let Some(input) = &self.tool_input {
                    let cmd = input["command"].as_str().unwrap_or("?");
                    truncate_str(cmd, 50)
                } else {
                    "?".to_string()
                }
            }
            "Grep" => {
                if let Some(input) = &self.tool_input {
                    let pattern = input["pattern"].as_str().unwrap_or("?");
                    format!("\"{}\"", truncate_str(pattern, 30))
                } else {
                    "?".to_string()
                }
            }
            "Glob" => {
                if let Some(input) = &self.tool_input {
                    let pattern = input["pattern"].as_str().unwrap_or("?");
                    format!("\"{}\"", truncate_str(pattern, 30))
                } else {
                    "?".to_string()
                }
            }
            "Task" => {
                if !self.subagent_type.is_empty() {
                    format!("{} agent", self.subagent_type)
                } else if let Some(input) = &self.tool_input {
                    input["subagent_type"]
                        .as_str()
                        .map(|s| format!("{} agent", s))
                        .unwrap_or_else(|| "subagent".to_string())
                } else {
                    "subagent".to_string()
                }
            }
            "WebFetch" => {
                if let Some(input) = &self.tool_input {
                    input["url"]
                        .as_str()
                        .map(|s| truncate_str(s, 40))
                        .unwrap_or_else(|| "?".to_string())
                } else {
                    "?".to_string()
                }
            }
            "WebSearch" => {
                if let Some(input) = &self.tool_input {
                    let query = input["query"].as_str().unwrap_or("?");
                    format!("\"{}\"", truncate_str(query, 30))
                } else {
                    "?".to_string()
                }
            }
            _ => String::new(),
        }
    }

    /// Get the subagent type if this is a Task tool call
    pub fn get_subagent_type(&self) -> Option<String> {
        if self.tool_name != "Task" {
            return None;
        }

        if !self.subagent_type.is_empty() {
            return Some(self.subagent_type.clone());
        }

        if let Some(input) = &self.tool_input {
            input["subagent_type"].as_str().map(|s| s.to_string())
        } else {
            None
        }
    }
}

fn truncate_str(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

fn truncate_path(path: &str, max_len: usize) -> String {
    if path.len() <= max_len {
        path.to_string()
    } else {
        let parts: Vec<&str> = path.split('/').collect();
        if let Some(filename) = parts.last() {
            if filename.len() < max_len - 5 {
                format!(".../{}", filename)
            } else {
                truncate_str(filename, max_len)
            }
        } else {
            truncate_str(path, max_len)
        }
    }
}

/// Parse a line from the events JSONL file
pub fn parse_event(line: &str) -> Option<ToolEvent> {
    serde_json::from_str(line).ok()
}

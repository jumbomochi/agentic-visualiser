#!/bin/bash
# Claude Code Visualizer Hook
# Captures tool events and writes them to the visualizer events file

# Configuration
VISUALIZER_DIR="$HOME/.claude-visualizer"
EVENTS_FILE="$VISUALIZER_DIR/events.jsonl"

# Ensure directory exists
mkdir -p "$VISUALIZER_DIR"

# Read JSON input from stdin
INPUT=$(cat)

# Parse common fields
SESSION_ID=$(echo "$INPUT" | jq -r '.session_id // "unknown"')
EVENT_TYPE=$(echo "$INPUT" | jq -r '.hook_event_name // "unknown"')
TOOL_NAME=$(echo "$INPUT" | jq -r '.tool_name // "system"')
TOOL_USE_ID=$(echo "$INPUT" | jq -r '.tool_use_id // ""')
CWD=$(echo "$INPUT" | jq -r '.cwd // ""')
TRANSCRIPT_PATH=$(echo "$INPUT" | jq -r '.transcript_path // ""')

# Get current timestamp (macOS compatible)
TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

# Extract tool input and response (may be null)
TOOL_INPUT=$(echo "$INPUT" | jq -c '.tool_input // null')
TOOL_RESPONSE=$(echo "$INPUT" | jq -c '.tool_response // null')

# For Task tool, extract subagent info
SUBAGENT_TYPE=""
SUBAGENT_PROMPT=""
if [ "$TOOL_NAME" = "Task" ]; then
    SUBAGENT_TYPE=$(echo "$INPUT" | jq -r '.tool_input.subagent_type // ""')
    SUBAGENT_PROMPT=$(echo "$INPUT" | jq -r '.tool_input.prompt // ""' | head -c 200)
fi

# Build the output event JSON (compact, single line for JSONL)
EVENT_JSON=$(jq -c -n \
    --arg timestamp "$TIMESTAMP" \
    --arg session_id "$SESSION_ID" \
    --arg event_type "$EVENT_TYPE" \
    --arg tool_name "$TOOL_NAME" \
    --arg tool_use_id "$TOOL_USE_ID" \
    --arg cwd "$CWD" \
    --arg transcript_path "$TRANSCRIPT_PATH" \
    --arg subagent_type "$SUBAGENT_TYPE" \
    --arg subagent_prompt "$SUBAGENT_PROMPT" \
    --argjson tool_input "$TOOL_INPUT" \
    --argjson tool_response "$TOOL_RESPONSE" \
    '{timestamp: $timestamp, session_id: $session_id, event_type: $event_type, tool_name: $tool_name, tool_use_id: $tool_use_id, cwd: $cwd, transcript_path: $transcript_path, subagent_type: $subagent_type, subagent_prompt: $subagent_prompt, tool_input: $tool_input, tool_response: $tool_response}')

# Append to events file
echo "$EVENT_JSON" >> "$EVENTS_FILE"

# Exit successfully (don't block Claude Code)
exit 0

#!/bin/bash
# Launch the visualizer in a new Terminal.app window
# This script is called by the SessionStart hook

VISUALIZER_DIR="$HOME/GitHub/agentic-visualiser"
VISUALIZER_BIN="$VISUALIZER_DIR/target/release/agentic-visualiser"
LOCK_FILE="/tmp/claude-visualizer.lock"

# Check if visualizer is already running
if [ -f "$LOCK_FILE" ]; then
    PID=$(cat "$LOCK_FILE")
    if ps -p "$PID" > /dev/null 2>&1; then
        # Already running, don't launch another
        exit 0
    fi
fi

# Check if binary exists
if [ ! -f "$VISUALIZER_BIN" ]; then
    # Try to build it
    cd "$VISUALIZER_DIR" && source "$HOME/.cargo/env" && cargo build --release 2>/dev/null
fi

# Launch in new Terminal window
osascript <<EOF
tell application "Terminal"
    activate
    set newWindow to do script "cd '$VISUALIZER_DIR' && '$VISUALIZER_BIN' --clear; rm -f '$LOCK_FILE'"
    set custom title of front window to "Claude Code Visualiser"
end tell
EOF

# Store a marker (the actual PID is tricky to get from osascript, so we use a simple lock)
echo "$$" > "$LOCK_FILE"

exit 0

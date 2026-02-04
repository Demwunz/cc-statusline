# cc-statusline

A lightweight statusline for [Claude Code](https://docs.anthropic.com/en/docs/claude-code) that shows where your context tokens are going and how much you're spending.

![Demo](demo.gif)

## What It Shows

```
[opus] ████████████████░░░░░░░░ 42% │ 2h30m │ $0.45 / $12.30
       ▲ context bar              ▲ time   ▲ session / daily cost
```

The colored bar shows your context usage at a glance:
- **Blue** - Base system prompt (~5k tokens)
- **Cyan** - Skills you've installed
- **Magenta** - Enabled plugins
- **Yellow** - MCP servers
- **Green** - Your conversation
- **Gray** - Available space

Run `cc-statusline --legend` to see this in your terminal.

## Installation

### From source

```bash
git clone https://github.com/Demwunz/cc-statusline.git
cd cc-statusline
cargo install --path .
```

### From crates.io (coming soon)

```bash
cargo install cc-statusline
```

## Setup

Add to your `~/.claude/settings.json`:

```json
{
  "statusLine": {
    "type": "command",
    "command": "cc-statusline"
  }
}
```

## How It Works

1. Claude Code pipes JSON to stdin with token counts and model info
2. The binary scans `~/.claude` for skills, plugins, and MCP servers
3. It reads `~/.claude/projects/**/*.jsonl` for daily cost totals
4. Outputs a single ANSI-colored line

## Configuration

Create `~/.config/cc-statusline/config.toml` to customize:

```toml
[format]
show_model = true
show_cost = true
show_daily_cost = true
bar_width = 24          # 0 = auto-detect

[colors]
base = "blue"
skills = "cyan"
plugins = "magenta"
mcp = "yellow"
conversation = "green"
empty = "gray"

[cache]
ttl_seconds = 5
```

## Usage

```bash
# Normal mode - reads JSON from stdin (Claude Code sends this automatically)
echo '{"model":{"display_name":"Opus"},"context_window":{"total_input_tokens":50000,"total_output_tokens":10000,"context_window_size":200000},"cost":{"total_cost_usd":0.45,"total_duration_ms":5400000}}' | cc-statusline

# Show what the colors mean
cc-statusline --legend

# Version
cc-statusline --version
```

## Performance

- Binary size: ~570KB
- Startup time: <50ms
- Memory: <5MB

## License

MIT

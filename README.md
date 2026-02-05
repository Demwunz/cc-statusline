<h1 align="center"> Claude Code Statusline </h1>

<p align="center">
  <img src="https://img.shields.io/badge/Claude-191919?style=flat-square&logo=anthropic&logoColor=white" alt="Claude" />
  <img src="https://img.shields.io/badge/CLI-000000?style=flat-square&logo=windowsterminal&logoColor=white" alt="CLI" />
  <img src="https://img.shields.io/badge/Rust-000000?style=flat-square&logo=rust&logoColor=white" alt="Rust" />
  <img src="https://img.shields.io/badge/macOS-000000?style=flat-square&logo=apple&logoColor=white" alt="macOS" />
  <img src="https://img.shields.io/badge/Linux-FCC624?style=flat-square&logo=linux&logoColor=black" alt="Linux" />
</p>

A lightweight statusline for [Claude Code](https://docs.anthropic.com/en/docs/claude-code) that shows where your context tokens are going and how much you're spending.

![Demo](demo.gif)

## Features

- 📊 **Context breakdown** - Visual bar showing token usage by segment
- 🚦 **Degradation thresholds** - Color-coded warnings as context fills up
- ⚡ **Subagent tracking** - See how many background agents are running
- 💰 **Cost tracking** - Session and daily spending at a glance
- 🔀 **Git integration** - Branch, staged/unstaged changes, ahead/behind
- 📁 **Working directory** - Fish-style abbreviated paths

## Performance

Built in Rust with zero runtime dependencies. A statusline runs on every prompt, so speed matters.

| Metric | cc-statusline | Typical Node.js CLI |
|--------|---------------|---------------------|
| ⚡ Binary size | ~620KB | 50-100MB+ (with node_modules) |
| 🚀 Startup | <50ms | 200-500ms |
| 💾 Memory | <5MB | 30-100MB |

## What It Shows

```
[opus] ████████████████░░░░░░░░ 42% 116k left ⚡3 │ 1h 30m │ $0.45 / $12.30 │ main ✚2 │ ~/d/myproject
       ▲ context bar             ▲ remaining   ▲ subagents ▲ elapsed ▲ cost           ▲ git     ▲ path
```

**Context bar** — colored segments show where tokens are going:
- 🔵 Blue = base system prompt (~5k)
- 🩵 Cyan = skills
- 🟣 Magenta = plugins
- 🟡 Yellow = MCP servers
- 🟢 Green = conversation
- ⬜ Gray = available

**Degradation thresholds** — percentage color warns you as context fills:

| Range | Color | Meaning |
|-------|-------|---------|
| 0–33% | plain | Optimal — plenty of room |
| 34–60% | yellow | Degrading — recall precision drops |
| 61–75% | red | Danger — approaching compaction |
| 76%+ | **bold red** | Critical — auto-compaction imminent |

Remaining tokens shown after percentage (e.g. `178k left`, `1.2m left`).

**Other indicators:**
- `⚡n` — active subagents (hidden when 0)
- 🟢 green branch = clean, 🔴 red = dirty, `↑↓` ahead/behind, `✚●?` staged/unstaged/untracked
- Paths are fish-style abbreviated (`~/d/cc-statusline`)

Run `cc-statusline --legend` to see this in your terminal.

## Installation

### Shell script

```bash
curl -fsSL https://raw.githubusercontent.com/Demwunz/cc-statusline/main/install.sh | bash
```

### Homebrew

```bash
brew tap Demwunz/tap
brew install cc-statusline
```

### Debian/Ubuntu

Download from [releases](https://github.com/Demwunz/cc-statusline/releases):

```bash
# x86_64
sudo dpkg -i cc-statusline_0.1.0_amd64.deb

# ARM64
sudo dpkg -i cc-statusline_0.1.0_arm64.deb
```

### Cargo

```bash
cargo install cc-statusline
```

### From source

```bash
git clone https://github.com/Demwunz/cc-statusline.git
cd cc-statusline
cargo install --path .
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
plan = "api"            # "api" (pay-per-token) or "max" (subscription)
plan_cost = 100.0       # Monthly cost of your Max plan (for savings calculation)
show_git = true         # Show git branch and status
show_cwd = true         # Show current working directory (fish-style)

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
echo '{"model":{"display_name":"Opus"},"context_window":{"total_input_tokens":50000,"total_output_tokens":10000,"context_window_size":200000,"used_percentage":11.0,"remaining_percentage":89.0},"cost":{"total_cost_usd":0.45,"total_duration_ms":5400000}}' | cc-statusline

# Show what the colors mean
cc-statusline --legend

# Version
cc-statusline --version
```

## License

MIT

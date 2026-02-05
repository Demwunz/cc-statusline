# VHS tape for cc-statusline demo
Output demo.gif

Set Shell "bash"
Set FontSize 16
Set Width 1100
Set Height 550
Set Theme "Dracula"
Set Padding 20

Type "# cc-statusline - context usage bar for Claude Code"
Enter
Sleep 1s

Type "# First, let's see what the colors mean"
Enter
Sleep 500ms

Type "cc-statusline --legend"
Enter
Sleep 4s

Type "# The statusline receives JSON from Claude Code via stdin"
Enter
Sleep 500ms

Type "# 11% usage — optimal zone, plenty of room"
Enter
Sleep 500ms

Type@50ms `echo '{"model":{"display_name":"Opus"},"context_window":{"total_input_tokens":15000,"total_output_tokens":5000,"context_window_size":200000,"used_percentage":11,"remaining_percentage":89},"cost":{"total_cost_usd":0.45,"total_duration_ms":5400000},"workspace":{"current_dir":"/Users/demo/dev/cc-statusline"}}' | cc-statusline`
Enter
Sleep 2s

Type "# 45% — degrading zone (yellow), recall precision drops"
Enter
Sleep 500ms

Type@50ms `echo '{"model":{"display_name":"Opus"},"context_window":{"total_input_tokens":60000,"total_output_tokens":20000,"context_window_size":200000,"used_percentage":45,"remaining_percentage":55},"cost":{"total_cost_usd":1.20,"total_duration_ms":900000},"workspace":{"current_dir":"/Users/demo/dev/cc-statusline"},"subagents":{"active":3}}' | cc-statusline`
Enter
Sleep 2s

Type "# 70% — danger zone (red), approaching compaction"
Enter
Sleep 500ms

Type@50ms `echo '{"model":{"display_name":"Opus"},"context_window":{"total_input_tokens":100000,"total_output_tokens":30000,"context_window_size":200000,"used_percentage":70,"remaining_percentage":30},"cost":{"total_cost_usd":5.80,"total_duration_ms":14400000},"workspace":{"current_dir":"/Users/demo/dev/cc-statusline"}}' | cc-statusline`
Enter
Sleep 2s

Type "# 85% — critical (bold red), auto-compaction imminent"
Enter
Sleep 500ms

Type@50ms `echo '{"model":{"display_name":"Sonnet"},"context_window":{"total_input_tokens":140000,"total_output_tokens":30000,"context_window_size":200000,"used_percentage":85,"remaining_percentage":15},"cost":{"total_cost_usd":12.50,"total_duration_ms":337680000},"workspace":{"current_dir":"/Users/demo/projects/webapp"}}' | cc-statusline`
Enter
Sleep 2s

Type "# Add to ~/.claude/settings.json to use with Claude Code!"
Enter
Sleep 2s

# VHS tape for cc-statusline demo
Output demo.gif

Set Shell "bash"
Set FontSize 16
Set Width 900
Set Height 450
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
Sleep 3s

Type "# The statusline receives JSON from Claude Code via stdin"
Enter
Sleep 500ms

Type "# Here's what it looks like with Opus at 32% context:"
Enter
Sleep 500ms

Type@50ms `echo '{"model":{"display_name":"Opus"},"context_window":{"total_input_tokens":50000,"total_output_tokens":10000,"context_window_size":200000},"cost":{"total_cost_usd":0.45,"total_duration_ms":5400000}}' | cc-statusline`
Enter
Sleep 2s

Type "# Sonnet at 75% context usage:"
Enter
Sleep 500ms

Type@50ms `echo '{"model":{"display_name":"Sonnet"},"context_window":{"total_input_tokens":120000,"total_output_tokens":30000,"context_window_size":200000},"cost":{"total_cost_usd":1.80,"total_duration_ms":9000000}}' | cc-statusline`
Enter
Sleep 2s

Type "# Haiku with low usage:"
Enter
Sleep 500ms

Type@50ms `echo '{"model":{"display_name":"Haiku"},"context_window":{"total_input_tokens":5000,"total_output_tokens":1000,"context_window_size":200000},"cost":{"total_cost_usd":0.02,"total_duration_ms":300000}}' | cc-statusline`
Enter
Sleep 2s

Type "# Add to ~/.claude/settings.json to use with Claude Code!"
Enter
Sleep 2s

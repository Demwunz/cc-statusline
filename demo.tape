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

Type "# Here's what it looks like with Opus at 32% context:"
Enter
Sleep 500ms

Type@50ms `echo '{"model":{"display_name":"Opus"},"context_window":{"total_input_tokens":50000,"total_output_tokens":10000,"context_window_size":200000},"cost":{"total_cost_usd":0.45,"total_duration_ms":5400000},"workspace":{"current_dir":"/Users/demo/dev/cc-statusline"}}' | cc-statusline`
Enter
Sleep 2s

Type "# Shows git branch (green=clean, red=dirty), language, and path"
Enter
Sleep 1s

Type "# When subagents are spawned, they appear after the percentage:"
Enter
Sleep 500ms

Type@50ms `echo '{"model":{"display_name":"Opus"},"context_window":{"total_input_tokens":80000,"total_output_tokens":20000,"context_window_size":200000},"cost":{"total_cost_usd":1.20,"total_duration_ms":900000},"workspace":{"current_dir":"/Users/demo/dev/cc-statusline"},"subagents":{"active":3}}' | cc-statusline`
Enter
Sleep 2s

Type "# Time shows remaining session time (5h limit). When low:"
Enter
Sleep 500ms

Type@50ms `echo '{"model":{"display_name":"Opus"},"context_window":{"total_input_tokens":180000,"total_output_tokens":40000,"context_window_size":200000},"cost":{"total_cost_usd":12.50,"total_duration_ms":17700000},"workspace":{"current_dir":"/Users/demo/dev/cc-statusline"}}' | cc-statusline`
Enter
Sleep 2s

Type "# When session exceeds 5h, it shows 'resetting':"
Enter
Sleep 500ms

Type@50ms `echo '{"model":{"display_name":"Sonnet"},"context_window":{"total_input_tokens":400000,"total_output_tokens":100000,"context_window_size":200000},"cost":{"total_cost_usd":45.00,"total_duration_ms":18500000},"workspace":{"current_dir":"/Users/demo/dev/cc-statusline"}}' | cc-statusline`
Enter
Sleep 2s

Type "# Add to ~/.claude/settings.json to use with Claude Code!"
Enter
Sleep 2s

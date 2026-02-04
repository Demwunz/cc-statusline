# VHS tape for cc-statusline demo
Output demo.gif

Set Shell "bash"
Set FontSize 16
Set Width 800
Set Height 400
Set Theme "Dracula"

Type "# cc-statusline - context usage bar for Claude Code"
Enter
Sleep 1s

Type "# Let's see what the colors mean"
Enter
Sleep 500ms

Type "cc-statusline --legend"
Enter
Sleep 3s

Type "# Now let's see it with some sample data"
Enter
Sleep 500ms

Type `echo '{"model":"claude-opus-4-5-20251101","inputTokens":50000,"outputTokens":10000,"contextWindow":200000,"cost":0.45}' | cc-statusline`
Enter
Sleep 2s

Type "# Different model and usage"
Enter
Sleep 500ms

Type `echo '{"model":"claude-sonnet-4-20251101","inputTokens":120000,"outputTokens":30000,"contextWindow":200000,"cost":1.20}' | cc-statusline`
Enter
Sleep 2s

Type "# Low usage"
Enter
Sleep 500ms

Type `echo '{"model":"claude-haiku","inputTokens":5000,"outputTokens":1000,"contextWindow":200000,"cost":0.02}' | cc-statusline`
Enter
Sleep 2s

Type "# That's it! Add to ~/.claude/settings.json to use in Claude Code"
Enter
Sleep 2s

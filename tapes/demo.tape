# Run from the repository root after `cargo build --locked`:
#   vhs tapes/demo.tape
#   sh tapes/optimize.sh tapes/demo.gif
Output tapes/demo.gif

Require bash
Require ./target/debug/sig

Set Shell "bash"
Set Theme "Dracula"
Set Width 1000
Set Height 480
Set FontSize 20
Set Padding 20
Set Framerate 12
Set CursorBlink false
Set TypingSpeed 40ms

# Use the local build and explicit config; enable syntax colors in recordings.
Hide
Type `export PATH="$PWD/target/debug:$PATH"`
Enter
Type "unset NO_COLOR"
Enter
Type "clear"
Enter
Sleep 400ms
Show

Type "bash tapes/stream.sh | sig -c default.toml"
Enter
Sleep 2s

# New matching lines continue to arrive while the query changes.
Type "INFO"
Sleep 2500ms
Ctrl+U
Type "WARN|ERROR"
Sleep 3s

# Pause and resume ingestion, then search the buffered history.
Ctrl+S
Sleep 1200ms
Ctrl+S
Sleep 1500ms
Ctrl+F
Sleep 1s
Type "ERROR"
Sleep 1500ms
Down
Sleep 1500ms

Hide
Ctrl+C

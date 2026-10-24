# Run from the repository root after `cargo build --locked`:
#   vhs tapes/demo.static.tape
#   sh tapes/optimize.sh tapes/demo.static.gif
Output tapes/demo.static.gif

Require cat
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

Hide
Type `export PATH="$PWD/target/debug:$PATH"`
Enter
Type "unset NO_COLOR"
Enter
Type "clear"
Enter
Sleep 400ms
Show

# EOF automatically opens archived mode for static input.
Type "cat tapes/demo.log | sig -c default.toml"
Enter
Sleep 1500ms
Type "ERROR"
Sleep 1500ms
Down
Sleep 800ms

# Refine a regex and navigate the matches without rerunning the command.
Ctrl+U
Type "WARN|ERROR"
Sleep 1500ms
Down
Sleep 800ms
Ctrl+U
Type "worker.*timeout|worker.*retry"
Sleep 2500ms

Hide
Ctrl+C

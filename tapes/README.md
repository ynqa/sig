# Demo GIFs

Requires Cargo, VHS, ttyd, FFmpeg (`ffmpeg` / `ffprobe`), and ImageMagick (`magick`).
Run from the repository root:

```sh
cargo build --locked
vhs tapes/demo.tape
vhs tapes/demo.static.tape
sh tapes/optimize.sh
```

- `demo.tape` → `demo.gif`: streaming filters, pause/resume, and archived search.
- `demo.static.tape` → `demo.static.gif`: regex filtering and navigation over static input.
- `demo.log` supplies the sample data; `stream.sh` replays it for the streaming demo.
- `optimize.sh` overwrites both GIFs with optimized versions: 12 fps, 128 colors, background noise cleanup, and similar-frame compression. To optimize only one, run `sh tapes/optimize.sh tapes/demo.gif`.

Edit the tapes to change the interactions, timing, or terminal appearance, then rerun the commands above.

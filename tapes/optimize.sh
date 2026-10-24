#!/bin/sh
# Optimize VHS recordings while preserving frame timing and readable text.
# Usage from the repository root: sh tapes/optimize.sh [path/to/demo.gif ...]
set -eu
if [ "$#" -eq 0 ]; then
    set -- tapes/demo.gif tapes/demo.static.gif
fi

demo_tmp=$(mktemp -d)
trap 'rm -rf "$demo_tmp"' EXIT

for demo_path in "$@"; do
    demo_size=$(ffprobe -v error -select_streams v:0 \
        -show_entries stream=width,height -of csv=s=x:p=0 "$demo_path")
    ffmpeg -hide_banner -loglevel error -y -i "$demo_path" \
        -filter_complex "[0:v]fps=12,format=rgba,colorkey=0x000000:0.2:0[fg];color=c=0x1e1f29:s=${demo_size}:r=12[bg];[bg][fg]overlay=shortest=1,split[a][b];[a]palettegen=max_colors=128:stats_mode=full[p];[b][p]paletteuse=dither=none:diff_mode=rectangle" \
        -loop 0 "$demo_tmp/palette.gif"
    magick "$demo_tmp/palette.gif" -fuzz 3% -layers Optimize "$demo_tmp/optimized.gif"
    mv "$demo_tmp/optimized.gif" "$demo_path"
done

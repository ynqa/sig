#!/bin/sh
# Replay the local fixture so the streaming demo needs no external services.
set -eu
demo_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
while :; do
    while IFS= read -r line; do
        printf '%s\n' "$line"
        sleep 0.4
    done < "$demo_dir/demo.log"
done

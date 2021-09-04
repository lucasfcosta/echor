#!/usr/bin/env bash

SCRIPT_DIR="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"

OUTDIR="$SCRIPT_DIR/../tests/expected"
[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR"

echo "Hello there" > $OUTDIR/hello1.txt
echo "Hello"  "there" > $OUTDIR/hello2.txt
echo -n "Hello  there" > $OUTDIR/hello1.n.txt
echo -n "Hello" "there" > $OUTDIR/hello2.n.txt

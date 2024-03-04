#!/bin/sh

PROFILE=${PROFILE:-'release2'}
SIZE=$1
SOLUTION=$2

cargo build --profile $PROFILE --package obrc2024 --bin $SOLUTION

# bench
hyperfine --runs 5 --export-csv "data/measurements_$SIZE.time.csv.tmp" --output "data/measurements_$SIZE.out.txt" --command-name "${SOLUTION}_${PROFILE}" "../target/$PROFILE/$SOLUTION 'data/measurements_$SIZE.txt'"
tail -n 1 "data/measurements_$SIZE.time.csv.tmp" >>"data/measurements_$SIZE.time.csv"; rm "data/measurements_$SIZE.time.csv.tmp"

# check correctness
git diff -U0 --word-diff --no-index -- "data/measurements_$SIZE.out.txt" "data/measurements_$SIZE.ref.out.txt" || exit 1

# profile
PROFILE_ENV=$(echo $PROFILE | tr '[:lower:]' '[:upper:]')
env "CARGO_PROFILE_${PROFILE_ENV}_STRIP=none" "CARGO_PROFILE_${PROFILE_ENV}_DEBUG=true" cargo flamegraph --profile $PROFILE --package obrc2024 --bin $SOLUTION --root -o data/measurements_$SIZE.$SOLUTION.$PROFILE.flamegraph.svg --inverted --reverse -- "data/measurements_$SIZE.txt" >/dev/null

# show
open data/measurements_$SIZE.$SOLUTION.$PROFILE.flamegraph.svg
./data/viz.py "data/measurements_$SIZE.time.csv"

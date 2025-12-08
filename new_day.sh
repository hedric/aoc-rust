#!/bin/bash

# exit on any failed command,
# exit when detecting any undefined variable
# return pipeline fails
set -euo pipefail

if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <year> <day>"
    echo "Example: $0 2025 01"
    exit 1
fi

YEAR=$1
DAY=$2

YEAR_DIR="y${YEAR}"
SRC_DIR="${YEAR_DIR}/src"
INPUT_DIR="${YEAR_DIR}/inputs"

DAY_FILE="${SRC_DIR}/day${DAY}.rs"
INPUT_FILE="${INPUT_DIR}/day${DAY}.txt"

mkdir -p "$SRC_DIR"
mkdir -p "$INPUT_DIR"

# Create day template file
if [ ! -f "$DAY_FILE" ]; then
    cat <<EOF > "$DAY_FILE"
use aoclib::measure_time;

pub fn part1(input: &str) -> usize {
    0 // TODO: implement
}

pub fn part2(input: &str) -> usize {
    0 // TODO: implement
}

pub fn run() {
    let input = include_str!("../inputs/day${DAY}.txt");

    let p1 = measure_time("Part 1", || part1(input));
    println!("Part 1 result: {}", p1);

    let p2 = measure_time("Part 2", || part2(input));
    println!("Part 2 result: {}", p2);
}

EOF
    echo "Created $DAY_FILE"
else
    echo "$DAY_FILE already exists, skipping"
fi

# Create empty input file
if [ ! -f "$INPUT_FILE" ]; then
    touch "$INPUT_FILE"
    echo "Created $INPUT_FILE"
else
    echo "$INPUT_FILE already exists, skipping"
fi

# Update the year lib.rs to include the new day
LIB_FILE="${SRC_DIR}/lib.rs"
DAY_MOD="pub mod day${DAY};"

if [ ! -f "$LIB_FILE" ]; then
    echo "$DAY_MOD" > "$LIB_FILE"
    echo "Created $LIB_FILE with $DAY_MOD"
elif ! grep -q "$DAY_MOD" "$LIB_FILE"; then
    echo "$DAY_MOD" >> "$LIB_FILE"
    echo "Added $DAY_MOD to $LIB_FILE"
else
    echo "$DAY_MOD already in $LIB_FILE, skipping"
fi

echo "Done!"


set shell := ["bash", "-eu", "-o", "pipefail", "-c"]

new-day day:
    #!/usr/bin/env bash
    set -euo pipefail
    day="{{day}}"
    test "$day" -ge 1
    test "$day" -le 25
    source="crates/2023/src/bin/day$day.rs"
    input_dir="input/2023/$day"
    test ! -e "$source"
    test ! -e "$input_dir"
    mkdir -p "$input_dir"
    printf '%s\n' \
        'fn main() {' \
        "    let input = aoc::read_input_arg(2023, $day).expect(\"could not read input\");" \
        '}' > "$source"
    touch "$input_dir/real.txt" "$input_dir/sample.txt"
    echo "Created $source and $input_dir/{real,sample}.txt"

run day filename="real":
    cargo run --bin day{{day}} -- {{filename}}
// No regex or external crates.
// You may keep only the previous, current, and next row in memory.
// Identify numbers yourself by scanning characters left to right.
// You may inspect neighboring cells, but you cannot search the whole schematic for each number.
// Stop once your code works for the sample input.

use std::collections::HashMap;

fn push_if_sym(
    grid: &[Vec<char>],
    adj: &mut String,
    gear_pos: &mut Option<(usize, usize)>,
    r: usize,
    c: usize,
) {
    for ri in r.saturating_sub(1)..=r + 1 {
        if ri < grid.len() {
            let ch = grid[ri][c];
            if !matches!(ch, '0'..='9' | '.') {
                adj.push(ch)
            }
            if matches!(ch, '*') {
                *gear_pos = Some((ri, c));
            }
        }
    }
}

fn main() {
    let input = aoc::read_input_arg(2023, 3).expect("could not read input");

    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let nrows = grid.len();
    let ncols = grid[0].len();

    let mut active = String::new();
    let mut adj = String::new();
    let mut gear_pos = Some((0, 0));
    let mut sum = 0;
    let mut part2 = 0;

    let mut gear_map = HashMap::new();

    for r in 0..nrows {
        for c in 0..ncols {
            let ch = grid[r][c];
            let is_num = ch.is_ascii_digit();

            if active.is_empty() && c >= 1 {
                push_if_sym(&grid, &mut adj, &mut gear_pos, r, c - 1);
            }

            push_if_sym(&grid, &mut adj, &mut gear_pos, r, c);

            if is_num {
                active.push(ch);
            }

            if !is_num || c == ncols - 1 {
                if !active.is_empty() && !adj.is_empty() {
                    let num = active.parse::<u32>().unwrap();
                    sum += num;
                    if let Some(g) = gear_pos {
                        let entry = gear_map.get(&g);

                        match entry {
                            None => {
                                gear_map.insert(g, num);
                            }
                            Some(&entry) => {
                                part2 += entry * num;
                            }
                        }
                    }
                }
                active.clear();
                adj.clear();
                gear_pos = None;
            }
        }
    }

    println!("Part 1: {sum}");
    println!("Part 2: {part2}");
}

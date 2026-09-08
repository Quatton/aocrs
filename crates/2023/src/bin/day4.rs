use std::collections::HashSet;

fn main() {
    let input = aoc::read_input_arg(2023, 4).expect("could not read input");

    let mut sum = 0;
    let mut card_values = vec![];

    for line in input.lines() {
        let (_, card) = line.split_once(": ").unwrap();

        let (winning, have) = card.split_once(" | ").unwrap();

        let mut lsum = 0;

        let winning: HashSet<_> = winning
            .split_whitespace()
            .map(|w| w.parse::<u32>().unwrap())
            .collect();

        let have: HashSet<_> = have
            .split_whitespace()
            .map(|w| w.parse::<u32>().unwrap())
            .collect();

        for w in winning {
            if have.contains(&w) {
                lsum += 1;
            }
        }

        card_values.push(lsum);
        if lsum > 0 {
            sum += 1 << (lsum - 1);
        }
    }

    let mut part2 = 0;

    let mut multipliers = vec![1; card_values.len()];

    for i in 0..card_values.len() {
        let size = card_values[i];
        for offset in 1..=size {
            multipliers[i + offset] += multipliers[i];
        }
    }

    for mul in multipliers {
        part2 += mul;
    }

    println!("Part 1: {sum}");
    println!("Part 2: {part2}");
}

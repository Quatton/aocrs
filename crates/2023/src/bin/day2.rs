fn main() {
    let input = aoc::read_input_arg(2023, 2).expect("could not read input");

    let games = parse(&input);

    let sum = games.iter().enumerate().fold(0, |acc, (i, game)| {
        if game_possible(game) {
            acc + i + 1
        } else {
            acc
        }
    });

    println!("{}", sum);

    let part2 = games
        .iter()
        .map(|game| game_min(game).iter().product::<u32>())
        .sum::<u32>();

    println!("{}", part2);
}

fn possible(set: &[u32; 3]) -> bool {
    set[0] <= 12 && set[1] <= 13 && set[2] <= 14
}

fn game_possible(game: &Vec<[u32; 3]>) -> bool {
    game.iter().all(possible)
}

fn game_min(game: &Vec<[u32; 3]>) -> [u32; 3] {
    game.iter().fold([0, 0, 0], |mut acc, cur| {
        for i in 0..3 {
            acc[i] = acc[i].max(cur[i])
        }
        acc
    })
}

fn parse(input: &str) -> Vec<Vec<[u32; 3]>> {
    input
        .lines()
        .map(|l| {
            let (_, sets) = l.split_once(':').unwrap();

            sets.trim()
                .split(';')
                .map(|set| {
                    set.split(", ").fold([0, 0, 0], |mut acc, pair| {
                        let (num, color) = pair.trim().split_once(' ').unwrap();

                        let num_int = num.parse::<u32>().unwrap();

                        let idx: usize = match color {
                            "red" => 0,
                            "green" => 1,
                            "blue" => 2,
                            _ => unreachable!("how?"),
                        };

                        acc[idx] += num_int;

                        acc
                    })
                })
                .collect()
        })
        .collect()
}

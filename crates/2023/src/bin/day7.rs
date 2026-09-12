fn get_rank(suit: char) -> u8 {
    match suit {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        num => num as u8 - b'0',
    }
}

fn get_rank_joker(suit: char) -> u8 {
    match suit {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'T' => 10,
        'J' => 1,
        num => num as u8 - b'0',
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Hand {
    eval: Eval,
    uhu8: [u8; 5],
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Eval {
    High,
    Pair,
    TwoPair,
    Three,
    Full,
    Four,
    Five,
}

fn helper<P>(uh: &str, parser: P) -> (Hand, [u8; 15])
where
    P: Fn(char) -> u8 + Clone,
{
    let mut uhu8 = [0; 5];
    let mut counts = [0u8; 15];

    for (i, c) in uh.chars().take(5).enumerate() {
        let rank = parser(c);
        uhu8[i] = rank;
        counts[rank as usize] += 1;
    }

    let mut pairs = 0;
    let mut threes = false;
    let mut fours = false;
    let mut fives = false;

    for rank in 2..=14 {
        match counts[rank as usize] {
            5 => fives = true,
            4 => fours = true,
            3 => threes = true,
            2 => {
                pairs += 1;
            }
            _ => {}
        }
    }

    let eval = if fives {
        Eval::Five
    } else if fours {
        Eval::Four
    } else if threes && pairs == 1 {
        Eval::Full
    } else if threes {
        Eval::Three
    } else if pairs == 2 {
        Eval::TwoPair
    } else if pairs == 1 {
        Eval::Pair
    } else {
        Eval::High
    };

    (Hand { eval, uhu8 }, counts)
}

impl Hand {
    fn from_unordered_hand(uh: &str) -> Self {
        helper(uh, get_rank).0
    }

    fn with_joker_rule(uh: &str) -> Self {
        let (Hand { eval, uhu8 }, counts) = helper(uh, get_rank_joker);

        let eval = match (eval, counts[1]) {
            (Eval::Four, 1) | (Eval::Three, 2) | (Eval::Pair, 3) | (Eval::High, 4..) => Eval::Five,
            (Eval::Pair, 2) | (Eval::Three, 1) | (Eval::High, 3) => Eval::Four,
            (Eval::TwoPair, 1) => Eval::Full,
            (Eval::Pair, 1) | (Eval::High, 2) => Eval::Three,
            (Eval::High, 1) => Eval::Pair,
            _ => eval,
        };

        Hand { eval, uhu8 }
    }
}

fn main() {
    let input = aoc::read_input_arg(2023, 7).expect("could not read input");

    let input = input.lines().map(|line| {
        line.split_once(" ")
            .map(|(hand, bid)| (hand, bid.parse::<u64>().unwrap()))
            .unwrap()
    });

    let part1 = solve(input.clone(), Hand::from_unordered_hand);
    let part2 = solve(input, Hand::with_joker_rule);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn solve<'a, F, I>(input: I, f: F) -> u64
where
    F: Fn(&str) -> Hand,
    I: Iterator<Item = (&'a str, u64)> + Clone,
{
    let mut hands = input
        .clone()
        .map(|(hand, bid)| (f(hand), bid))
        .collect::<Vec<_>>();

    input
        .zip(hands.iter())
        .for_each(|((l, _), (h, _))| println!("{l}: {h:?}"));

    hands.sort_unstable_by_key(|h| h.0);

    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (rank, &(_, bid))| acc + ((rank + 1) as u64) * bid)
}

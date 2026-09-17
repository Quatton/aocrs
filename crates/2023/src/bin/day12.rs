use std::{
    fmt::{Display, Write},
    str::FromStr,
};

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl Display for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            Spring::Damaged => '#',
            Spring::Operational => '.',
            Spring::Unknown => '?',
        };
        f.write_char(repr)
    }
}

impl Spring {
    fn from_char(c: char) -> Self {
        match c {
            '#' => Spring::Damaged,
            '.' => Spring::Operational,
            '?' => Spring::Unknown,
            _ => unreachable!(),
        }
    }

    fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

fn is_unknown(s: &Spring) -> bool {
    matches!(s, Spring::Unknown)
}

struct Line {
    springs: Vec<Spring>,
    damaged: Vec<usize>,
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(' ')
            .map(|(pipes, damaged)| Line {
                springs: pipes.chars().map(Spring::from_char).collect(),
                damaged: damaged.split(',').map(|d| d.parse().unwrap()).collect(),
            })
            .ok_or(())
    }
}

impl Line {
    fn is_valid(&self, springs: &[Spring]) -> bool {
        let mut consecutive_damaged = 0;
        let mut verdict = true;
        let mut damaged_iter = self.damaged.iter().peekable();

        for spring in springs.iter().chain(std::iter::once(&Spring::Operational)) {
            let mut should_break = false;
            let mut reset = true;
            match spring {
                Spring::Unknown => {
                    should_break = true;
                }
                Spring::Damaged => {
                    reset = false;
                    consecutive_damaged += 1;
                }
                _ => {}
            }

            let damaged_next = damaged_iter.peek();

            let cmp = **damaged_next.unwrap_or(&&0);

            if consecutive_damaged > cmp {
                verdict = false;
                should_break = true;
            }

            if should_break {
                break;
            }

            if reset {
                consecutive_damaged = 0;
                damaged_iter.next();
            }
        }

        verdict
    }

    fn count_helper(&self, springs: Vec<Spring>) -> usize {
        let is_valid = self.is_valid(&springs);

        if !is_valid {
            return 0;
        }

        let next_unknown_idx = springs
            .iter()
            .enumerate()
            .find(|&(_, spring)| matches!(spring, Spring::Unknown));

        match next_unknown_idx {
            None => 1,
            Some((idx, _)) => {
                let mut first = springs.clone();
                let mut second = springs;
                first[idx] = Spring::Operational;
                second[idx] = Spring::Damaged;

                self.count_helper(first) + self.count_helper(second)
            }
        }
    }

    fn count(&self) -> usize {
        self.count_helper(self.springs.clone())
    }
}

fn main() {
    let input = aoc::read_input_arg(2023, 12).expect("could not read input");

    let lines = input
        .lines()
        .map(|l| Line::from_str(l).unwrap())
        .collect::<Vec<_>>();

    let counts = lines.iter().map(|l| l.count()).collect::<Vec<_>>();

    println!("{counts:?}");
}

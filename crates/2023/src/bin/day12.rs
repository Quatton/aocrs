use std::{
    fmt::{Display, Write},
    str::FromStr,
};

#[repr(u8)]
#[derive(PartialEq, Eq, Clone, Copy)]
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
    fn check_candidate(&self, candidate: &[Spring]) -> bool {
        let mut consecutives = 0;
        let mut damaged_iter = self.damaged.iter().peekable();

        for spring in candidate.iter() {
            match spring {
                Spring::Unknown => return true,
                Spring::Damaged => {
                    consecutives += 1;

                    match damaged_iter.peek() {
                        None => return false,
                        Some(&&dmg) => {
                            if dmg < consecutives {
                                return false;
                            }
                        }
                    }
                }
                Spring::Operational => {
                    match damaged_iter.peek() {
                        None => {}
                        Some(&&dmg) => {
                            if dmg != consecutives {
                                return false;
                            }
                            damaged_iter.next();
                        }
                    }
                    consecutives = 0
                }
            }
        }

        damaged_iter.peek().is_none()
    }

    fn fill(&self, prev: &[Spring]) -> Option<[Vec<Spring>; 2]> {
        prev.iter().enumerate().find_map(|(idx, s)| match s {
            Spring::Operational | Spring::Damaged => None,
            Spring::Unknown => {
                let mut op = prev.to_vec();
                let mut dmg = prev.to_vec();
                op[idx] = Spring::Operational;
                dmg[idx] = Spring::Damaged;
                Some([op, dmg])
            }
        })
    }

    fn count(&self) -> usize {
        let mut stack = vec![self.springs.clone()];
    }
}

fn main() {
    let input = aoc::read_input_arg(2023, 12).expect("could not read input");

    let lines = input
        .lines()
        .map(|l| Line::from_str(l).unwrap())
        .collect::<Vec<_>>();
}

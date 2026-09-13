use std::ops::ControlFlow;

fn main() {
    println!(
        "{:?}",
        aoc::read_input_arg(2023, 8)
            .expect("could not read input")
            .split_once("\n\n")
            .map(|(moves, maps)| (
                moves.len(),
                moves.chars().map(|c| if c == 'L' { 0usize } else { 1 }),
                maps.lines().map(|l| l
                    .split_once(" = ")
                    .map(|(src, dst)| (
                        src.trim(),
                        dst.strip_circumfix("(", ")")
                            .unwrap()
                            .split_once(", ")
                            .unwrap()
                    ))
                    .map(|(src, dst)| [src, dst.0, dst.1].map(|s| {
                        s.chars()
                            .rev()
                            .fold(0, |acc, c| acc * 26 + (c as usize - 'A' as usize))
                    }))
                    .unwrap())
            ))
            .map(|(len, moves, maps)| (
                len,
                moves,
                maps.fold([None; 17576], |mut acc, [src, l, r]| {
                    acc[src] = Some([l, r]);
                    acc
                })
            ))
            .map(|(len, moves, matrix)| (
                moves
                    .clone()
                    .cycle()
                    .enumerate()
                    .try_fold(0, |state, (offset, input)| {
                        let state = matrix[state].unwrap()[input];
                        if state == 17575 {
                            ControlFlow::Break(offset + 1)
                        } else {
                            ControlFlow::Continue(state)
                        }
                    })
                    .break_ok()
                    .unwrap(),
                matrix[..26 * 26]
                    .iter()
                    .enumerate()
                    .filter_map(|(state, next)| next.map(|_| state))
                    .map(|init_state| {
                        moves
                            .clone()
                            .cycle()
                            .enumerate()
                            .try_fold(
                                (
                                    init_state,
                                    vec![vec![None::<usize>; len]; 17576],
                                    vec![],
                                    0,
                                    0,
                                    0,
                                ),
                                |(state, mut visited, mut seenz, offset, _, cycle),
                                 (total, input)| match visited
                                    [state][offset]
                                {
                                    Some(first) => ControlFlow::Break((
                                        state, visited, seenz, offset, first, cycle,
                                    )),
                                    None => {
                                        if matches!(state, 16900..17576) {
                                            seenz.push(total);
                                        }
                                        visited[state][offset] = Some(cycle);
                                        let next_state = matrix[state].unwrap()[input];
                                        let offset = offset + 1;
                                        ControlFlow::Continue((
                                            next_state,
                                            visited,
                                            seenz,
                                            if offset == len { 0 } else { offset },
                                            0,
                                            cycle + (offset == len) as usize,
                                        ))
                                    }
                                },
                            )
                            .break_ok()
                            .unwrap()
                    })
                    .map(move |(_, _, seenz, _, first, cycle)| {
                        assert!(seenz.len() == 1);
                        assert!(first == 0);
                        assert!(seenz[0] == cycle * len);
                        cycle
                    })
                    .product::<usize>()
                    * len
            ))
            .unwrap()
    )
}

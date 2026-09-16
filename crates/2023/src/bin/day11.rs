use std::collections::BTreeSet;

fn is_star(c: &char) -> bool {
    matches!(c, '#')
}

fn combinations(
    slice: &[(usize, usize)],
) -> impl Iterator<Item = (&(usize, usize), &(usize, usize))> {
    (0..slice.len()).flat_map(move |i| (i + 1..slice.len()).map(move |j| (&slice[i], &slice[j])))
}

fn hamming_distance(a: &(usize, usize), b: &(usize, usize)) -> usize {
    let row_diff = a.0.abs_diff(b.0);
    let col_diff = a.1.abs_diff(b.1);
    row_diff + col_diff
}

fn main() {
    let input = aoc::read_input_arg(2023, 11).expect("could not read input");

    let rows = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let columns = (0..rows[0].len())
        .map(|col_idx| rows.iter().map(|row| row[col_idx]).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let row_indices_with_no_stars = rows
        .iter()
        .enumerate()
        .filter(|(_, row)| !row.iter().any(is_star))
        .map(|(idx, _)| idx)
        .collect::<BTreeSet<_>>();

    let column_indices_with_no_stars = columns
        .iter()
        .enumerate()
        .filter(|(_, column)| !column.iter().any(is_star))
        .map(|(idx, _)| idx)
        .collect::<BTreeSet<_>>();

    let r_set = &row_indices_with_no_stars;
    let c_set = &column_indices_with_no_stars;

    println!("Part 1: {}", solve(&rows, r_set, c_set, 1));
    println!("Part 1.5: {}", solve(&rows, r_set, c_set, 10 - 1));
    println!("Part 1.75: {}", solve(&rows, r_set, c_set, 100 - 1));
    println!("Part 2: {}", solve(&rows, r_set, c_set, 1000000 - 1));
}

fn solve(
    rows: &[Vec<char>],
    riwn: &BTreeSet<usize>,
    ciwn: &BTreeSet<usize>,
    expand_size: usize,
) -> usize {
    let star_positions = rows
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, c)| is_star(c))
                .map(move |(col_idx, _)| {
                    let num_rows_expanded = riwn.range(..row_idx).count();
                    let num_cols_expanded = ciwn.range(..col_idx).count();
                    (
                        row_idx + num_rows_expanded * expand_size,
                        col_idx + num_cols_expanded * expand_size,
                    )
                })
        })
        .collect::<Vec<_>>();

    let star_combinations = combinations(&star_positions);

    star_combinations.map(|(a, b)| hamming_distance(a, b)).sum()
}

static LEFT: u8 = 0b1000;
static UP: u8 = 0b0100;
static DOWN: u8 = 0b0010;
static RIGHT: u8 = 0b0010;

fn process_pipe(cell: &char) -> u8 {
    match cell {
        '7' => LEFT & DOWN,
        'J' => LEFT & UP,
        'L' => UP & RIGHT,
        'F' => DOWN & RIGHT,
        _ => 0,
    }
}

fn main() {
    let input = aoc::read_input_arg(2023, 10).expect("could not read input");

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let u8grid: Vec<Vec<u8>> = grid
        .iter()
        .map(|row| row.iter().map(process_pipe).collect())
        .collect();

    let s_pos = grid
        .iter()
        .enumerate()
        .find_map(|(r, row)| {
            row.iter()
                .enumerate()
                .find_map(|(c, col)| (*col == 'S').then_some((r, c)))
        })
        .unwrap();
}

static LEFT: u8 = 0b1000;
static DOWN: u8 = 0b0100;
static UP: u8 = 0b0010;
static RIGHT: u8 = 0b0001;

fn process_pipe(cell: &char) -> u8 {
    match cell {
        '7' => LEFT | DOWN,
        'J' => LEFT | UP,
        'L' => UP | RIGHT,
        'F' => DOWN | RIGHT,
        '|' => DOWN | UP,
        '-' => LEFT | RIGHT,
        'S' => LEFT | RIGHT | DOWN | UP,
        _ => 0,
    }
}

struct Grid {
    char_grid: Vec<Vec<char>>,
    grid: Vec<Vec<u8>>,
    s_pos: (usize, usize),
    row_size: usize,
    col_size: usize,
}

impl Grid {
    fn from_str(input: &str) -> Self {
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

        let row_size = grid.len();
        let col_size = grid[0].len();

        Self {
            char_grid: grid,
            grid: u8grid,
            s_pos,
            col_size,
            row_size,
        }
    }

    fn find_loop(&self) -> Vec<(usize, usize)> {
        let mut stack = vec![];
        let mut visited: Vec<Vec<bool>> = self
            .grid
            .iter()
            .map(|row| row.iter().map(|_| false).collect())
            .collect();

        stack.push(vec![self.s_pos]);
        visited[self.s_pos.0][self.s_pos.1] = true;

        while let Some(next) = stack.pop() {
            let front = next.last().unwrap();

            for coord in self.neighbours(front) {
                if visited[coord.0][coord.1] {
                    if coord == *next.iter().nth_back(1).unwrap() {
                        continue;
                    }
                    let mut cloned = next.clone();
                    cloned.push(coord);
                    return cloned;
                } else {
                    visited[coord.0][coord.1] = true
                }

                let mut cloned = next.clone();
                cloned.push(coord);

                // println!(
                //     "{:?} {}{:?} : {}{:?} -> {:?}",
                //     next,
                //     self.char_grid[front.0][front.1],
                //     front,
                //     self.char_grid[coord.0][coord.1],
                //     coord,
                //     v,
                // );
                stack.push(cloned)
            }
        }

        panic!("No!")
    }

    fn neighbours(&self, pos: &(usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        let cur = self.grid[pos.0][pos.1];
        [
            if pos.1 == 0 {
                None
            } else {
                // L- <- S
                let r = pos.0;
                let c = pos.1 - 1;
                (cur & LEFT != 0 && self.grid[r][c] & RIGHT != 0).then_some((r, c))
            },
            if pos.0 == self.row_size - 1 {
                None
            } else {
                let r = pos.0 + 1;
                let c = pos.1;
                (cur & DOWN != 0 && self.grid[r][c] & UP != 0).then_some((r, c))
            },
            if pos.0 == 0 {
                None
            } else {
                let r = pos.0 - 1;
                let c = pos.1;
                (cur & UP != 0 && self.grid[r][c] & DOWN != 0).then_some((r, c))
            },
            if pos.1 == self.col_size - 1 {
                None
            } else {
                let r = pos.0;
                let c = pos.1 + 1;
                (cur & RIGHT != 0 && self.grid[r][c] & LEFT != 0).then_some((r, c))
            },
        ]
        .into_iter()
        .flatten()
    }
}

fn main() {
    let input = aoc::read_input_arg(2023, 10).expect("could not read input");

    let grid = Grid::from_str(&input);

    let found_loop = grid.find_loop();

    let found_loop_set = found_loop
        .clone()
        .into_iter()
        .collect::<std::collections::HashSet<_>>();

    let loop_set_ref = &found_loop_set;

    let out: String = grid
        .char_grid
        .iter()
        .enumerate()
        .flat_map(|(row, cols)| {
            cols.iter()
                .copied()
                .chain(std::iter::once('\n'))
                .enumerate()
                // Add "move" right here 👇
                .map(move |(col, cell)| {
                    if col == grid.col_size {
                        '\n'
                    } else if loop_set_ref.contains(&(row, col)) {
                        cell
                    } else {
                        ' '
                    }
                })
        })
        .collect();

    if out.len() < 1000 {
        println!("{out}");
    } else {
        println!("Output too large")
    }

    let far = found_loop.len();

    println!("Part 1: {}", far / 2);

    let mut inside_count = 0;
    let mut inside;
    let mut prev_vert = UP | DOWN;

    for row in 0..grid.row_size {
        inside = false;
        for col in 0..grid.col_size {
            if found_loop_set.contains(&(row, col)) {
                let dir = grid.grid[row][col];
                if dir & prev_vert & (UP | DOWN) != 0 {
                    inside = !inside;
                    prev_vert = dir;
                }
            } else {
                if inside {
                    inside_count += 1;
                }
            }
        }
    }

    println!("Part 2: {inside_count}");
}

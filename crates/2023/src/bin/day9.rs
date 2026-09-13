type Num = i32;

fn main() {
    let input = aoc::read_input_arg(2023, 9).expect("could not read input");

    let lines = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<Num>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut part1 = 0;
    let mut part2 = 0;
    for line in lines {
        let mut layers = vec![line];
        let mut i = 0;
        while layers[i].iter().any(|&x| x != 0) {
            layers.push(layers[i].windows(2).map(|x| x[1] - x[0]).collect());
            i += 1
        }
        let mut next = 0;
        for layer in layers.iter().rev() {
            next += layer.last().unwrap();
        }
        let mut prev = 0;
        for layer in layers.iter().rev() {
            prev = layer.first().unwrap() - prev;
        }
        part1 += next;
        part2 += prev;
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn main() {
    let input = aoc::read_input_arg(2023, 6).expect("could not read input");

    let lines = input.lines().collect::<Vec<_>>();
    let times = split_and_parse(lines[0]);
    let distances = split_and_parse(lines[1]);

    let part2_time = join_and_parse(lines[0]);
    let part2_distance = join_and_parse(lines[1]);

    let prod = times
        .zip(distances)
        .map(|(t, d)| solve_quadratic_for_diff(t, d))
        .product::<u128>();

    println!("Part 1: {prod}");

    let part2_result = solve_quadratic_for_diff(part2_time, part2_distance);
    println!("Part 2: {part2_result}");
}

fn split_and_parse(s: &str) -> impl Iterator<Item = u128> + '_ {
    s.split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<u128>().unwrap())
}

fn join_and_parse(s: &str) -> u128 {
    s.split_once(":")
        .unwrap()
        .1
        .replace(" ", "")
        .parse::<u128>()
        .unwrap()
}

fn solve_quadratic_for_diff(t: u128, d: u128) -> u128 {
    let tf = t as f64;
    let df = d as f64;
    let discriminant = tf * tf - 4.0 * df;
    assert!(discriminant >= 0.0, "discriminant is negative");
    let sqrt_discriminant = discriminant.sqrt();
    let p = (tf + sqrt_discriminant) / 2.0;
    let q = (tf - sqrt_discriminant) / 2.0;
    let pceil = p.ceil() as u128;
    let qfloor = q.floor() as u128;

    pceil - qfloor - 1
}

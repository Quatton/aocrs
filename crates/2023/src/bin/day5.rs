use std::collections::BTreeMap;
// 26714517
#[derive(Debug)]
struct RangeMap {
    // key, (value, count)
    range_map: BTreeMap<u64, (u64, u64)>,
    start: u64,
    end: u64,
}

impl RangeMap {
    // format:
    // ```
    // <value> <start> <count>
    // ```
    fn from_raw(data: &str) -> Self {
        let data = data.split_once(':').unwrap().1.trim();

        let mut range_map = BTreeMap::new();
        let mut s = u64::MAX;
        let mut end = 0;

        for line in data.lines() {
            let (value, rest) = line.split_once(' ').unwrap();
            let (start, count) = rest.split_once(' ').unwrap();

            let value = value.parse::<u64>().unwrap();
            let start = start.parse::<u64>().unwrap();
            s = s.min(start);
            let count = count.parse::<u64>().unwrap();
            end = end.max(start + count - 1);

            range_map.insert(start, (value, count));
        }

        Self {
            range_map,
            start: s,
            end,
        }
    }

    fn get(&self, key: u64) -> u64 {
        if let Some((start, (value, count))) = self.range_map.range(..=key).next_back()
            && key >= *start
            && key < start + count
        {
            value + (key - start)
        } else {
            key
        }
    }

    // trust that key_ranges are sorted and non-overlapping (for efficiency)
    fn get_ranges(&self, key_ranges: &[(u64, u64)]) -> Vec<(u64, u64)> {
        let mut result = vec![];

        for &(key_start, key_end) in key_ranges {
            // ks1    ke1            ks2       ke2
            //            s    e
            // no overlap ks~ke map to itself
            if key_end <= self.start || key_start >= self.end {
                insert_range(&mut result, (key_start, key_end));
                continue;
            }

            let mut ns = key_start;
            let mut ne = key_end;

            // ks
            //       s          e
            // overlap where ks < s and ke whereever it wants
            if key_start < self.start {
                insert_range(&mut result, (key_start, self.start - 1));
                ns = self.start;
            }

            // ns               ke
            //  s       e
            // overlap where ke > e and ks already set to >= s
            if key_end > self.end {
                insert_range(&mut result, (self.end, key_end));
                ne = self.end;
            }

            // assume new_start >= s and new_end <= e
            assert!(ns >= self.start && ne <= self.end);
            //  s  ps      pegs       ge           qs    qe  e
            //       ns                        ne
            // loop through the range_map from pr to qr
            assert!(ns <= ne);
            let pr = self.range_map.range(..=ns).next_back().unwrap();
            let qr = self
                .range_map
                .range(ne..)
                .next()
                .unwrap_or_else(|| self.range_map.range(..).next_back().unwrap());

            let new_key_ranges = self.range_map.range(pr.0..=qr.0);

            for (&gs, &(gv, gc)) in new_key_ranges {
                let ge = gs + gc - 1;
                // if it ends before ns, skip it
                if ge < ns {
                    continue;
                }
                // if it starts after ne, break
                if gs > ne {
                    break;
                }

                // if it starts after ns, then we can see the gap, add identity range
                if gs > ns {
                    insert_range(&mut result, (ns, gs - 1));
                }

                let new_start = ns.max(gs);
                let new_end = ne.min(ge);

                let vs = gv + (new_start - gs);
                let ve = gv + (new_end - gs);
                assert!(vs <= ve);
                insert_range(&mut result, (vs, ve));

                ns = new_end + 1;
            }

            // add the last identity range if needed
            if ns <= ne {
                insert_range(&mut result, (ns, ne));
            }
        }

        assert!(!result.is_empty());

        result
    }
}

fn insert_range(ranges: &mut Vec<(u64, u64)>, new_range: (u64, u64)) {
    assert!(new_range.0 <= new_range.1);
    let mut i = 0;
    while i < ranges.len() {
        let (start, end) = ranges[i];
        if new_range.1 < start {
            break;
        } else if new_range.0 > end {
            i += 1;
            continue;
        } else {
            let new_start = new_range.0.min(start);
            let new_end = new_range.1.max(end);
            ranges[i] = (new_start, new_end);
            return;
        }
    }
    ranges.insert(i, new_range);
}

fn main() {
    let input = aoc::read_input_arg(2023, 5).expect("could not read input");

    let (seeds, range_maps) = parse(&input);

    // confirm no maps have gaps -> result: negative
    // for range_map in range_maps.iter() {
    //     let mut last_end = range_map.start;
    //     for (start, (_, count)) in range_map.range_map.iter() {
    //         assert_eq!(*start, last_end);
    //         last_end = start + count;
    //     }
    //     assert_eq!(last_end, range_map.end);
    // }

    let mut part1 = u64::MAX;
    let mut part2 = u64::MAX;

    for seed in seeds.iter() {
        let mut value = *seed;

        for range_map in range_maps.iter() {
            value = range_map.get(value);
        }

        part1 = part1.min(value);
    }

    for (idx, seed_pair) in seeds.chunks(2).enumerate() {
        let mut key_ranges = vec![(seed_pair[0], seed_pair[0] + seed_pair[1] - 1)];

        for (rdx, range_map) in range_maps.iter().enumerate() {
            println!("Seed pair {idx} at key index {rdx}");
            key_ranges = range_map.get_ranges(&key_ranges);
        }

        let &(first_min, _) = key_ranges.first().unwrap();
        part2 = part2.min(first_min);
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn parse(input: &str) -> (Vec<u64>, Vec<RangeMap>) {
    let mut sections = input.split("\n\n");

    let seeds = sections
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let range_maps = sections.map(RangeMap::from_raw).collect();

    (seeds, range_maps)
}

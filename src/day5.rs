use std::ops::RangeInclusive;

use crate::utils::get_input;

#[allow(dead_code)]
const TEST_DATA: &str = r#"
3-5
10-14
16-20
12-18

1
5
8
11
17
32
"#;

fn get_data() -> (Vec<RangeInclusive<i64>>, Vec<i64>) {
    let data_str = get_input(5).unwrap();
    // let data_str = TEST_DATA;
    let (part1, part2) = data_str.trim().split_once("\n\n").unwrap();

    let ranges = part1
        .split("\n")
        .map(|s| {
            let (a, b) = s.split_once('-').unwrap();
            a.parse().unwrap()..=b.parse().unwrap()
        })
        .collect::<Vec<_>>();
    let values = part2
        .split("\n")
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();

    (ranges, values)
}

pub fn part_1() {
    let (ranges, values) = get_data();

    let count = values
        .iter()
        .filter(|x| ranges.iter().any(|r| r.contains(x)))
        .count();

    assert_eq!(count, 737);
    println!("Part 1: {count:?}");
}

pub fn part_2() {
    let (mut ranges, _) = get_data();

    ranges.sort_by(|a, b| a.start().cmp(b.start()));

    let mut prev_end = 0;

    let sum: i64 = ranges
        .iter()
        .map(|r| {
            if r.end() <= &prev_end {
                return 0;
            }
            let start = *r.start().max(&(prev_end + 1));
            prev_end = *r.end();
            r.end() - start + 1
        })
        .sum();

    assert_eq!(sum, 357485433193284);
    println!("Part 2: {sum}");
}

use std::ops::RangeInclusive;

use crate::utils::get_input;

#[allow(dead_code)]
const TEST_DATA: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

fn parse_range(s: &str) -> RangeInclusive<i64> {
    let (a, b) = s.split_once("-").unwrap();
    let start = a.parse().unwrap();
    let end = b.parse().unwrap();
    start..=end
}

fn get_data() -> Vec<RangeInclusive<i64>> {
    // let data = TEST_DATA;
    let data = get_input(2).unwrap();
    data.trim().split(",").map(parse_range).collect()
}

pub fn part_1() {
    let mut invalid_ids: Vec<i64> = Vec::new();
    let data = get_data();

    for range in data {
        for id in range {
            let id_str = id.to_string();
            let (a, b) = id_str.split_at(id_str.len() / 2);
            if a == b {
                invalid_ids.push(id);
            }
        }
    }

    let sum = invalid_ids.iter().sum::<i64>();
    assert_eq!(sum, 19386344315);
    println!("Part 1: {sum}");
}

pub fn part_2() {
    let mut invalid_ids: Vec<i64> = Vec::new();
    let data = get_data();

    for range in data {
        for id in range {
            let id_str = id.to_string();
            for i in 1..=id_str.len() / 2 {
                let mut chunks = id_str.as_bytes().chunks(i);
                let first_chunk = chunks.next().unwrap();
                if chunks.all(|c| c == first_chunk) {
                    invalid_ids.push(id);
                }
            }
        }
    }
    invalid_ids.sort();
    invalid_ids.dedup();

    let sum = invalid_ids.iter().sum::<i64>();
    assert_eq!(sum, 34421651192);
    println!("Part 2: {sum}");
}

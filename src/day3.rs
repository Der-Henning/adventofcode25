use crate::utils::get_input;

#[allow(dead_code)]
const TEST_DATA: &str = "987654321111111\n811111111111119\n234234234234278\n818181911112111";

fn get_data() -> Vec<String> {
    let data_str = get_input(3).unwrap();
    // let data_str = TEST_DATA;
    data_str.trim().split("\n").map(|s| s.to_string()).collect()
}

pub fn part_1() {
    let data = get_data();

    let result = data
        .into_iter()
        .map(|b| {
            b.chars()
                .take(b.len() - 1)
                .enumerate()
                .map(|(i, x)| {
                    let y = b.chars().skip(i + 1).max().unwrap();
                    format!("{x}{y}").parse::<i32>().unwrap()
                })
                .max()
                .unwrap()
        })
        .sum::<i32>();

    assert_eq!(result, 17144);
    println!("Part 1: {result:?}");
}

pub fn part_2() {
    let data = get_data();

    let sum: i64 = data
        .into_iter()
        .map(|b| {
            let mut position = 0;
            let mut yoltage = String::new();

            // Search for best match from left to right
            // Use a sliding window starting after the last selected element
            // The window size is selected so that there are enough digits left
            // when the rightmost digit would be selected
            // Take the largest element of the window as next digit for the yoltage
            for i in 0..12 {
                // select slice in which to search for the next element
                let s = &b[position..=b.len() - 12 + i];

                // find first max value and its index in the slice
                let (max_idx, max_val) = s
                    .chars()
                    .enumerate()
                    .fold((0, '0'), |a, b| if b.1 > a.1 { b } else { a });

                // push new element to yoltage
                yoltage.push(max_val);

                // move search window
                position += max_idx + 1;
            }
            // parse yoltage as integer
            yoltage.parse::<i64>().unwrap()
        })
        // Sum up all yoltages
        .sum();

    assert_eq!(sum, 170371185255900);
    println!("Part 2: {sum}");
}

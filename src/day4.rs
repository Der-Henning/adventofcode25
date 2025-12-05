use crate::utils::get_input;

#[allow(dead_code)]
const TEST_DATA: &str = r#"
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"#;

fn get_data() -> Vec<String> {
    let data_str = get_input(4).unwrap();
    // let data_str = TEST_DATA;
    data_str.trim().split("\n").map(|s| s.to_string()).collect()
}

fn get_pos(data: &[String], x: usize, y: usize) -> Option<char> {
    data.get(y)?.chars().nth(x)
}

pub fn part_1() {
    let data = get_data();

    let mut count = 0;
    let size = (data[0].len(), data.len());
    let pos_iter = (0..size.0).flat_map(|x| (0..size.1).map(move |y| (x, y)));

    for (x, y) in pos_iter {
        if get_pos(&data, x, y).unwrap() == '@' {
            let mut rolls = 0;
            for i in x.saturating_sub(1)..=(x + 1).min(size.0 - 1) {
                for j in y.saturating_sub(1)..=(y + 1).min(size.1 - 1) {
                    if !(x == i && y == j) && get_pos(&data, i, j).unwrap() == '@' {
                        rolls += 1;
                    }
                }
            }
            if rolls < 4 {
                count += 1;
            }
        }
    }

    assert_eq!(count, 1495);
    println!("Part 1: {count:?}");
}

pub fn part_2() {
    let mut data = get_data();

    let mut abs_count = 0;
    let size = (data[0].len(), data.len());

    loop {
        let mut count = 0;
        let mut new_data = Vec::new();
        for y in 0..size.1 {
            let mut new_row = String::new();
            for x in 0..size.0 {
                let c = get_pos(&data, x, y).unwrap();
                if c == '@' {
                    let mut rolls = 0;
                    for i in x.saturating_sub(1)..=(x + 1).min(size.0 - 1) {
                        for j in y.saturating_sub(1)..=(y + 1).min(size.1 - 1) {
                            if !(x == i && y == j) && get_pos(&data, i, j).unwrap() == '@' {
                                rolls += 1;
                            }
                        }
                    }
                    if rolls < 4 {
                        count += 1;
                        new_row.push('.');
                    } else {
                        new_row.push(c);
                    }
                } else {
                    new_row.push('.');
                }
            }
            new_data.push(new_row);
        }
        if count == 0 {
            break;
        }
        abs_count += count;
        data = new_data;
    }

    assert_eq!(abs_count, 8768);
    println!("Part 2: {abs_count}");
}

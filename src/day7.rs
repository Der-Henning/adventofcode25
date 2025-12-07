use crate::utils::get_input;

#[allow(dead_code)]
const TEST_DATA: &str = r#"
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"#;

fn get_data() -> Vec<String> {
    let data_str = get_input(7).unwrap();
    // let data_str = TEST_DATA;

    data_str.trim().split("\n").map(str::to_string).collect()
}

pub fn part_1() {
    let mut data = get_data();

    let rows = data.len();
    let cols = data[0].len();

    let mut count = 0;

    // Draw rays into data by replacing the dots in place
    for i in 1..rows {
        for j in 0..cols {
            let above = data[i - 1].chars().nth(j).unwrap();
            let current = data[i].chars().nth(j).unwrap();
            match (current, above) {
                ('.', 'S') | ('.', '|') => data[i].replace_range(j..j + 1, "|"),
                ('^', '|') => {
                    count += 1;
                    data[i].replace_range(j - 1..j + 2, "|^|");
                }
                _ => (),
            };
        }
    }

    // for row in data {
    //     println!("{row}");
    // }

    assert_eq!(count, 1600);
    println!("Part 1: {count:?}");
}

pub fn part_2() {
    let mut data = get_data();

    let rows = data.len();
    let cols = data[0].len();

    // Again draw the tree like in part 1
    for i in 1..rows {
        for j in 0..cols {
            let above = data[i - 1].chars().nth(j).unwrap();
            let current = data[i].chars().nth(j).unwrap();
            match (current, above) {
                ('.', 'S') | ('.', '|') => data[i].replace_range(j..j + 1, "|"),
                ('^', '|') => data[i].replace_range(j - 1..j + 2, "|^|"),
                _ => (),
            };
        }
    }

    // for row in &data {
    //     println!("{row}");
    // }

    // for every cell, calculate the number of options to reach the cell with the beam
    // This is zero by default for every cell at start
    // the sum of the last row is then the number of possible timelines
    let mut tree = Vec::new();
    let mut first_row = vec![0; cols];
    first_row[data[0].find('S').unwrap()] = 1;
    tree.push(first_row);

    // iterate over the drawn tree and calculate for every beam the number of options
    // to reach it
    for (j, row) in data.iter().skip(1).enumerate() {
        let mut tree_row = vec![0; cols];
        for (i, char) in row.chars().enumerate() {
            // the number of options always propagate downwards
            // Only under the splitter the possibility is zero
            if char != '^' {
                let timelines = tree[j].get(i).unwrap();
                tree_row[i] += timelines;
            }
            // if next to the beam is a splitter, these timeline could create the beam
            if char == '|' {
                // Splitter on the left
                if let Some(left_char) = row.chars().nth(i - 1)
                    && left_char == '^'
                {
                    let timelines = tree[j].get(i - 1).unwrap();
                    tree_row[i] += timelines;
                }
                // Splitter on the right
                if let Some(right_char) = row.chars().nth(i + 1)
                    && right_char == '^'
                {
                    let timelines = tree[j].get(i + 1).unwrap();
                    tree_row[i] += timelines;
                }
            }
        }
        tree.push(tree_row);
    }

    // for row in &tree {
    //     println!("{row:?}");
    // }

    let tree_leaves = tree.last().unwrap();
    // println!("{:?}", tree_leaves);

    let count: i64 = tree_leaves.iter().sum();

    assert_eq!(count, 8632253783011);
    println!("Part 2: {count}");
}

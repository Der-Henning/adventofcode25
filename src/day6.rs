use crate::utils::get_input;

#[allow(dead_code)]
const TEST_DATA: &str = r#"
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
"#;

fn get_data() -> String {
    let data_str = get_input(6).unwrap();
    // let data_str = TEST_DATA;

    data_str.to_string()
}

pub fn part_1() {
    let data_str = get_data();
    let data = data_str
        .trim()
        .split("\n")
        .map(|r| r.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let cols = data[0].len();
    let rows = data.len();

    let mut nums: Vec<i64> = Vec::with_capacity(rows - 1);

    let sum = (0..cols)
        .map(|i| {
            nums.clear();
            for d in data.iter().take(rows - 1) {
                nums.push(d[i].parse().unwrap());
            }
            match data[rows - 1][i] {
                "+" => nums.iter().sum::<i64>(),
                "*" => nums.iter().product::<i64>(),
                _ => panic!("Unsupported operation"),
            }
        })
        .sum::<i64>();

    assert_eq!(sum, 4771265398012);
    println!("Part 1: {sum:?}");
}

pub fn part_2() {
    let data_str = get_data();
    let mut data = data_str.trim().split("\n").collect::<Vec<_>>();
    let ops = data.pop().unwrap();

    let mut calculations = Vec::new();
    let mut operations = Vec::new();

    let mut nums: Vec<i64> = Vec::new();
    for i in 0..data[0].len() {
        match ops.chars().nth(i) {
            op @ (Some('+') | Some('*')) => {
                if !nums.is_empty() {
                    calculations.push(nums.clone());
                }
                nums.clear();
                operations.push(op.unwrap());
            }
            Some(' ') | None => (),
            Some(_) => panic!("Unsupported operation"),
        }

        let num_str = data
            .iter()
            .map(|r| r.chars().nth(i).unwrap())
            .filter(|c| c.is_numeric())
            .collect::<String>();
        if !num_str.is_empty() {
            nums.push(num_str.parse().unwrap());
        }
    }
    calculations.push(nums.clone());

    let sum = calculations
        .iter()
        .zip(operations.iter())
        .map(|(calc, op)| match op {
            '+' => calc.iter().sum::<i64>(),
            '*' => calc.iter().product::<i64>(),
            _ => panic!("Unsupported operation"),
        })
        .sum::<i64>();

    assert_eq!(sum, 10695785245101);
    println!("Part 2: {sum}");
}

use good_lp::*;
use regex::Regex;

use crate::utils::get_input;

#[allow(dead_code)]
const TEST_DATA: &str = r#"
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"#;

fn press(lights: &[bool], button: &Vec<bool>) -> Vec<bool> {
    lights.iter().zip(button).map(|(a, b)| *a ^ *b).collect()
}

type Data = (Vec<bool>, Vec<Vec<bool>>, Vec<u32>);

fn get_data() -> Vec<Data> {
    let data_str = get_input(10).unwrap();
    // let data_str = TEST_DATA;

    let machine_re = Regex::new(r"\[(.*)\] (.*) \{(.*)\}").unwrap();
    let button_re = Regex::new(r"\(([\d,]+)\)").unwrap();

    machine_re
        .captures_iter(&data_str)
        .map(|c| {
            let s: [&str; 3] = c.extract().1;
            let target: Vec<bool> = s[0].chars().map(|s| s == '#').collect();
            let buttons_len = target.len();
            let buttons = button_re
                .captures_iter(s[1])
                .map(|c| {
                    let s: [&str; 1] = c.extract().1;
                    let mut b = vec![false; buttons_len];
                    s[0].split(",")
                        .map(|i| i.parse::<usize>().unwrap())
                        .for_each(|i| b[i] = true);
                    b
                })
                .collect::<Vec<_>>();
            let joltages = s[2].split(",").map(|i| i.parse().unwrap()).collect();
            (target, buttons, joltages)
        })
        .collect::<Vec<_>>()
}

pub fn part_1() {
    let data = get_data();

    let mut levels = Vec::new();
    for (target, buttons, _) in data.iter() {
        let mut lights = vec![vec![false; target.len()]];
        let mut level = 0;
        // Brute force iterate over all combinations increasing in number
        // returning level when target is reached
        'outer: loop {
            let mut new_lights = Vec::new();
            level += 1;
            for l in lights.iter() {
                for b in buttons {
                    let light = press(l, b);
                    if light == *target {
                        // target reached, break out of loop
                        break 'outer;
                    }
                    new_lights.push(light);
                }
            }
            lights = new_lights;
        }
        levels.push(level);
    }

    let sum: i32 = levels.iter().sum();

    assert_eq!(sum, 441);
    println!("Part 1: {sum}");
}

pub fn part_2() {
    let data = get_data();

    let mut clicks = Vec::new();

    for (_, buttons, joltages) in data.iter() {
        // Solve linear system using the good_lp crate
        // This crate supports integer problems

        // The problem variables are the number of presses for each button
        // Each variable can be zero or a positive integer
        let mut vars = variables!();
        let variables = (0..buttons.len())
            .map(|_| vars.add(variable().integer().min(0)))
            .collect::<Vec<_>>();

        // The expression to minimize is the sum of the variables
        let objective = variables.iter().sum::<Expression>();
        let mut problem = vars.minimise(&objective).using(default_solver);

        // Each joltage value is a constraint to the problem
        // The equation is given by the button properties
        for (j, t) in joltages.iter().enumerate() {
            let expr = variables
                .iter()
                .enumerate()
                .filter_map(|(i, x)| if buttons[i][j] { Some(x) } else { None })
                .sum::<Expression>();
            problem = problem.with(expr.eq(*t));
        }

        // Solve the lp
        let solution = problem.solve().unwrap();
        clicks.push(solution.eval(objective));
    }

    let sum = clicks.iter().sum::<f64>() as i32;

    assert_eq!(sum, 18559);
    println!("Part 2: {sum}");
}

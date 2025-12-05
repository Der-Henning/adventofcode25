use crate::utils::get_input;

#[allow(dead_code)]
const TEST_DATA: &str = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";

fn get_data() -> Vec<String> {
    let data_str = get_input(1).unwrap();
    data_str.trim().split("\n").map(|s| s.to_string()).collect()
}

fn parse_dir(s: &str) -> i32 {
    let (a, b) = s.split_at(1);
    let x: i32 = b.parse().unwrap();
    if a == "R" { x } else { -x }
}

pub fn part_1() {
    let data = get_data();

    let mut position = 50;
    let mut counter = 0;

    for d in data {
        let click = parse_dir(&d);
        position += click;
        if position % 100 == 0 {
            counter += 1;
        }
    }

    assert_eq!(counter, 1165);
    println!("Part 1: {counter}")
}

struct Tresor {
    position: i32,
}

impl Tresor {
    fn new(start_position: i32) -> Self {
        Self {
            position: start_position,
        }
    }

    fn rotate(&mut self, r: &str) -> i32 {
        let mut count = 0;
        let click = parse_dir(r);
        let a = click.abs();
        let x = click.signum();
        for _ in 0..a {
            self.position += x;
            match self.position {
                ..0 => self.position += 100,
                100.. => self.position -= 100,
                _ => (),
            };
            if self.position == 0 {
                count += 1;
            }
        }
        // println!("New position {} -> {}", r, self.position);
        count
    }
}

pub fn part_2() {
    let data = get_data();

    let mut tresor = Tresor::new(50);

    let count: i32 = data.iter().map(|x| tresor.rotate(x)).sum();
    assert_eq!(count, 6496);
    println!("Part 2: {count}");
}

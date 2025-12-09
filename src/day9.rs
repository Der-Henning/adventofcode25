use itertools::Itertools;
use std::hash::Hash;

use crate::utils::get_input;

#[allow(dead_code)]
const TEST_DATA: &str = r#"
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"#;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let mut s = value.trim().split(',').map(|s| s.parse().unwrap());
        Self {
            x: s.next().unwrap(),
            y: s.next().unwrap(),
        }
    }
}

impl Point {
    fn area(&self, other: &Self) -> i64 {
        ((self.x - other.x).abs() + 1) * ((self.y - other.y).abs() + 1)
    }
}

fn get_data() -> Vec<Point> {
    let data_str = get_input(9).unwrap();
    // let data_str = TEST_DATA;

    data_str.trim().split("\n").map(Point::from).collect()
}

pub fn part_1() {
    let data = get_data();

    let area = data
        .iter()
        .tuple_combinations()
        .map(|(a, b)| a.area(b))
        .max()
        .unwrap();

    assert_eq!(area, 4790063600);
    println!("Part 1: {area}");
}

#[derive(Debug, Clone)]
struct Line {
    a: Point,
    b: Point,
}

impl Line {
    fn new(a: Point, b: Point) -> Self {
        Self { a, b }
    }
}

pub fn part_2() {
    let data = get_data();

    // Create lines
    let mut lines: Vec<Line> = data
        .windows(2)
        .map(|w| Line::new(w[0], w[1]))
        .collect();
    lines.push(Line::new(*data.last().unwrap(), *data.first().unwrap()));

    // Check for every rectangle, whether any line intersects with it
    // Calculate the area as above and select the largest
    let area = data
        .iter()
        .tuple_combinations()
        .filter(|(a, b)| {
            let (min_x, max_x) = (a.x.min(b.x), a.x.max(b.x));
            let (min_y, max_y) = (a.y.min(b.y), a.y.max(b.y));
            !lines.iter().any(|l| {
                let (min_lx, max_lx) = (l.a.x.min(l.b.x), l.a.x.max(l.b.x));
                let (min_ly, max_ly) = (l.a.y.min(l.b.y), l.a.y.max(l.b.y));
                // check for intersection
                min_lx < max_x && max_lx > min_x && min_ly < max_y && max_ly > min_y
            })
        })
        .map(|(a, b)| a.area(b))
        .max()
        .unwrap();

    assert_eq!(area, 1516172795);
    println!("Part 2: {area}");
}

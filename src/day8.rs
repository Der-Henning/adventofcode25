use itertools::Itertools;
use std::{collections::HashMap, hash::Hash};

use crate::utils::get_input;

#[allow(dead_code)]
const TEST_DATA: &str = r#"
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"#;

fn get_data() -> Vec<Point> {
    let data_str = get_input(8).unwrap();
    // let data_str = TEST_DATA;

    data_str.trim().split("\n").map(Point::from).collect()
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let mut s = value.split(',').map(|x| x.parse::<i64>().unwrap());
        Self {
            x: s.next().unwrap(),
            y: s.next().unwrap(),
            z: s.next().unwrap(),
        }
    }
}

impl Point {
    fn dist(&self, other: &Self) -> i64 {
        (other.x - self.x).pow(2) + (other.y - self.y).pow(2) + (other.z - self.z).pow(2)
    }
}

pub fn part_1() {
    let data = get_data();

    let mut circuit_id: u32 = 0;
    let mut circuits: HashMap<u32, Vec<Point>> = HashMap::new();
    let mut points: HashMap<Point, u32> = HashMap::new();

    // generate all possible combinations
    // calculate and sort by each distance
    // take the first 1000 closest points
    // build the circuits
    data.iter()
        .tuple_combinations()
        .map(|(a, b)| (a, b, a.dist(b)))
        .sorted_unstable_by(|a, b| a.2.cmp(&b.2))
        .take(1000)
        .for_each(
            |(p1, p2, _)| match (points.get(p1).cloned(), points.get(p2).cloned()) {
                // None of the two points is in a circuit
                // create a new circuit
                (None, None) => {
                    circuits.insert(circuit_id, vec![p1.clone(), p2.clone()]);
                    points.insert(p1.clone(), circuit_id);
                    points.insert(p2.clone(), circuit_id);
                    circuit_id += 1;
                }
                // the right point is in a circuit, add the left
                (None, Some(i)) => {
                    circuits.entry(i).and_modify(|v| v.push(p1.clone()));
                    points.insert(p1.clone(), i);
                }
                // the left point is in a circuit, add the right
                (Some(i), None) => {
                    circuits.entry(i).and_modify(|v| v.push(p2.clone()));
                    points.insert(p2.clone(), i);
                }
                // both points are in a circuit
                // when in different circuits join them
                (Some(i), Some(j)) => {
                    if i != j {
                        let o = circuits.remove(&j).unwrap();
                        points.extend(o.iter().map(|p| (p.clone(), i)));
                        circuits.entry(i).and_modify(|v| v.extend(o));
                    }
                }
            },
        );

    // take the three longest circuits
    // and calculate the product of their lengths
    let prod: usize = circuits
        .values()
        .map(|c| c.len())
        .sorted_unstable()
        .rev()
        .take(3)
        .product();

    assert_eq!(prod, 97384);
    println!("Part 1: {prod:?}");
}

pub fn part_2() {
    let data = get_data();

    // Same as in part 1
    // But dont stop at 1000
    // break when all points are in one circuit
    let iter = data
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (a, b, a.dist(b)))
        .sorted_unstable_by(|a, b| a.2.cmp(&b.2));

    let mut circuit_id: u32 = 0;
    let mut circuits: HashMap<u32, Vec<Point>> = HashMap::new();
    let mut points: HashMap<Point, u32> = HashMap::new();
    let mut prod = 0;

    for (p1, p2, _) in iter {
        match (points.get(p1).cloned(), points.get(p2).cloned()) {
            (None, None) => {
                circuits.insert(circuit_id, vec![p1.clone(), p2.clone()]);
                points.insert(p1.clone(), circuit_id);
                points.insert(p2.clone(), circuit_id);
                circuit_id += 1;
            }
            (None, Some(i)) => {
                circuits.entry(i).and_modify(|v| v.push(p1.clone()));
                points.insert(p1.clone(), i);
            }
            (Some(i), None) => {
                circuits.entry(i).and_modify(|v| v.push(p2.clone()));
                points.insert(p2.clone(), i);
            }
            (Some(i), Some(j)) => {
                if i != j {
                    let o = circuits.remove(&j).unwrap();
                    points.extend(o.iter().map(|p| (p.clone(), i)));
                    circuits.entry(i).and_modify(|v| v.extend(o));
                }
            }
        }
        // Check if done
        // calculate final product
        if points.len() == data.len() && circuits.len() == 1 {
            // println!("done {:?} {:?}", p1, p2);
            prod = p1.x * p2.x;
            break;
        }
    }

    assert_eq!(prod, 9003685096);
    println!("Part 2: {prod}");
}

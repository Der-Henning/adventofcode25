use std::{
    collections::HashSet,
    fmt::{Display, Write},
};

use itertools::Itertools;

use crate::utils::get_input;

#[allow(dead_code)]
const TEST_DATA: &str = r#"
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
"#;

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct Present([[bool; 3]; 3]);

impl Present {
    // rotate present clockwise
    fn rotate(&self) -> Self {
        Self(
            (0..3)
                .map(|i| {
                    (0..3)
                        .map(|j| self.0[2 - j][i])
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap()
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        )
    }

    // get the number of cells in the region
    fn cell_count(&self) -> u32 {
        (0..3)
            .cartesian_product(0..3)
            .map(|(x, y)| self.0[y][x] as u32)
            .sum()
    }

    // Calculate all present rotations without symmetric duplicates
    fn rotations(&self) -> Vec<Self> {
        let mut rotated_present = vec![*self];
        for _ in 0..3 {
            rotated_present.push(rotated_present.last().unwrap().rotate())
        }
        rotated_present.sort_unstable();
        rotated_present.dedup();
        rotated_present
    }
}

// Implement Display for debugging
impl Display for Present {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0 {
            for col in row {
                if col {
                    f.write_char('#')?;
                } else {
                    f.write_char('.')?;
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

// Create Present from str input
impl From<&str> for Present {
    fn from(value: &str) -> Self {
        Self(
            value
                .trim()
                .split('\n')
                .skip(1)
                .map(|r| {
                    r.chars()
                        .map(|c| c == '#')
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap()
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        )
    }
}

#[derive(Debug, Clone)]
struct Region(Vec<Vec<bool>>);

// Create Region from str input
impl From<&str> for Region {
    fn from(value: &str) -> Self {
        let (x, y) = value.split_once('x').unwrap();
        Self(vec![vec![false; x.parse().unwrap()]; y.parse().unwrap()])
    }
}

// Implement display for debugging
impl Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.0.len() {
            for x in 0..self.0[0].len() {
                if self.0[y][x] {
                    f.write_char('#')?;
                } else {
                    f.write_char('.')?;
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Region {
    // try to place a present a position (x, y)
    // Return updated region when successful else None
    fn place(&self, present: &Present, x: usize, y: usize) -> Option<Self> {
        let mut updated_region = self.clone();
        for i in 0..present.0[0].len() {
            for j in 0..present.0.len() {
                if present.0[i][j] {
                    if updated_region.0[y + i][x + j] {
                        return None;
                    }
                    updated_region.0[y + i][x + j] = true;
                }
            }
        }
        Some(updated_region)
    }
}

type Data = (Vec<Vec<Present>>, Vec<(Region, Vec<u32>)>);

fn get_data() -> Data {
    let data_str = get_input(12).unwrap();
    // let data_str = TEST_DATA.to_string();

    let parts = data_str.split("\n\n");

    // parse presents
    let presents = parts
        .clone()
        .take(6)
        .map(Present::from)
        .map(|t| t.rotations())
        .collect::<Vec<_>>();

    // parse regions
    let regions = parts
        .last()
        .unwrap()
        .trim()
        .split("\n")
        .map(|s| {
            let (a, s) = s.split_once(": ").unwrap();
            let region = Region::from(a);
            let shapes = s.split_whitespace().map(|i| i.parse().unwrap()).collect();
            (region, shapes)
        })
        .collect::<Vec<_>>();

    (presents, regions)
}

// Recursively place presents in region until all presents are positioned
// Iterate over open slots in the region and try to place any present
// When all presents are placed return true
// Only search for a single solution
fn place_presents(
    region: Region,
    present_counts: Vec<u32>,
    presents: &Vec<Vec<Present>>,
    mut blocked_slots: HashSet<(usize, usize)>,
) -> Option<bool> {
    // Iterate over region slots
    (0..region.0[0].len() - 2)
        .cartesian_product(0..region.0.len() - 2)
        .filter_map(|(x, y)| {
            // Skip Slot, when already occupied or in set of blocked slots
            if region.0[y][x] || blocked_slots.contains(&(x, y)) {
                return None;
            }
            // Iterate over available presents
            match present_counts
                .iter()
                .enumerate()
                .filter_map(|(i, c)| {
                    if *c == 0 {
                        return None;
                    }
                    // Iterate over present rotations
                    presents[i]
                        .iter()
                        .filter_map(|present| {
                            let mut present_counts = present_counts.clone();
                            // Check if present can be placed
                            // Update present counts and increase recursion level
                            if let Some(new_region) = region.place(present, x, y) {
                                present_counts[i] -= 1;
                                if present_counts.iter().sum::<u32>() == 0 {
                                    // recursion base case: All presents are placed
                                    return Some(true);
                                }
                                return place_presents(
                                    new_region,
                                    present_counts,
                                    presents,
                                    blocked_slots.clone(),
                                );
                            }
                            None
                        })
                        .next()
                })
                .next()
            {
                Some(b) => Some(b),
                None => {
                    // If no present can be placed in this slot,
                    // cache this information for this branch
                    blocked_slots.insert((x, y));
                    None
                }
            }
        })
        .next()
}

pub fn part_1() {
    let (presents, regions) = get_data();

    let mut counter = 0;
    for (region, present_counts) in regions.iter() {
        // Early exit, when the number of presents exceeds the available area of the region
        // Actually this completely solves the puzzle without all the recursion ...
        if present_counts
            .iter()
            .enumerate()
            .map(|(i, present_count)| present_count * presents[i][0].cell_count())
            .sum::<u32>()
            > (region.0.len() * region.0[0].len()) as u32
        {
            continue;
        }

        if place_presents(
            region.clone(),
            present_counts.clone(),
            &presents,
            HashSet::new(),
        )
        .is_some()
        {
            // When found one solution increase counter
            counter += 1;
        }
    }

    assert_eq!(counter, 548);
    println!("Part 1: {counter}");
}

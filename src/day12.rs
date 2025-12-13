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
struct Tile([[bool; 3]; 3]);

impl Tile {
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

    fn cell_count(&self) -> u32 {
        (0..3)
            .cartesian_product(0..3)
            .map(|(x, y)| self.0[y][x] as u32)
            .sum()
    }

    fn rotations(&self) -> Vec<Self> {
        let mut new_tiles = vec![*self];
        for _ in 0..3 {
            new_tiles.push(new_tiles.last().unwrap().rotate())
        }
        new_tiles.sort_unstable();
        new_tiles.dedup();
        new_tiles
    }
}

impl Display for Tile {
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

impl From<&str> for Tile {
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
struct Area(Vec<Vec<bool>>);

impl From<&str> for Area {
    fn from(value: &str) -> Self {
        let (x, y) = value.split_once('x').unwrap();
        Self(vec![vec![false; x.parse().unwrap()]; y.parse().unwrap()])
    }
}

impl Display for Area {
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

impl Area {
    fn place(&self, tile: &Tile, x: usize, y: usize) -> Option<Self> {
        let mut new_area = self.clone();
        for i in 0..tile.0[0].len() {
            for j in 0..tile.0.len() {
                if tile.0[i][j] {
                    if new_area.0[y + i][x + j] {
                        return None;
                    }
                    new_area.0[y + i][x + j] = true;
                }
            }
        }
        Some(new_area)
    }
}

type Data = (Vec<Vec<Tile>>, Vec<(Area, Vec<u32>)>);

fn get_data() -> Data {
    let data_str = get_input(12).unwrap();
    // let data_str = TEST_DATA.to_string();

    let parts = data_str.split("\n\n");

    let tiles = parts
        .clone()
        .take(6)
        .map(Tile::from)
        .map(|t| t.rotations())
        .collect::<Vec<_>>();

    let areas = parts
        .last()
        .unwrap()
        .trim()
        .split("\n")
        .map(|s| {
            let (a, s) = s.split_once(": ").unwrap();
            let area = Area::from(a);
            let shapes = s.split_whitespace().map(|i| i.parse().unwrap()).collect();
            (area, shapes)
        })
        .collect::<Vec<_>>();

    (tiles, areas)
}

fn fill_area(
    area: Area,
    tile_counts: Vec<u32>,
    tiles: &Vec<Vec<Tile>>,
    mut blocked_slots: HashSet<(usize, usize)>,
) -> Option<bool> {
    (0..area.0[0].len() - 2)
        .cartesian_product(0..area.0.len() - 2)
        .filter_map(|(x, y)| {
            if area.0[y][x] || blocked_slots.contains(&(x, y)) {
                return None;
            }
            match tile_counts
                .iter()
                .enumerate()
                .filter_map(|(i, c)| {
                    if *c == 0 {
                        return None;
                    }
                    tiles[i]
                        .iter()
                        .filter_map(|tile| {
                            let mut tile_counts = tile_counts.clone();
                            if let Some(new_area) = area.place(tile, x, y) {
                                tile_counts[i] -= 1;
                                if tile_counts.iter().sum::<u32>() == 0 {
                                    return Some(true);
                                }
                                return fill_area(
                                    new_area,
                                    tile_counts,
                                    tiles,
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
                    blocked_slots.insert((x, y));
                    None
                }
            }
        })
        .next()
}

pub fn part_1() {
    let (tiles, areas) = get_data();

    let mut counter = 0;
    for (area, tile_counts) in areas.iter() {
        if tile_counts
            .iter()
            .enumerate()
            .map(|(i, tile_count)| tile_count * tiles[i][0].cell_count())
            .sum::<u32>()
            > (area.0.len() * area.0[0].len()) as u32
        {
            continue;
        }

        if fill_area(area.clone(), tile_counts.clone(), &tiles, HashSet::new()).is_some() {
            counter += 1;
        }
    }

    assert_eq!(counter, 548);
    println!("Part 1: {counter}");
}

mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

mod utils;

use dotenvy::dotenv;

fn main() {
    dotenv().unwrap();

    println!("Day 1");
    day1::part_1();
    day1::part_2();

    println!("---------\nDay 2");
    day2::part_1();
    day2::part_2();

    println!("---------\nDay 3");
    day3::part_1();
    day3::part_2();

    println!("---------\nDay 4");
    day4::part_1();
    day4::part_2();

    println!("---------\nDay 5");
    day5::part_1();
    day5::part_2();

    println!("---------\nDay 6");
    day6::part_1();
    day6::part_2();

    println!("---------\nDay 7");
    day7::part_1();
    day7::part_2();

    println!("---------\nDay 8");
    day8::part_1();
    day8::part_2();

    println!("---------\nDay 9");
    day9::part_1();
    day9::part_2();

    println!("---------\nDay 10");
    day10::part_1();
    day10::part_2();
}

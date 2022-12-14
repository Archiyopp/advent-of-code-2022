use advent_of_code_2022::read_file;

use crate::solutions::{
    day_1::day_1, day_2::day_2, day_3::day_3, day_4::day_4, day_5::day_5, day_6::day_6,
    day_9::day_9,
};

pub mod solutions;
fn main() {
    let first_day_result = day_1(read_file("inputs", "first"));
    println!(
        "Day 1, first answer: {}, second answer: {}",
        first_day_result[0],
        first_day_result.iter().sum::<i32>()
    );
    day_2(read_file("inputs", "second"));
    day_3();
    day_4();
    day_5();
    day_6();
    day_9();
}

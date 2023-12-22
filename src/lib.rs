pub mod puzzle_1;
pub mod puzzle_10;
pub mod puzzle_11;
pub mod puzzle_12;
pub mod puzzle_13;
pub mod puzzle_14;
pub mod puzzle_15;
pub mod puzzle_17;
pub mod puzzle_18;
pub mod puzzle_2;
pub mod puzzle_20;
pub mod puzzle_22;
pub mod puzzle_3;
pub mod puzzle_4;
pub mod puzzle_5;
pub mod puzzle_6;
pub mod puzzle_7;
pub mod puzzle_8;
pub mod puzzle_9;

mod util;

use std::fmt::Display;

pub fn solve_puzzle(id: (u32, u32)) -> Box<dyn Display> {
    Box::new(match id {
        (1, 1) => format!("{}", puzzle_1::puzzle_1_1()),
        (1, 2) => format!("{}", puzzle_1::puzzle_1_2()),
        (2, 1) => format!("{}", puzzle_2::puzzle_2_1()),
        (2, 2) => format!("{}", puzzle_2::puzzle_2_2()),
        (3, 1) => format!("{}", puzzle_3::puzzle_3_1()),
        (3, 2) => format!("{}", puzzle_3::puzzle_3_2()),
        (4, 1) => format!("{}", puzzle_4::puzzle_4_1()),
        (4, 2) => format!("{}", puzzle_4::puzzle_4_2()),
        (5, 1) => format!("{}", puzzle_5::puzzle_5_1()),
        (5, 2) => format!("{}", puzzle_5::puzzle_5_2()),
        (6, 1) => format!("{}", puzzle_6::puzzle_6_1()),
        (6, 2) => format!("{}", puzzle_6::puzzle_6_2()),
        (7, 1) => format!("{}", puzzle_7::puzzle_7_1()),
        (7, 2) => format!("{}", puzzle_7::puzzle_7_2()),
        (8, 1) => format!("{}", puzzle_8::puzzle_8_1()),
        (8, 2) => format!("{}", puzzle_8::puzzle_8_2()),
        (9, 1) => format!("{}", puzzle_9::puzzle_9_1()),
        (9, 2) => format!("{}", puzzle_9::puzzle_9_2()),
        (10, 1) => format!("{}", puzzle_10::puzzle_10_1()),
        (11, 1) => format!("{}", puzzle_11::puzzle_11_1()),
        (11, 2) => format!("{}", puzzle_11::puzzle_11_2()),
        (12, 1) => format!("{}", puzzle_12::puzzle_12_1()),
        (13, 1) => format!("{}", puzzle_13::puzzle_13_1()),
        (14, 1) => format!("{}", puzzle_14::puzzle_14_1()),
        (15, 1) => format!("{}", puzzle_15::puzzle_15_1()),
        (15, 2) => format!("{}", puzzle_15::puzzle_15_2()),
        (17, 1) => format!("{}", puzzle_17::puzzle_17_1()),
        (18, 1) => format!("{}", puzzle_18::puzzle_18_1()),
        (18, 2) => format!("{}", puzzle_18::puzzle_18_2()),
        (20, 1) => format!("{}", puzzle_20::puzzle_20_1()),
        (22, 1) => format!("{}", puzzle_22::puzzle_22_1()),
        (1..=25, 1..=2) => String::from("Still unknown..."),
        (_, _) => String::from("Forever unknown..."),
    })
}

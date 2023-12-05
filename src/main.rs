mod puzzle_1;
mod puzzle_2;
mod puzzle_3;
mod puzzle_4;
mod util;

use std::fmt::Display;

fn solve_puzzle(id: (u32, u32)) -> Box<dyn Display> {
    Box::new(match id {
        (1, 1) => format!("{}", puzzle_1::puzzle_1()),
        (2, 1) => format!("{}", puzzle_2::puzzle_2_1()),
        (2, 2) => format!("{}", puzzle_2::puzzle_2_2()),
        (3, 1) => format!("{}", puzzle_3::puzzle_3_1()),
        (3, 2) => format!("{}", puzzle_3::puzzle_3_2()),
        (4, 1) => format!("{}", puzzle_4::puzzle_4_1()),
        (4, 2) => format!("{}", puzzle_4::puzzle_4_2()),
        (1..=25, 1..=2) => String::from("Still unknown..."),
        (_, _) => String::from("Forever unknown..."),
    })
}

fn main() {
    if let Some(arg) = std::env::args().skip(1).next() {
        let mut id_parts = arg.split('.');
        let id = (
            id_parts
                .next()
                .expect("major ID")
                .parse::<u32>()
                .expect("not a number"),
            id_parts
                .next()
                .expect("minor ID")
                .parse::<u32>()
                .expect("not a number"),
        );
        let answer = solve_puzzle(id);
        println!("The answer to puzzle {}.{} is:", id.0, id.1);
        println!("{answer}");
    } else {
        eprintln!("Please specify a puzzle id!");
    }
}

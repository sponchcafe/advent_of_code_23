mod puzzle_1;
mod util;

use std::fmt::Display;

fn solve_puzzle(id: (u32, u32)) -> Box<dyn Display> {
    Box::new(match id {
        (1, 1) => puzzle_1::puzzle_1(),
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

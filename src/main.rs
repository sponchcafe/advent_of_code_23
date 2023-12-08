use aoc32lib::solve_puzzle;

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

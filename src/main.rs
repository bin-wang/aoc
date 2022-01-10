use clap::Parser;

use aoc::y21;
use aoc::Solution;

#[derive(Parser)]
#[clap(about, author)]
struct Args {
    day: u32,
    part: u32,
    #[clap(name = "file", short, long, default_value = "input.txt")]
    input_file: String,
}

fn main() {
    let args = Args::parse();
    let solution_by_day: Box<dyn Solution> = match args.day {
        1 => Box::new(y21::Day01 {}),
        2 => Box::new(y21::Day02 {}),
        3 => Box::new(y21::Day03 {}),
        4 => Box::new(y21::Day04 {}),
        5 => Box::new(y21::Day05 {}),
        6 => Box::new(y21::Day06 {}),
        7 => Box::new(y21::Day07 {}),
        8 => Box::new(y21::Day08 {}),
        9 => Box::new(y21::Day09 {}),
        10 => Box::new(y21::Day10 {}),
        11 => Box::new(y21::Day11 {}),
        12 => Box::new(y21::Day12 {}),
        13 => Box::new(y21::Day13 {}),
        14 => Box::new(y21::Day14 {}),
        15 => Box::new(y21::Day15 {}),
        16 => Box::new(y21::Day16 {}),
        _ => unimplemented!(),
    };

    let ans = if args.part == 1 {
        solution_by_day.part_1(&args.input_file)
    } else {
        solution_by_day.part_2(&args.input_file)
    };

    println!("{}", ans);
}

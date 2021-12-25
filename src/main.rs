use clap::Parser;

mod solution;
mod y21;

#[derive(Parser)]
#[clap(version = "0.1.0", author = "Bin Wang <wkk@users.noreply.github.com>")]
struct Opts {
    day: u32,
    part: u32,
    input_file: String,
}

fn main() {
    let opts: Opts = Opts::parse();
    let solution_by_day: Box<dyn solution::Solution> = match opts.day {
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
        _ => unimplemented!(),
    };

    let answer: String = if opts.part == 1 {
        solution_by_day.part_1(opts.input_file.as_str())
    } else {
        solution_by_day.part_2(opts.input_file.as_str())
    };

    println!("{}", answer);
}

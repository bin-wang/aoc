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
        _ => unimplemented!(),
    };

    let answer: String = if opts.part == 1 {
        solution_by_day.part_1(opts.input_file.as_str())
    } else {
        solution_by_day.part_2(opts.input_file.as_str())
    };

    println!("{}", answer);
}

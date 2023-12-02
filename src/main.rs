use clap::Parser;

mod args;
use args::{Args, Commands};
use default::Solution;

const DEFAULT_YEAR: u32 = 2023;

fn main() {
    let args = Args::parse();
    match args.command {
        Commands::Run { .. } => {}
        Commands::List { year } => {
            let year = year.unwrap_or(DEFAULT_YEAR);
            let solutions = get_year(year);
            println!("[*] Solutions for {year}:");
            println!("Day {:20} {:<20} {:<20}", "Name", "Part 1", "Part 2");
            for day in 1..=25 {
                if let Some(solution) = solutions.get(day - 1) {
                    let input = default::load(year, day as u32).expect("No input found!");

                    println!(
                        "{day:02}: {:20} {:<20} {:<20}",
                        solution.name(),
                        solution.p1(&input).to_string(),
                        solution.p2(&input).to_string()
                    );
                } else {
                    println!("{:02}: {:20} {2:<20} {2:<20}", day, "N/A", "Unreleased");
                }
            }
        }
    }
}

fn get_year(year: u32) -> &'static [&'static dyn Solution] {
    match year {
        2023 => aoc_2023::ALL,
        _ => &[],
    }
}

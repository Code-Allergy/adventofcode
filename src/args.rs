use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[clap(about = "Run a solution to a problem")]
    Run {
        day: u32,
        part: char,
        year: Option<u32>,
    },
    #[clap(about = "List all solutions for a given year")]
    List { year: Option<u32> },
}

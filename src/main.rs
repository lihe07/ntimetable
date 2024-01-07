mod initial;
mod optimize;
pub mod pareto;
mod project;
mod utils;

use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    #[arg(default_value = "./demo")]
    project: String,
}

fn main() {
    let args = Args::parse();

    let proj = project::Project::parse(args.project);
}

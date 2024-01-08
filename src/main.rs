pub mod initial;
mod neighborhoods;
mod optimize;
pub mod pareto;
pub mod project;
mod score;
mod tsp;
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

    dbg!(&proj);

    initial::find_initial_solution(&proj, true);
}

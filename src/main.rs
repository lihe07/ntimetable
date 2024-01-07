pub mod initial;
mod optimize;
pub mod pareto;
pub mod project;
mod score;
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

    initial::find_initial_solution_tabu(&proj, true);
}

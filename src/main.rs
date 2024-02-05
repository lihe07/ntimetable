#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

mod criteria;
pub mod initial;
pub mod log;
pub mod neighborhoods;
pub mod optimize;
pub mod pareto;
pub mod project;
mod tsp;
mod utils;

use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    #[arg(default_value = "./demo")]
    project: String,

    #[arg(short, long)]
    only_initial: bool,
}

fn main() {
    let args = Args::parse();

    let proj = project::Project::parse(&args.project);

    let static_ref: &'static project::Project = Box::leak(Box::new(proj));

    let s = initial::find_initial_solution(static_ref, true);

    if args.only_initial {
        if let Some(s) = s {
            println!("{}", utils::make_table(&s, static_ref, None));
        }
        return;
    }

    if let Some(s) = s {
        let s = optimize::optimize_solution(s, static_ref);

        log::finish(static_ref, args.project, s.clone());

        // for s in s {
        //     println!("{}", utils::make_table(&s, static_ref, None));
        // }
    }
}

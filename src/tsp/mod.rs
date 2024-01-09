use rand::Rng;

use crate::project::{Project, Room};

mod christofides;
mod naive;
mod sa;

pub use naive::solve;

fn random_adjacency_matrix(size: usize) -> Vec<Vec<i32>> {
    let mut rng = rand::thread_rng();

    let mut adj_matrix = vec![vec![0; size]; size];

    for i in 0..size {
        for j in i + 1..size {
            let weight = rng.gen_range(1..=10);
            adj_matrix[i][j] = weight;
            adj_matrix[j][i] = weight;
        }
    }

    adj_matrix
}

fn calculate_total_distance(path: &Vec<usize>, adj: &Vec<Vec<i32>>) -> i32 {
    let mut total_distance = 0;
    for i in 0..path.len() {
        let j = if i + 1 < path.len() { i + 1 } else { 0 };
        total_distance += adj[path[i]][path[j]];
    }
    total_distance
}

fn calculate_total_distance_proj(path: &Vec<(usize, Room)>, project: &Project) -> i32 {
    let mut total_distance = 0;
    for i in 0..path.len() {
        let j = if i + 1 < path.len() { i + 1 } else { 0 };
        // total_distance += adj[path[i]][path[j]];
        total_distance += project.rooms.distance(&path[i].1, &path[j].1);
    }
    total_distance
}

mod test {
    use super::*;

    #[test]
    fn test_all_tsp() {
        let size = 1000;
        let adjacency_matrix = crate::tsp::random_adjacency_matrix(size);

        // let mut naive_solution = naive::solve(&adjacency_matrix);
        // naive_solution.dedup();
        // assert_eq!(naive_solution.len(), size);
        // dbg!(crate::tsp::calculate_total_distance(
        //     &naive_solution,
        //     &adjacency_matrix
        // ));

        let mut chri_solution = christofides::solve(&adjacency_matrix);
        chri_solution.dedup();
        assert_eq!(chri_solution.len(), size);
        dbg!(crate::tsp::calculate_total_distance(
            &chri_solution,
            &adjacency_matrix
        ));

        let mut sa_solution = sa::solve(&adjacency_matrix);
        sa_solution.dedup();
        assert_eq!(sa_solution.len(), size);
        dbg!(crate::tsp::calculate_total_distance(
            &sa_solution,
            &adjacency_matrix
        ));
    }
}

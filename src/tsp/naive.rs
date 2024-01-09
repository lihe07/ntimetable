use crate::project::{Project, Room};

pub fn solve(rooms: Vec<(usize, Room)>, project: &Project) -> Vec<(usize, Room)> {
    let mut path = rooms;
    let mut min_distance = super::calculate_total_distance_proj(&path, &project);

    for _ in 0..1000 {
        let mut new_path = path.clone();
        let idx1 = rand::random::<usize>() % path.len();
        let idx2 = rand::random::<usize>() % path.len();
        new_path.swap(idx1, idx2);
        let new_distance = super::calculate_total_distance_proj(&new_path, &project);
        if new_distance < min_distance {
            path = new_path;
            min_distance = new_distance;
        }
    }

    path
}

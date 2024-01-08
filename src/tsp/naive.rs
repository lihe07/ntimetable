pub fn solve(adj: &Vec<Vec<i32>>) -> Vec<usize> {
    let mut path: Vec<usize> = (0..adj.len()).collect();
    let mut min_distance = super::calculate_total_distance(&path, &adj);

    for _ in 0..1000 {
        let mut new_path = path.clone();
        let idx1 = rand::random::<usize>() % path.len();
        let idx2 = rand::random::<usize>() % path.len();
        new_path.swap(idx1, idx2);
        let new_distance = super::calculate_total_distance(&new_path, &adj);
        if new_distance < min_distance {
            path = new_path;
            min_distance = new_distance;
        }
    }

    path
}

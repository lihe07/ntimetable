use rand::Rng;

pub fn solve(adj: &Vec<Vec<i32>>) -> Vec<usize> {
    let mut current_path: Vec<usize> = (0..adj.len()).collect();
    let mut current_distance = super::calculate_total_distance(&current_path, &adj);

    let mut best_path = current_path.clone();
    let mut best_distance = current_distance;

    let temperature_start = 1000.0;
    let mut temperature = temperature_start;

    while temperature > 1.0 {
        let mut new_path = current_path.clone();
        let idx1 = rand::random::<usize>() % current_path.len();
        let idx2 = rand::random::<usize>() % current_path.len();
        new_path.swap(idx1, idx2);

        let new_distance = super::calculate_total_distance(&new_path, &adj);
        let delta_distance = new_distance - current_distance;

        if delta_distance < 0
            || rand::random::<f64>() < (-delta_distance as f64 / temperature).exp()
        {
            // Accept the new solution
            current_path = new_path;
            current_distance = new_distance;
        }

        // Update the best solution if needed
        if current_distance < best_distance {
            best_path = current_path.clone();
            best_distance = current_distance;
        }

        // Decrease temperature
        temperature *= 0.995; // You can experiment with the cooling rate
    }

    best_path
}

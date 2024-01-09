use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

pub fn solve(adj: &Vec<Vec<i32>>) -> Vec<usize> {
    let n = adj.len();

    // Step 1: Minimum Spanning Tree (MST)
    let mut mst_adj = prim_mst(&adj);

    // Step 2: Minimum Perfect Matching (MPM) on odd-degree vertices
    let mut odd_vertices: HashSet<usize> = HashSet::new();
    for i in 0..n {
        if mst_adj[i].len() % 2 != 0 {
            odd_vertices.insert(i);
        }
    }

    let mpm_edges = minimum_perfect_matching(&adj, &odd_vertices);

    // Step 3: Combine MST and MPM to form a multigraph
    for (u, v) in mpm_edges {
        mst_adj[u].push((v, adj[u][v]));
        mst_adj[v].push((u, adj[u][v]));
    }

    // Step 4: Eulerian Circuit on the multigraph
    let eulerian_circuit = find_eulerian_circuit(&mut mst_adj);

    // Step 5: Shorten the circuit to get the TSP tour
    let mut visited = vec![false; n];
    let mut tsp_tour = Vec::new();

    for node in eulerian_circuit {
        if !visited[node] {
            tsp_tour.push(node);
            visited[node] = true;
        }
    }

    tsp_tour
}

// Helper function for Step 1: Prim's Minimum Spanning Tree algorithm
fn prim_mst(adj: &Vec<Vec<i32>>) -> Vec<Vec<(usize, i32)>> {
    let n = adj.len();
    let mut mst_adj = vec![Vec::new(); n];
    let mut min_heap = BinaryHeap::new();
    let mut visited = vec![false; n];

    min_heap.push((Reverse(0), 0)); // (Reverse(weight), vertex)

    while let Some((_, u)) = min_heap.pop() {
        if visited[u] {
            continue;
        }

        visited[u] = true;

        for v in 0..n {
            if !visited[v] && adj[u][v] > 0 {
                min_heap.push((Reverse(adj[u][v]), v));
                mst_adj[u].push((v, adj[u][v]));
                mst_adj[v].push((u, adj[u][v]));
            }
        }
    }

    mst_adj
}

// Helper function for Step 2: Minimum Perfect Matching on odd-degree vertices
fn minimum_perfect_matching(
    adj: &Vec<Vec<i32>>,
    odd_vertices: &HashSet<usize>,
) -> Vec<(usize, usize)> {
    // Placeholder implementation (Naive greedy algorithm)
    let mut matching_edges = Vec::new();

    for &u in odd_vertices {
        let mut min_weight = i32::MAX;
        let mut min_v = 0;

        for &v in odd_vertices {
            if u != v && adj[u][v] < min_weight {
                min_weight = adj[u][v];
                min_v = v;
            }
        }

        matching_edges.push((u, min_v));
    }

    matching_edges
}

// Helper function for Step 4: Hierholzer's algorithm for finding Eulerian Circuit
fn find_eulerian_circuit(adj: &mut Vec<Vec<(usize, i32)>>) -> Vec<usize> {
    let mut circuit = Vec::new();
    let mut stack = Vec::new();
    let mut current_vertex = 0;

    stack.push(current_vertex);

    while !stack.is_empty() {
        if adj[current_vertex].is_empty() {
            // If the current vertex has no more outgoing edges, add it to the circuit.
            circuit.push(current_vertex);
            current_vertex = stack.pop().unwrap_or(0); // Pop from stack and set as current vertex
        } else {
            // Otherwise, choose an arbitrary neighbor, remove the edge, and set the neighbor as the current vertex.
            let (next_vertex, _) = adj[current_vertex].pop().unwrap();
            stack.push(current_vertex);
            current_vertex = next_vertex;
        }
    }

    circuit.reverse(); // Reverse the circuit to get the correct order
    circuit
}

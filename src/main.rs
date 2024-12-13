//--------------------------------------------------------------
// Project Title: Average Distance Between Two Vertices in a Graph
//
// Description:
// This program reads an undirected graph from a file named "fb-pages-company_edges.txt".
// Each line in the file is expected to have an edge in the format "u,v".
// The program constructs the graph, performs a BFS from the first vertex mentioned,
// randomly selects up to 1000 distinct pairs of reachable vertices, computes shortest
// path distances for these pairs, and prints the average shortest path distance.
//
// Steps:
// 1. Read edges from file.
// 2. Construct an undirected graph.
// 3. Run BFS to find which vertices are reachable from the first vertex.
// 4. Randomly select up to 1000 distinct pairs of these reachable vertices.
// 5. Compute shortest path distances using BFS for each pair.
// 6. Print the average shortest path distance.
//
// This code uses:
// - Basic Rust features (structs, vectors, loops, if statements).
// - The `rand` crate for generating random indices.
// - BFS algorithm for graph traversal and shortest path calculations.
//
//--------------------------------------------------------------

// import crates
use std::io::BufRead;
use std::fs::File;
use std::collections::{VecDeque, HashSet};
use rand::Rng;

struct Graph {
    n: usize,
    adjacency: Vec<Vec<usize>>,
}

fn main() {
    // Print Title
    println!("--------------------------------------------------------");
    println!("   Average Distance Between Two Vertices in a Graph");
    println!("--------------------------------------------------------");

    // Step 1: Read the edge list from file
    let edges = match read_edge_list("fb-pages-company_edges.txt") {
        Some(e) => e,
        None => {
            eprintln!("Error: Could not read a valid edge list from the file.");
            return;
        }
    };

    if edges.is_empty() {
        eprintln!("Error: The edge list is empty. Cannot proceed.");
        return;
    }

    // Determine the number of vertices in the graph
    let max_vertex_index = edges.iter().flat_map(|&(u,v)| [u,v]).max().unwrap_or(0);
    let total_vertices = max_vertex_index + 1;

    // Step 2: Construct an undirected graph
    let mut adjacency_lists = vec![Vec::new(); total_vertices];
    for &(u, v) in &edges {
        if u < total_vertices && v < total_vertices {
            adjacency_lists[u].push(v);
            adjacency_lists[v].push(u);
        }
    }

    // Sort adjacency lists for consistency
    for neighbors in &mut adjacency_lists {
        neighbors.sort();
    }

    let graph = Graph { n: total_vertices, adjacency: adjacency_lists };

    // Step 3: Perform a BFS from the first vertex found in the edges
    let start_vertex = edges[0].0;
    let visited_vertices = bfs_traverse(&graph, start_vertex);
    println!("\n- BFS started from vertex {} and visited {} vertices.",
             start_vertex, visited_vertices.len());

    if visited_vertices.len() < 2 {
        println!("Not enough visited vertices to form pairs (need at least 2).");
        return;
    }

    // Step 4: Randomly select up to 1000 distinct pairs of reachable vertices
    // We will choose pairs (a,b) where a<b to avoid duplicates like (b,a).
    let pair_sample_size = 1000;
    let mut rng = rand::thread_rng();
    let visited_count = visited_vertices.len();
    let mut chosen_pairs = HashSet::new();
    let mut random_pairs = Vec::new();

    let max_attempts = pair_sample_size * 100;
    let mut attempts = 0;

    while random_pairs.len() < pair_sample_size && attempts < max_attempts {
        let i = rng.gen_range(0..visited_count);
        let j = rng.gen_range(0..visited_count);

        if i != j {
            let a = visited_vertices[i];
            let b = visited_vertices[j];
            let ordered_pair = if a < b { (a,b) } else { (b,a) };

            if !chosen_pairs.contains(&ordered_pair) {
                chosen_pairs.insert(ordered_pair);
                random_pairs.push(ordered_pair);
            }
        }
        attempts += 1;
    }

    if random_pairs.is_empty() {
        println!("Could not form any distinct pairs.");
        return;
    }

    // Step 5: Compute shortest path distances for each pair
    let mut total_distance = 0;
    let mut counted_pairs = 0;
    for &(a, b) in &random_pairs {
        let dist = shortest_path(&graph, a, b);
        if dist != std::usize::MAX {
            total_distance += dist;
            counted_pairs += 1;
        }
    }

    if counted_pairs == 0 {
        println!("None of the selected pairs are reachable from each other.");
        return;
    }

    // Step 6: Calculate and print the average shortest path distance
    let average_distance = total_distance as f64 / counted_pairs as f64;
    println!("- Computed distances for {} pairs.", counted_pairs);
    println!("- Total combined distance: {}", total_distance);
    println!("- Estimated average shortest path distance: {:.4}", average_distance);

    println!("--------------------------------------------------------");
    println!("Run Completed.");
    println!("--------------------------------------------------------");
}

// Reads an edge list from a file specified by `path`.
// Each line should be in the format "u,v" where u and v are integers.
// Returns Some(vector_of_edges) if successful, or None if no edges found.
fn read_edge_list(path: &str) -> Option<Vec<(usize, usize)>> {
    let file = File::open(path).ok()?;
    let mut lines = std::io::BufReader::new(file).lines();

    // Skip possible header line 
    lines.next();

    let mut edges = Vec::new();
    for line_result in lines {
        if let Ok(line_str) = line_result {
            let parts: Vec<&str> = line_str.trim().split(',').collect();
            if parts.len() == 2 {
                if let (Ok(a), Ok(b)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
                    edges.push((a, b));
                }
            }
        }
    }
    if edges.is_empty() { None } else { Some(edges) }
}

// Performs a BFS starting from `start_vertex`, returning a vector of visited vertices.
// If `start_vertex` is invalid, returns an empty vector.
fn bfs_traverse(graph: &Graph, start_vertex: usize) -> Vec<usize> {
    if start_vertex >= graph.n {
        return Vec::new();
    }

    let mut visited = vec![false; graph.n];
    let mut queue = VecDeque::new();
    let mut visited_order = Vec::new();

    visited[start_vertex] = true;
    queue.push_back(start_vertex);

    while let Some(current) = queue.pop_front() {
        visited_order.push(current);
        for &neighbor in &graph.adjacency[current] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back(neighbor);
            }
        }
    }
    visited_order
}

// Computes the shortest path distance between two vertices using BFS.
// Returns std::usize::MAX if no path is found.
fn shortest_path(graph: &Graph, start: usize, end: usize) -> usize {
    if start >= graph.n || end >= graph.n {
        return std::usize::MAX;
    }
    if start == end {
        return 0;
    }

    let mut distances = vec![std::usize::MAX; graph.n];
    let mut visited = vec![false; graph.n];
    let mut queue = VecDeque::new();

    visited[start] = true;
    distances[start] = 0;
    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        if current == end {
            return distances[end];
        }
        for &neighbor in &graph.adjacency[current] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                distances[neighbor] = distances[current] + 1;
                queue.push_back(neighbor);
            }
        }
    }
    std::usize::MAX
}

// Basic test for BFS traversal 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_small_graph() {
        // Construct a small graph:
        // 0 -- 1 -- 2
        // |    |
        // 3    4
        let edges = vec![(0,1),(1,2),(0,3),(1,4)];
        let n = 5;
        let mut adjacency = vec![Vec::new(); n];
        for &(u,v) in &edges {
            adjacency[u].push(v);
            adjacency[v].push(u);
        }
        for a in &mut adjacency {
            a.sort();
        }
        let graph = Graph { n, adjacency };

        let visited_result = bfs_traverse(&graph, 0);
        assert_eq!(visited_result.len(), 5);
    }

    #[test]
    fn test_shortest_path_small_graph() {
        let edges = vec![(0,1),(1,2),(0,3),(1,4)];
        let n = 5;
        let mut adjacency = vec![Vec::new(); n];
        for &(u,v) in &edges {
            adjacency[u].push(v);
            adjacency[v].push(u);
        }
        for a in &mut adjacency {
            a.sort();
        }
        let graph = Graph { n, adjacency };

        // Distance from 0 to 2 is 2 (0->1->2)
        let dist_0_2 = shortest_path(&graph, 0, 2);
        assert_eq!(dist_0_2, 2);

        // Distance from 3 to 4 is 3 (3->0->1->4)
        let dist_3_4 = shortest_path(&graph, 3, 4);
        assert_eq!(dist_3_4, 3);
    }
}

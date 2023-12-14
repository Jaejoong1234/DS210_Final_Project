use crate::readfiles::Graph;
use std::collections::VecDeque;
use rand::prelude::IteratorRandom;

type Vertex = usize;

// Compute the average distance from all distances collected
pub fn compute_average_distance_bfs(graph: &Graph) -> f32 {
    let (total_distance, num_pairs) = (0..graph.n)
        .map(|i| distance_bfs(i, graph))
        .enumerate()
        .flat_map(|(i, distances)| {
            distances.into_iter().enumerate().skip(i + 1)
                .filter_map(|(j, distance)| distance.map(|d| (d, j)))
        })
        .fold((0, 0), |(total, count), (distance, _)| (total + distance, count + 1));

    total_distance as f32 / num_pairs as f32
}

// BFS algorithm for finding distances
pub fn distance_bfs(start: Vertex, graph: &Graph) -> Vec<Option<u32>> {
    let mut distance = vec![None; graph.n];
    distance[start] = Some(0);
    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some(v) = queue.pop_front() {
        for &u in &graph.outedges[v] {
            if distance[u].is_none() {
                distance[u] = distance[v].map(|d| d + 1);
                queue.push_back(u);
            }
        }
    }
    distance
}

// Create subgraphs by randomly picking nodes and find average distance afterwards
pub fn subgraph(graph: &Graph, num_nodes: usize) -> Graph {
    let mut rng = rand::thread_rng();
    let random_nodes = (0..graph.n).choose_multiple(&mut rng, num_nodes);

    let mut node_map = vec![None; graph.n];
    random_nodes.iter().enumerate().for_each(|(i, &node)| node_map[node] = Some(i));

    let edges = random_nodes.iter()
        .flat_map(|&u| {
            let node_map_ref = &node_map;  // Create a reference to node_map here
            graph.outedges[u].iter()
                .filter_map(move |&v| {  // Capture `u` by value, use `node_map_ref`
                    node_map_ref[v].and_then(|i| node_map_ref[u].map(|j| (j, i)))
                })
        })
        .collect::<Vec<_>>();

    Graph::create_undirected(num_nodes, &edges)
}



#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    
    #[test]
    fn t_compute_average_distance_bfs() {
        let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)];
        let graph = Graph::create_undirected(5, &edges);
        assert_relative_eq!(compute_average_distance_bfs(&graph), 1.5);
    }
}

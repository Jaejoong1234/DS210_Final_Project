mod readfiles;
mod bfs;
mod component;

use crate::readfiles::{Graph, read_file};
use crate::component::count_components;
use crate::bfs::{compute_average_distance_bfs, subgraph};
type Vertex = usize;
type ListOfEdges = Vec<(Vertex, Vertex)>;

pub fn main() {
    // Read and process the graph data from a file
    let n = 4039; // Total number of vertices in the graph
    let mut edges: ListOfEdges = read_file("facebook_combined.txt");
    edges.sort_unstable(); 

    let graph = Graph::create_undirected(n, &edges); // Creating an undirected graph

    // Printing degree information
    print_degrees(&graph);
    
    // Printing average degree of the graph
    print_average_degree(&graph);
    
    // Printing the number of components in the graph
    print_number_of_components(&graph);
    
    // Calculating and printing average distances in the graph and its subgraphs
    print_average_distances(&graph);
}

/// Prints the degrees of the most popular and most isolated vertices
fn print_degrees(graph: &Graph) {
    let degrees = graph.get_all_degrees(); // Retrieving all degrees

    // Finding and printing the maximum degree
    if let Some(&max) = degrees.iter().max() {
        println!("Vertex with highest connectivity has {} connections.", max);
    } else {
        println!("The graph has no vertices.");
    }

    // Finding and printing the minimum degree
    if let Some(&min) = degrees.iter().min() {
        println!("Vertex with lowest connectivity has {} connections.", min);
    } else {
        println!("The graph has no vertices.");
    }
}

/// Prints the average degree of the graph
fn print_average_degree(graph: &Graph) {
    let total_degree: usize = graph.get_all_degrees().iter().sum(); // Summing all degrees
    let average_degree = total_degree as f64 / graph.n as f64; // Calculating average
    println!("Graph's average connectivity: {:.2}", average_degree);
}

/// Prints the number of components in the graph
fn print_number_of_components(graph: &Graph) {
    let num_components = count_components(graph); // Counting the components
    println!("Total connected components in the graph: {}", num_components);

}

/// Prints the average distances within the graph and its subgraphs
fn print_average_distances(graph: &Graph) {
    let num_nodes_list = [100, 200, 300, 500, 1000, 2000, 3000, 4000]; // Different sizes for subgraphs
    let num_experiments = 15; // Number of experiments per subgraph size
    for &num_nodes in &num_nodes_list {
        // Compute average distances for subgraphs of each size
        let total_distance: f32 = (0..num_experiments)
            .map(|_| compute_average_distance_bfs(&subgraph(graph, num_nodes)))
            .sum();
        let avg_distance = total_distance / num_experiments as f32;
        println!("Average distance among nodes in a subgraph of {} nodes: {}", num_nodes, avg_distance);
    }

    // Compute and print the average distance for the entire graph
    let avg_distance = compute_average_distance_bfs(graph);
    println!("Average distance among all pairs of nodes: {}", avg_distance);
}

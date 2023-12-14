use std::fs::File;
use std::io::prelude::*;

type Vertex = usize;
type ListOfEdges = Vec<(Vertex, Vertex)>;
type AdjacencyLists = Vec<Vec<Vertex>>;

/// Reads a graph file and returns a list of edges.
/// Assumes that each line of the file represents an edge in the format "u v".
pub fn read_file(path: &str) -> ListOfEdges {
    let mut result = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();

    for line in buf_reader {
        let line_str = line.expect("Error reading line");
        let vertices: Vec<&str> = line_str.trim().split_whitespace().collect();
        if vertices.len() == 2 {
            let u = vertices[0].parse::<Vertex>().unwrap();
            let v = vertices[1].parse::<Vertex>().unwrap();
            result.push((u, v));
        }
    }

    result
}

#[derive(Debug)]
pub struct Graph {
    pub n: usize, // Number of vertices
    pub outedges: AdjacencyLists, // Adjacency list representation
}

/// Reverses edges in a list of edges.
fn reverse_edges(list: &ListOfEdges) -> ListOfEdges {
    list.iter().map(|&(u, v)| (v, u)).collect()
}

impl Graph {
    /// Adds directed edges to the graph.
    pub fn add_directed_edges(&mut self, edges: &ListOfEdges) {
        for &(u, v) in edges {
            self.outedges[u].push(v);
        }
    }

    /// Sorts adjacency lists for each vertex.
    pub fn sort_graph_lists(&mut self) {
        for adj_list in self.outedges.iter_mut() {
            adj_list.sort();
        }
    }

    /// Creates a directed graph from a list of edges.
    pub fn create_directed(n: usize, edges: &ListOfEdges) -> Graph {
        let mut graph = Graph {
            n,
            outedges: vec![vec![]; n],
        };
        graph.add_directed_edges(edges);
        graph.sort_graph_lists();
        graph
    }

    /// Creates an undirected graph from a list of edges.
    pub fn create_undirected(n: usize, edges: &ListOfEdges) -> Graph {
        let mut graph = Self::create_directed(n, edges);
        graph.add_directed_edges(&reverse_edges(edges));
        graph.sort_graph_lists();
        graph
    }

    /// Gets the out-degree of a vertex.
    pub fn get_out_degree(&self, u: Vertex) -> usize {
        self.outedges[u].len()
    }

    /// Returns a vector containing the degrees of all vertices.
    pub fn get_all_degrees(&self) -> Vec<usize> {
        (0..self.n).map(|u| self.get_out_degree(u)).collect()
    }
}

#[test]
fn t_create_undirected() {
    let edges = vec![(0, 1), (1, 2), (2, 3), (3, 0)];
    let graph = Graph::create_undirected(4, &edges);
    assert_eq!(graph.get_out_degree(0), 2);
    assert_eq!(graph.get_out_degree(1), 2);
    assert_eq!(graph.get_out_degree(2), 2);
    assert_eq!(graph.get_out_degree(3), 2);
}

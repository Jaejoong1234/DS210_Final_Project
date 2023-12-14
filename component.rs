type Component = usize;
type Vertex = usize;
use crate::readfiles::Graph;
use std::collections::VecDeque;

pub fn mark_component_bfs(vertex: Vertex, graph: &Graph, component: &mut Vec<Option<Component>>, component_no: Component) {
    let mut queue = VecDeque::new();
    queue.push_back(vertex);
    component[vertex] = Some(component_no);

    while let Some(v) = queue.pop_front() {
        // Collect vertices that need their component marked
        let mut vertices_to_mark = Vec::new();
        for &w in &graph.outedges[v] {
            if component[w].is_none() {
                vertices_to_mark.push(w);
            }
        }
        // Perform the marking
        for w in vertices_to_mark {
            component[w] = Some(component_no);
            queue.push_back(w);
        }
    }
}


// Count the total number of components in the graph
pub fn count_components(graph: &Graph) -> usize {
    let mut component = vec![None; graph.n];
    let mut component_no = 0;

    for v in 0..graph.n {
        if component[v].is_none() {
            mark_component_bfs(v, graph, &mut component, component_no);
            component_no += 1;
        }
    }

    component_no
}

#[test]
fn t_count_components() {
    let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)];
    let graph = Graph::create_undirected(5, &edges);
    assert_eq!(count_components(&graph), 1);
    let edges = vec![(0, 1), (1, 2), (2, 0), (2, 3)];
    let graph = Graph::create_undirected(4, &edges);
    assert_eq!(count_components(&graph), 1);
    let edges = vec![(0, 1), (2, 3)];
    let graph = Graph::create_undirected(4, &edges);
    assert_eq!(count_components(&graph), 2);
}
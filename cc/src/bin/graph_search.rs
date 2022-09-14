use std::collections::{HashMap, HashSet, VecDeque};

type Node = usize;
type Cost = usize;

struct Graph {
    edges: HashMap<Node, Vec<(Node, Cost)>>,
    nodes: HashSet<Node>,
}

impl Graph {
    fn from_edge_list(edge_list: &Vec<(Node, Node, Cost)>) -> Self {
        let mut adjacency_list: HashMap<Node, Vec<(Node, Cost)>> = HashMap::new();
        let mut nodes = HashSet::new();

        for &(source, destination, cost) in edge_list.iter() {
            let destinations = adjacency_list
                .entry(source)
                .or_insert_with(|| Vec::new());

            destinations.push((destination, cost));

            nodes.insert(source);
            nodes.insert(destination);
        }

        Graph {
            edges: adjacency_list,
            nodes,
        }
    }
}


fn shortest_path(g: &Graph, start: Node, goal: Node) -> Option<(Vec<Node>, Cost)> {

    let mut path: Vec<Node> = Vec::with_capacity(g.nodes.len());
    let mut queue = VecDeque::new();
    let mut best_cost = u32::MAX as Cost;
    let mut best_path = None;

    //let mut visited = HashSet::<Node>::new();

    // push start node in the DFS queue
    // Node/Cost, trail path pointer, cost)
    queue.push_front(((start,0), 0usize, 0 as Cost));

    // while a node in the queue pick the node
    while let Some((node, path_pos, path_cost)) = queue.pop_front() {

        // push start node in the path/trail
        path.truncate(path_pos);
        path.push(node.0);

        println!("\t Scan: ({:?}, {path_cost})", path);
        if let Some(edges) = g.edges.get(&node.0) {

            // for each edge
            for edge in edges {

                // if edge node is the target node
                // and the cost to get there still better than best cost
                let edge_cost = path_cost + edge.1;
                if edge_cost < best_cost {
                    if edge.0 == goal {
                        best_cost = edge_cost;
                        path.push(edge.0);
                        best_path = Some( (path.clone(), best_cost) );
                        println!("\t Path!: {:?}", best_path);
                        path.pop();
                    } else {
                        if !path.contains(&edge.0) {
                            // push edge node to further explore
                            queue.push_front((*edge, path.len(), edge_cost));
                        }
                    }
                }
                else {
                    println!("\t\t\t Ignore: Edge::({:?}) has path cost ({edge_cost}) that exceeds best cost::({best_cost})",edge);
                }
            }
        } else {
            path.pop();
        }

    }

    println!("Path: {:?}", best_path);
    best_path
}

fn main() {
    // let edge_list = include!("large_graph.in");
    // let g = Graph::from_edge_list(&edge_list);
    //
    // if let Some((path, cost)) = shortest_path(
    //     &g, 1000, 9000) {
    //     println!("1000->9000, {:?} {}", path, cost);
    // };
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn large_graph() {
    //     let edge_list = include!("large_graph.in");
    //     let g = Graph::from_edge_list(&edge_list);
    //
    //     let path = shortest_path(&g, 1000, 4000);
    //     assert!(path.is_some());
    //     assert_eq!(path.unwrap().1, 24);
    // }
    #[test]
    fn small_graph() {
        let edge_list = include!("small_graph.in");
        let g = Graph::from_edge_list(&edge_list);

        let path = shortest_path(&g, 2, 5);
        assert!(path.is_some());
        assert_eq!(path.unwrap().1, 4);
    }
}
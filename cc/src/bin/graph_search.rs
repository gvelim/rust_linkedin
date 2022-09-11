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


fn shortest_path(g: &Graph, start: Node, goal: Node) -> Option<(Vec<(Node,Cost)>, Cost)> {

    let mut paths: Vec<(Vec<(Node, Cost)>, Cost)> = Vec::new();
    let mut path: Vec<(Node,Cost)> = Vec::new();
    let mut queue = VecDeque::new();
    let mut trail_cost = u32::MAX as Cost;
    let mut best_cost = u32::MAX as Cost;

    //let mut visited = HashSet::<Node>::new();

    // push start node in the DFS queue
    // Node/Cost, trail path pointer, cost)
    queue.push_front(((start,0), 0usize, 0 as Cost));

    // while a node in the queue pick the node
    while let Some((node, path_pos, cost)) = queue.pop_front() {

        // push start node in the path
        path.truncate(path_pos);
        path.push(node);
        trail_cost = cost + node.1;

        println!("\t\t Scan: ({trail_cost})::{:?}", path);

        // if the path cost is higher than the path already found don't pursue any further
        if trail_cost < best_cost {
            if let Some(edges) = g.edges.get(&node.0) {

                // for each edge
                for edge in edges {

                    // if edge node is the target node
                    if edge.0 == goal {
                        // capture all visited (node,cost) in the stack
                        best_cost = trail_cost + edge.1;
                        path.push(*edge);
                        paths.push((path.clone(), best_cost));
                        println!("\t Path!: ({best_cost})::{:?}", path);
                        path.pop();
                    } else {
                        if !path.contains(edge) {
                            // push edge node to further explore
                            queue.push_front((*edge, path.len(), trail_cost));
                        }
                    }
                }
            } else {
                path.pop();
            }
        } else {
            println!("\t\t trail cost exceeds best cost");
        }

    }

    // select lowest cost
    let mut lowest_cost= u32::MAX as Cost;
    paths.iter()
        .fold( None, |mut shorted_path, path| {
            if path.1 < lowest_cost {
                println!("\t Shortest Path! {:?}", path);
                lowest_cost = path.1;
                shorted_path = Some(path.clone())
            }
            shorted_path
        })
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
    //     let path = shortest_path(&g, 1000, 9000);
    //     assert!(path.is_some());
    //     assert_eq!(path.unwrap().1, 24);
    // }
    #[test]
    fn small_graph() {
        let edge_list = include!("small_graph.in");
        let g = Graph::from_edge_list(&edge_list);

        let path = shortest_path(&g, 1, 6);
        assert!(path.is_some());
        assert_eq!(path.unwrap().1, 4);
    }
}
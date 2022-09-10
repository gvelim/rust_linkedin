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

    let mut paths = Vec::new();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::<Node>::new();

    // push start node in the DFS queue
    queue.push_front(((start,0), Vec::new()));

    // while a node in the queue pick the node
    while let Some((node, mut path)) = queue.pop_front() {

        // push start node in the path
        path.push(node);

        if let Some(edges) = g.edges.get(&node.0) {

            // for each edge
            for edge in edges {

                // if edge node is target node
                if edge.0 == goal {
                    // capture all visited (node,cost) in the stack
                    path.push(*edge);
                    paths.push(path.clone());
                    println!("\t Path! {:?}", path);
                    path.pop();
                } else {
                    // push node
                    queue.push_front((*edge, path.clone()));
                    // continue
                }
            }
        } else {
            path.pop();
        }
    }

    // calculate path cost
    let mut p : Vec<(Vec<Node>, Cost)> = Vec::new() ;
    paths.iter()
        .for_each(|path| {
            p.push(path.iter()
                .fold( (Vec::<Node>::new(),0), |(mut nodes,mut cost), path| {
                    nodes.push(path.0);
                    cost += path.1;
                    (nodes,cost)
                })
            );
        });

    // select lowest cost
    let mut lowest_cost= u32::MAX as Cost;
    p.iter()
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

    #[test]
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

        let path = shortest_path(&g, 1, 5);
        assert!(path.is_some());
        assert_eq!(path.unwrap().1, 7);
    }
}
use std::collections::{
    HashMap,
    HashSet,
    vec_deque::VecDeque
};

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
    let mut node_cost = g.nodes.iter()
        .fold( HashMap::<Node,(Cost,Option<Node>)>::new(), |mut cost_history, node| {
            cost_history.entry(*node).or_insert( (Cost::MAX, None));
            cost_history
        });
    node_cost.entry(start).and_modify(|c| *c = (0, None) );
    let mut best_path = None;

    // push start node in the DFS queue
    // Node/Cost, trail path pointer, cost)
    queue.push_front(start);

    // while a node in the queue pick the node
    while let Some(node) = queue.pop_front() {

        if node == goal {
            path.push(node);
            let mut cur_node  = node;
            while let Some(parent) = node_cost[&cur_node].1 {
                path.push(parent);
                cur_node = parent;
            }
            best_path = Some((path.clone(), node_cost[&node].0));
            println!("\t Path!: {:?}", best_path);
            path.truncate(0);
        } else {
            let path_cost = node_cost[&node].0;
            if let Some(edges) = g.edges.get(&node) {

                // for each edge
                for edge in edges {

                    // calc the new path cost to the edge
                    let edge_cost = path_cost + edge.1;

                    // if new edge cost < existing edge cost
                    if edge_cost < node_cost[&edge.0].0 {

                        // set the new lower cost coming from new parent Node
                        node_cost.entry(edge.0)
                            .and_modify(|c|
                                *c = (edge_cost, Some(node))
                            );

                        queue.push_front(edge.0);
                    } else {
                        println!("\t\t\t Ignore: Edge::({:?}) has path cost ({edge_cost}) that exceeds previous cost", edge);
                    }
                }
            }
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
        assert_eq!(path.unwrap().1, 2);
    }
}
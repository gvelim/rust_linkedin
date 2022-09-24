use std::collections::{
    HashMap,
    HashSet,
    vec_deque::VecDeque
};

type Cost = usize;
type Node = usize;
enum Edge {
    N(Node),
    NC(Node, Cost)
}

struct Graph {
    edges: HashMap<Node, Vec<Edge>>,
    nodes: HashSet<Node>,
}

impl Graph {
    fn from_edge_list(edge_list: &Vec<(Node, Node, Cost)>) -> Self {
        let mut adjacency_list: HashMap<Node, Vec<Edge>> = HashMap::new();
        let mut nodes = HashSet::new();

        for &(source, destination, cost) in edge_list.iter() {
            let destinations = adjacency_list
                .entry(source)
                .or_insert_with(|| Vec::new());

            destinations.push(Edge::NC(destination, cost));

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

    let mut path: Vec<Node> = Vec::new();
    let mut queue = VecDeque::new();

    // reset all node costs to MAX value with no path-parent nodes
    let mut node_cost = g.nodes.iter()
        .fold( HashMap::<Node,(Cost,Option<Node>)>::new(), |mut cost_history, node| {
            cost_history.entry(*node).or_insert( (Cost::MAX, None));
            cost_history
        });
    // set cost at start node to zero with no parent
    node_cost.entry(start)
        .and_modify(
            |c| *c = (0, None)
        );
    let mut best_path = None;

    // push start node in the DFS queue
    queue.push_front(start);

    // while a node in the queue pick the node
    while let Some(node) = queue.pop_front() {

        let path_cost = node_cost[&node].0;

        // if node is the target node
        // assuming cost is the lowest cost
        if node == goal {
            // clear path for use in case we find another path
            // build the shortest path by pushing target node first
            path.clear();
            path.push(node);
            // set target as current node
            let mut cur_node= node;
            // backtrace all parents until you reached None, that is, the start node
            while let Some(parent) = node_cost[&cur_node].1 {
                path.push(parent);
                cur_node = parent;
            }
            best_path = Some((path.clone(), path_cost));
            println!("\t Path!: {:?}", best_path);
        } else {
            if let Some(edges) = g.edges.get(&node) {

                // for each edge
                edges.iter()
                    .filter_map(|edge| if let Edge::NC(edge,cost) = *edge { Some((edge, cost)) } else { panic!("Setup graph using enum type Edge::Cost") } )
                    .for_each(|(edge,cost)| {

                        // calc the new path cost to edge
                        let edge_cost = path_cost + cost;

                        // if new edge cost < currently known cost @edge
                        if edge_cost < node_cost[&edge].0 {

                            // set the new lower cost @node along with related parent Node
                            node_cost.entry(edge)
                                .and_modify(|c|
                                    *c = (edge_cost, Some(node))
                                );
                            // push_front for Depth First Search -> slower but finds all paths
                            // push_back for Breadth First Search -> faster but finds best only
                            queue.push_back(edge);
                        }
                    });
                }
            }
        }

    println!("Path: {:?}", best_path);
    best_path
}

fn main() {
    let edge_list = include!("large_graph.in");
    let g = Graph::from_edge_list(&edge_list);

    if let Some((path, cost)) = shortest_path(
        &g, 1000, 9000) {
        println!("1000->9000, {:?} {}", path, cost);
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn large_graph() {
        let edge_list = include!("large_graph.in");
        let g = Graph::from_edge_list(&edge_list);

        let path = shortest_path(&g, 1000, 9000);
        assert!(path.is_some());
        assert_eq!(path.unwrap().1, 22);
    }
    #[test]
    fn small_graph() {
        let edge_list = include!("../../../../CSX0003RUST/src/graphs/small_graph.in");
        let g = Graph::from_edge_list(&edge_list);

        let path = shortest_path(&g, 2, 5);
        assert!(path.is_some());
        assert_eq!(path.unwrap().1, 2);
    }
}
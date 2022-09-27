use std::collections::{
    HashMap,
    HashSet,
    vec_deque::VecDeque,
    BinaryHeap
};
use std::cmp::Ordering;

type Cost = usize;
type Node = usize;

#[derive(Clone, Copy)]
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

    // We are using a BinaryHeap queue in order to always have first in the queue
    // the node with lowest cost to explore next
    struct Step(Node,Cost);
    impl Eq for Step {}
    impl PartialEq<Self> for Step {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0 && self.1 == other.1
        }
    }
    impl PartialOrd<Self> for Step {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Ord for Step {
        fn cmp(&self, other: &Self) -> Ordering {
            // binary head is a max-heap implementation pushing to the top the biggest element self.cmp(other)
            // hence we need to reverse the comparison other.cmp(self)
            other.1.cmp(&self.1)
        }
    }

    let mut queue = BinaryHeap::new();

    // reset all node costs to MAX value with no path-parent nodes
    let mut node_cost : HashMap<Node, (Cost, Option<Node>)> = g.nodes.iter()
        .map( |&node| (node, (Cost::MAX, None)) )
        .collect() ;

        // set cost at start node to zero with no parent node
    node_cost.entry(start)
        .and_modify(
            |c| *c = (0, None)
        );

    // push start node in the BinaryHeap queue
    queue.push(Step(start,0));

    // while queue has nodes pick the node with the lowest cost
    while let Some(Step(node, path_cost)) = queue.pop() {

        // if we have found the the target node
        // then we have completed our search
        // (Dijkstra's algo property - all nodes are processed once)
        if node == goal {
            let mut path = VecDeque::new();
            // reconstruct the shortest path starting from the target node
            path.push_front(node);
            // set target as current node
            let mut cur_node= node;
            // backtrace all parents until you reach None, that is, the start node
            while let Some(parent) = node_cost[&cur_node].1 {
                path.push_front(parent);
                cur_node = parent;
            }
            println!("\t Path!: {:?} [{path_cost}]", path);
            return Some((path.into(), path_cost));
        } else {
            if let Some(edges) = g.edges.get(&node) {
                edges.iter()
                    .filter_map(|&edge| match edge {
                        Edge::NC(node, cost) => Some((node, cost)),
                        _ => panic!("Must use NodeType::NC")
                    })
                    .for_each(|(edge, cost)| {
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
                            queue.push(Step(edge, edge_cost));
                        }
                    })
            }
        }
    }

    println!("Cannot find a path !!");
    None
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
        assert_eq!(path.unwrap().1, 24);
    }
    #[test]
    fn small_graph() {
        let edge_list = include!("../../../../CSX0003RUST/src/graphs/small_graph.in");
        let g = Graph::from_edge_list(&edge_list);

        let path = shortest_path(&g, 2, 5);
        assert!(path.is_some());
        assert_eq!(path.unwrap().1, 4);
    }
}
use rand::Rng;
use std::collections::{HashMap, HashSet, VecDeque};

const INF: f64 = f64::INFINITY;

type NodeT = i32;

struct UpdateQuery {
    node_source: NodeT,
    node_to_update: NodeT,
    shortest_distances: HashMap<NodeT, f64>,
}

struct RipSimulator {
    nodes: HashSet<NodeT>,
    known_info: HashMap<(NodeT, NodeT), (f64, NodeT)>,
    update_queue: VecDeque<UpdateQuery>,
    hop_count: i32,
}

impl RipSimulator {
    fn process_one_step(&mut self, edges: HashMap<(NodeT, NodeT), f64>) {
        self.hop_count += 1;
        let top = self.update_queue.pop_back().unwrap();
        let mut updated = false;
        for nbr in &self.nodes {
            if *nbr == top.node_to_update {
                continue;
            }
            let known_distance = self.known_info[&(top.node_to_update, *nbr)].0;
            let mut distance_via_src = edges[&(top.node_to_update, top.node_source)];
            if *nbr != top.node_source {
                distance_via_src += top.shortest_distances[nbr];
            }
            if distance_via_src < known_distance {
                updated = true;
                self.known_info.insert(
                    (top.node_to_update, *nbr),
                    (distance_via_src, top.node_source),
                );
            } else if distance_via_src > known_distance
                && self.known_info[&(top.node_to_update, *nbr)].1 == top.node_source
            {
                // the distance got bigger than we thought
                updated = true;
                if distance_via_src == INF {
                    println!("Here!");
                }
                self.known_info.insert(
                    (top.node_to_update, *nbr),
                    (distance_via_src, top.node_source),
                );
            }
        }
        if updated {
            self.broadcast_update_in_node(top.node_to_update, edges)
        }
    }

    fn broadcast_update_in_node(&mut self, node: NodeT, edges: HashMap<(NodeT, NodeT), f64>) {
        let mut shortest_distances = HashMap::<NodeT, f64>::new();
        // create update query
        for nbr in &self.nodes {
            if *nbr != node {
                shortest_distances.insert(*nbr, INF);
            }
        }

        for ((nd, nbr), _) in &edges {
            if *nd == node {
                shortest_distances.insert(*nbr, self.known_info[&(node, *nbr)].0);
            }
        }
        // let mut new_queries = vec![];
        for (nd, nbr) in edges.keys() {
            if *nd == node {
                self.update_queue.push_back(UpdateQuery {
                    node_source: node,
                    node_to_update: *nbr,
                    shortest_distances: shortest_distances.clone(),
                })
            }
        }
    }

    fn init_known_info(&mut self, edges: HashMap<(NodeT, NodeT), f64>) {
        for ((node, nbr), weight) in &edges {
            self.known_info.insert((*node, *nbr), (*weight, *nbr));
        }
    }

    fn print_state(&mut self) {
        println!("After hop {}", self.hop_count);
        println!(
            "{:<8} {:<8} {:<8} {:.8}",
            "src ip", "dst ip", "metric", "next_hop"
        );
        for ((node, nbr), way_info) in &self.known_info {
            println!("{:<8} {:<8} {:.8} {:.8}", node, nbr, way_info.0, way_info.1)
        }
    }
}

const K: i32 = 7;

fn main() {
    let mut nodes = HashSet::<NodeT>::new();
    for i in 0..K {
        nodes.insert(i);
    }
    let mut edges = HashMap::<(NodeT, NodeT), f64>::new();
    for node in &nodes {
        for another_node in &nodes {
            if *node != *another_node {
                edges.insert((*node, *another_node), INF);
            }
        }
    }
    let mut rng = rand::thread_rng();
    for fst in 0..K {
        for snd in 0..K {
            if fst == snd {
                continue;
            }
            let weight = rng.gen_range(0.3..1.2);
            edges.insert((fst, snd), weight);
        }
    }
    // fill known info
    let mut sim = RipSimulator {
        nodes,
        known_info: Default::default(),
        update_queue: Default::default(),
        hop_count: 0,
    };

    sim.init_known_info(edges.clone());

    for node in sim.nodes.clone() {
        sim.broadcast_update_in_node(node, edges.clone());
    }

    loop {
        if sim.update_queue.is_empty() {
            break;
        }
        sim.process_one_step(edges.clone());
        sim.print_state();
    }
    sim.print_state();
}

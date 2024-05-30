use std::collections::{HashMap, HashSet, VecDeque};

const INF: f64 = f64::INFINITY;

type NodeT = i32;
struct UpdateQuery {
    node_source: NodeT,
    node_to_update: NodeT,
    shortest_distances: HashMap<NodeT, f64>,
}

fn main() {
    let mut known_info = HashMap::<(NodeT, NodeT), (f64, NodeT)>::new();
    let mut nodes = HashSet::<NodeT>::new();
    nodes.insert(0);
    nodes.insert(1);
    nodes.insert(2);
    nodes.insert(3);
    let mut edges = HashMap::<(NodeT, NodeT), f64>::new();
    for node in &nodes {
        for another_node in &nodes {
            if *node != *another_node {
                edges.insert((*node, *another_node), INF);
                // edges[node].insert(*another_node, INF);
            }
        }
    }
    // fill edges; assert that no infinity edges here
    edges.insert((0, 1), 1.0);
    edges.insert((1, 0), 1.0);
    edges.insert((1, 2), 1.0);
    edges.insert((2, 1), 1.0);
    edges.insert((0, 2), 3.0);
    edges.insert((2, 0), 3.0);
    edges.insert((2, 3), 2.0);
    edges.insert((3, 2), 2.0);
    edges.insert((0, 3), 7.0);
    edges.insert((3, 0), 7.0);
    // fill known info
    for ((node, nbr), weight) in &edges {
        known_info.insert((*node, *nbr), (*weight, *nbr));
    }
    let mut update_queries = VecDeque::<UpdateQuery>::new();

    let propagate_update_in_node =
        |node: NodeT, known_info: HashMap<(NodeT, NodeT), (f64, NodeT)>, edges:  HashMap<(NodeT, NodeT), f64>| {
            let mut shortest_distances = HashMap::<NodeT, f64>::new();
            // create update query
            for nbr in &nodes {
                if *nbr != node {
                    shortest_distances.insert(*nbr, INF);
                }
            }

            for ((nd, nbr), _) in &edges {
                if *nd == node {
                    shortest_distances.insert(*nbr, known_info[&(node, *nbr)].0);
                }
            }
            let mut new_queries = vec![];
            for (nd, nbr) in edges.keys() {
                if *nd == node {
                    new_queries.push(UpdateQuery {
                        node_source: node,
                        node_to_update: *nbr,
                        shortest_distances: shortest_distances.clone(),
                    })
                }
            }
            new_queries
        };

    for node in &nodes {
        let new_queries = propagate_update_in_node(*node, known_info.clone(), edges.clone());
        for query in new_queries {
            update_queries.push_back(query);
        }
    }
    loop {
        if update_queries.is_empty() {
            break;
        }
        let top = update_queries.pop_back().unwrap();
        let mut updated = false;
        for nbr in &nodes {
            if *nbr == top.node_to_update {
                continue;
            }
            let known_distance = known_info[&(top.node_to_update, *nbr)].0;
            let mut distance_via_src = edges[&(top.node_to_update, top.node_source)];
            if *nbr != top.node_source {
                distance_via_src += top.shortest_distances[nbr];
            }
            if distance_via_src < known_distance {
                updated = true;
                known_info.insert(
                    (top.node_to_update, *nbr),
                    (distance_via_src, top.node_source),
                );
            } else if distance_via_src > known_distance
                && known_info[&(top.node_to_update, *nbr)].1 == top.node_source
            {
                // the distance got bigger than we thought
                updated = true;
                known_info.insert(
                    (top.node_to_update, *nbr),
                    (distance_via_src, top.node_source),
                );
            }
        }
        if updated {
            let new_queries = propagate_update_in_node(top.node_to_update, known_info.clone(), edges.clone());
            for query in new_queries {
                update_queries.push_back(query);
            }
        }
    }
    for ((node, nbr), way_info) in &known_info {
        println!("{} -[w={}]-> {} via {}", node, way_info.0, nbr, way_info.1)
    }

    println!("Now update the edge of (0, 3) to 0");
    edges.insert((0, 3), 0.0);
    edges.insert((3, 0), 0.0);
    let new_queries = propagate_update_in_node(0, known_info.clone(), edges.clone());
    for query in new_queries {
        update_queries.push_back(query);
    }
    let new_queries = propagate_update_in_node(3, known_info.clone(), edges.clone());
    for query in new_queries {
        update_queries.push_back(query);
    }

    // oh no copy paste :(
    loop {
        if update_queries.is_empty() {
            break;
        }
        let top = update_queries.pop_back().unwrap();
        let mut updated = false;
        for nbr in &nodes {
            if *nbr == top.node_to_update {
                continue;
            }
            let known_distance = known_info[&(top.node_to_update, *nbr)].0;
            let mut distance_via_src = edges[&(top.node_to_update, top.node_source)];
            if *nbr != top.node_source {
                distance_via_src += top.shortest_distances[nbr];
            }
            if distance_via_src < known_distance {
                updated = true;
                known_info.insert(
                    (top.node_to_update, *nbr),
                    (distance_via_src, top.node_source),
                );
            } else if distance_via_src > known_distance
                && known_info[&(top.node_to_update, *nbr)].1 == top.node_source
            {
                // the distance got bigger than we thought
                updated = true;
                known_info.insert(
                    (top.node_to_update, *nbr),
                    (distance_via_src, top.node_source),
                );
            }
        }
        if updated {
            let new_queries = propagate_update_in_node(top.node_to_update, known_info.clone(), edges.clone());
            for query in new_queries {
                update_queries.push_back(query);
            }
        }
    }
    for ((node, nbr), way_info) in known_info {
        println!("{} -[w={}]-> {} via {}", node, way_info.0, nbr, way_info.1)
    }
}

// while true {
// if update_queries.is_empty() {
// break;
// }
// let top = update_queries.pop_back().unwrap();
// let edge_weight = edges[&(top.node_to_update, top.node_source)];
// let mut updated = false;
// for nbr in &nodes {
// if *nbr == top.node_to_update {
// continue;
// }
// let known_distance = known_info[&(top.node_to_update, *nbr)].0;
// let distance_via_src = edges[&(top.node_to_update, top.node_source)] + top.shortest_distances[nbr];
// if distance_via_src < known_distance {
// updated = true;
// known_info.insert((top.node_to_update, *nbr), (distance_via_src, top.node_source));
// } else if
// }
// if updated {
// let new_queries = propagate_update_in_node(top.node_to_update);
// for query in new_queries {
// update_queries.push_back(query);
// }
// }
// }

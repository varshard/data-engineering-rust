use std::collections::HashMap;
use petgraph::algo::kosaraju_scc;
use petgraph::graph::DiGraph;
use community_detection::TWITTER_USERNAMES;

fn main() {
	let mut graph = DiGraph::<&str, &str>::new();

	let mut nodes = HashMap::new();

	for window in TWITTER_USERNAMES.windows(2) {
		let user = window[0];
		let mentioned = window[1];

		// insert each user to the graph and the map of nodes for the first encounter
		let user_node = *nodes.entry(user).or_insert_with(|| graph.add_node(user));
		let mention_node = *nodes.entry(mentioned).or_insert_with(|| graph.add_node(mentioned));

		// create edges between users
		graph.add_edge(user_node, mention_node, "mention");
	}

	let scc = kosaraju_scc(&graph);
	for component in scc {
		println!("{} nodes in community discovered", component.len());
		let usernames: Vec<&str> = component
			.iter().map(|&node_index| graph[node_index]).collect();

		println!("{:?}", usernames);
	}
}

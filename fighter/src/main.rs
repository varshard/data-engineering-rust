use petgraph::graph::{UnGraph};
use std::fmt;
use petgraph::Direction;
use petgraph::visit::IntoNodeReferences;

#[derive(Debug)]
struct Pokemon {
    name: String
}

impl Pokemon {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl fmt::Display for Pokemon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn main() {
    let mut graph = UnGraph::new_undirected();
    let pikachu = graph.add_node(Pokemon::new("Pikachu"));
    let kabigon = graph.add_node(Pokemon::new("Kabigon"));
    let sengame = graph.add_node(Pokemon::new("Senigame"));
    let lizadon = graph.add_node(Pokemon::new("Lizadon"));
    let baeleave = graph.add_node(Pokemon::new("Baeleave"));
    let magmarashi = graph.add_node(Pokemon::new("Magmarashi"));

    graph.add_edge(pikachu, kabigon, 1.0);
    graph.add_edge(sengame, lizadon, 1.0);
    graph.add_edge(lizadon, baeleave, 1.0);
    graph.add_edge(sengame, pikachu, 1.0);
    graph.add_edge(magmarashi, sengame, 1.0);

    for (i, x) in graph.node_references() {
        let degree = graph.edges_directed(i, Direction::Outgoing).count() as f32;
        let closeness = 1.0 / degree;
        println!("the closeness centrality of {} is {:.2}", x.name, closeness)
    }
}

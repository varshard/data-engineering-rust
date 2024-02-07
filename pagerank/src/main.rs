struct PageRank {
    damping: f64,
    iterations: usize
}

impl PageRank {
    fn new(damping: f64, iterations: usize) -> Self {
        Self{damping, iterations}
    }

    fn rank(&self, graph: &Vec<Vec<usize>>) ->  Vec<f64> {
        let size = graph.len();

        // initialize rank for each node
        let mut ranks = vec![1.0 / (size as f64); size];

        for _ in 0..self.iterations {
            let mut new_ranks = vec![0.0; size];

            for (i, edges) in graph.iter().enumerate() {
                let contribution = ranks[i] / (edges.len() as f64);
                // update weight by incoming links
                for &edge in edges {
                    new_ranks[edge] += contribution;
                }
            }

            for rank in &mut new_ranks {
                *rank = *rank * self.damping + (1.0 - self.damping) / (size as f64);
            }
            ranks = new_ranks;

            // ranks = new_ranks.into_iter().map(|rank| {
            //     rank * self.damping + (1.0 - self.damping) / (size as f64)
            // }).collect();
        }

        ranks
    }
}

fn main() {
    let sites = vec!["ESPN", "NFL", "NBA", "UFC", "MLB"];
    let graph = vec![
        vec![1,2], // ESPN to NFL, NBA
        vec![0], // NFL to ESPN
        vec![0,3],
        vec![0],
        vec![0, 1],
    ];

    let pagerank = PageRank::new(0.85, 100);
    let ranks = pagerank.rank(&graph);

    let mut sum = 0.0;
    for (i, rank) in ranks.iter().enumerate() {
        sum += rank;
        println!("pagerank of {} is {}", sites[i], rank);
    }

    println!("total: {}", sum);
}

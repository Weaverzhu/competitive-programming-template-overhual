pub mod dijkstra;
pub mod topo;

struct Graph {
    g: Vec<Vec<usize>>,
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph { g: vec![vec![]; n] }
    }

    fn add(&mut self, u: usize, v: usize) {
        self.g[u].push(v);
    }
}

type T = i64;
struct WeightedGraph {
    g: Vec<Vec<(usize, T)>>,
}

impl WeightedGraph {
    fn new(n: usize) -> Self {
        WeightedGraph { g: vec![vec![]; n] }
    }

    fn init_edges(&mut self, e: &Vec<(usize, usize, T, bool)>) {
        let g = &mut self.g;
        for &(u, v, w, undirected) in e.iter() {
            g[u].push((v, w));
            if undirected {
                g[v].push((u, w));
            }
        }
    }
}

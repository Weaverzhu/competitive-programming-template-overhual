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

struct Dijkstra {
    g: Rc<RefCell<WeightedGraph>>,
}

impl Dijkstra {
    fn new(g: Rc<RefCell<WeightedGraph>>) -> Dijkstra {
        Dijkstra { g: g }
    }

    fn calc_from(&self, st: usize, def: T) -> Vec<T> {
        let g = &self.g.borrow().g;
        let n = g.len();
        let mut dist = vec![def; n];
        dist[st] = 0;
        let mut q = BinaryHeap::new();
        q.push((0 as T, st));
        while let Some(u) = q.pop() {
            let dis = -u.0;
            let u = u.1;
            if dist[u] < dis {
                continue;
            }

            for &(v, w) in g[u].iter() {
                if dis + w < dist[v] {
                    dist[v] = dis + w;
                    q.push((-dist[v], v));
                }
            }
        }
        dist
    }
}

use std::collections::*;

struct Graph {
    g: Vec<Vec<usize>>,
    init: Vec<usize>,
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            g: vec![vec![]; n],
            init: vec![],
        }
    }

    fn add(&mut self, u: usize, v: usize) {
        // println!("add {} {}", u, v);
        self.g[u].push(v);
    }

    fn topo(&mut self) -> Vec<usize> {
        let n = self.g.len();
        let mut deg = vec![0; n];
        let mut ret = vec![];
        let g = &self.g;
        for i in 0..n {
            for &v in g[i].iter() {
                deg[v] += 1;
            }
        }

        let mut q = VecDeque::new();
        let init = &mut self.init;
        for i in 0..n {
            if deg[i] == 0 {
                q.push_back(i);
                init.push(i);
            }
        }
        while let Some(u) = q.pop_front() {
            ret.push(u);
            for &v in g[u].iter() {
                deg[v] -= 1;
                if deg[v] == 0 {
                    q.push_back(v);
                }
            }
        }

        ret
    }
}

use super::Graph;
use std::collections::*;

struct Topo<'a> {
    g: &'a Graph,
}

impl<'a> Topo<'a> {
    fn new(g: &'a Graph) -> Self {
        Topo { g: g }
    }

    fn order(&self) -> Vec<usize> {
        let g = &self.g.g;
        let n = g.len();
        let mut ret = vec![];

        let mut degree = vec![0; n + 1];
        for u in 0..n {
            for &v in g[u].iter() {
                degree[v] += 1;
            }
        }
        let mut q = VecDeque::new();
        for u in 0..n {
            if degree[u] == 0 {
                q.push_back(u);
            }
        }

        while let Some(u) = q.pop_front() {
            ret.push(u);
            for &v in g[u].iter() {
                degree[v] -= 1;
                if degree[v] == 0 {
                    q.push_back(v);
                }
            }
        }

        ret
    }
}

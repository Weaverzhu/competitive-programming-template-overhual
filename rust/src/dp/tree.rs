struct TreeDp {
    g: Vec<Vec<usize>>,
    score: Vec<u32>,
    dp: Vec<u64>,
    ans: Vec<u64>,
    g2: Vec<Vec<usize>>,
    result: u64,
}

impl TreeDp {
    fn new(n: usize) -> Self {
        TreeDp {
            g: vec![vec![]; n],
            dp: vec![0; n],
            score: vec![0; n],
            ans: vec![0; n],
            g2: vec![vec![]; n],
            result: 0,
        }
    }

    // p_i means i's parent is p_i
    fn new_with_parent(p: &Vec<usize>) -> Self {
        let n = p.len();
        let mut r = TreeDp::new(n);
        for i in 1..n {
            r.add(p[i], i);
        }
        r
    }

    fn add(&mut self, u: usize, v: usize) {
        self.g[u].push(v);
    }

    fn set_score(&mut self, s: &Vec<u32>) {
        self.score = s.clone();
    }

    fn dfs(&mut self, u: usize, k: usize) {
        self.dp[u] = self.score[u] as u64;
        self.ans[u] = k as u64;
        self.result += self.dp[u] * (k as u64);
        let mut g = self.g[u].clone();
        if g.len() == 0 {
            // println!("{} {} {} {}", u, k, self.result, self.dp[u]);
            return;
        }
        let base = k / g.len();
        for &v in g.iter() {
            self.dfs(v, base);
        }

        g.sort_by(|a, b| self.dp[*b].cmp(&self.dp[*a]));

        let lim = k % g.len();
        for i in 0..lim {
            self.result += self.dp[g[i]];
        }
        self.dp[u] += self.dp[g[lim]];
        self.g2[u] = g;
        // println!("{} {} {} {}", u, k, self.result, self.dp[u]);
    }
}

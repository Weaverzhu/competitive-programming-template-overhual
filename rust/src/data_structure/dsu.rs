pub struct DSU {
    pub n: usize,
    f: Vec<usize>,
}

impl DSU {
    pub fn new(n: usize) -> DSU {
        let mut d = DSU {
            n: n,
            f: vec![0; n + 1],
        };

        for i in 0..n + 1 {
            d.f[i] = i;
        }
        d
    }

    pub fn find(&mut self, x: usize) -> usize {
        if x != self.f[x] {
            self.f[x] = self.find(self.f[x]);
        }
        self.f[x]
    }

    pub fn connect(&mut self, mut u: usize, mut v: usize) {
        u = self.find(u);
        v = self.find(v);
        self.f[u] = v;
    }

    pub fn test(&mut self, u: usize, v: usize) -> bool {
        return self.find(u) == self.find(v);
    }
}

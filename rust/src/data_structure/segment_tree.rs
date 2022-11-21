struct SegmentTree<T: Ord + Copy> {
    data: Vec<(T, usize)>,
    n: usize,
    nn: usize,
    def: T,
    ll: usize,
    rr: usize,
}

impl<T: Ord + Copy> SegmentTree<T> {
    fn cmp<'a>(lhs: &'a T, rhs: &'a T) -> &'a T {
        std::cmp::max(lhs, rhs)
    }

    fn new(a: &Vec<T>, def: T) -> Self {
        let n = a.len();
        let mut r = SegmentTree {
            data: vec![],
            n: n,
            nn: 1,
            def: def,
            ll: 0,
            rr: 0,
        };

        let nn = &mut r.nn;
        let data = &mut r.data;

        *nn = 1;
        while *nn < n {
            *nn *= 2;
        }

        let nn = *nn;
        *data = vec![(def, n); 2 * nn - 1];

        for i in 0..n {
            data[i + nn - 1] = (a[i], i);
        }

        for i in (0..(nn - 1)).rev() {
            let ls = i * 2 + 1;
            let rs = ls + 1;
            data[i] = *SegmentTree::cmp(&data[ls], &data[rs]);
        }
        r
    }

    fn u(&mut self, mut i: usize, x: &T) {
        i += self.nn - 1;
        self.data[i] = (*x, i);
        while i > 0 {
            let fa = (i - 1) / 2;
            self.data[fa] = *SegmentTree::cmp(&self.data[fa * 2 + 1], &self.data[fa * 2 + 2]);
            i = fa;
        }
    }

    fn qq(&mut self, l: usize, r: usize, rt: usize) -> (T, usize) {
        if self.ll <= l && r <= self.rr {
            return self.data[rt];
        }
        let m = (l + r) / 2;
        let mut ret = (self.def, self.n);
        if self.ll <= m {
            ret = *SegmentTree::cmp(&self.qq(l, m, rt * 2 + 1), &ret);
        }
        if m + 1 <= self.rr {
            ret = *SegmentTree::cmp(&self.qq(m + 1, r, rt * 2 + 2), &ret);
        }
        ret
    }

    fn q(&mut self, l: usize, r: usize) -> (T, usize) {
        self.ll = l;
        self.rr = r;
        return self.qq(1, self.nn, 0);
    }
}

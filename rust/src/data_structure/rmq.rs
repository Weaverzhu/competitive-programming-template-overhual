struct RMQ<T: Ord + Copy> {
    data: Vec<Vec<(T, usize)>>,
    pow2: Vec<usize>,
    len2: Vec<usize>,
}

impl<T: Ord + Copy + std::fmt::Debug> RMQ<T> {
    fn cmp(lhs: (T, usize), rhs: (T, usize)) -> (T, usize) {
        if lhs.0 != rhs.0 {
            std::cmp::max(lhs, rhs)
        } else {
            std::cmp::min(lhs, rhs)
        }
    }

    fn new(a: &Vec<T>) -> Self {
        let n = a.len();
        let mut r = RMQ {
            data: vec![vec![(a[0], 0); n]],
            len2: vec![0; n],
            pow2: vec![],
        };

        let data = &mut r.data;
        let len2 = &mut r.len2;
        let pow2 = &mut r.pow2;

        pow2.push(1);
        while *pow2.last().unwrap() < n {
            pow2.push(pow2.last().unwrap() * 2);
        }
        len2[1] = 0;
        for i in 2..n {
            len2[i] = len2[i / 2] + 1;
        }

        for i in 0..n {
            data[0][i] = (a[i], i);
        }

        for st in 1..pow2.len() {
            data.push(vec![(a[0], n); n]);
            for i in 0..(n - pow2[st - 1]) {
                data[st][i] = RMQ::cmp(data[st - 1][i], data[st - 1][i + pow2[st - 1]]);
            }
        }
        return r;
    }

    fn q(&self, l: usize, r: usize) -> Option<(T, usize)> {
        if l > r {
            return None;
        }
        let l2 = self.len2[r - l + 1];
        let data = &self.data;
        let l = &data[l2][l];
        let r = &data[l2][r + 1 - self.pow2[l2]];

        Some(RMQ::cmp(*l, *r))
    }
}

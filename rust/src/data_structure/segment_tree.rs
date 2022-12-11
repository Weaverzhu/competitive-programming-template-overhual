pub struct SegmentTreePURQ<T, F> {
    n: usize,
    size: usize,
    data: Vec<T>,
    e: T,
    op: F,
}

impl<T, F> SegmentTreePURQ<T, F>
where
    T: Clone,
    F: Fn(&T, &T) -> T,
{
    pub fn new(n: usize, e: T, op: F) -> Self {
        assert!(n > 0);
        let size = n.next_power_of_two();
        let data = vec![e.clone(); 2 * size];
        SegmentTreePURQ {
            n,
            size,
            data,
            e,
            op,
        }
    }
    pub fn update_tmp(&mut self, x: usize, v: T) {
        assert!(x < self.n);
        self.data[x + self.size] = v;
    }
    pub fn update_all(&mut self) {
        for i in (1..self.size).rev() {
            self.data[i] = (self.op)(&self.data[2 * i], &self.data[2 * i + 1]);
        }
    }
    pub fn update(&mut self, x: usize, v: T) {
        assert!(x < self.n);
        let mut x = x + self.size;
        self.data[x] = v;
        x >>= 1;
        while x > 0 {
            self.data[x] = (self.op)(&self.data[2 * x], &self.data[2 * x + 1]);
            x >>= 1;
        }
    }
    pub fn find(&self, l: usize, r: usize) -> T {
        assert!(l <= r && r <= self.n);
        if l == r {
            return self.e.clone();
        }
        let mut l = self.size + l;
        let mut r = self.size + r;
        let mut x = self.e.clone();
        let mut y = self.e.clone();
        while l < r {
            if l & 1 == 1 {
                x = (self.op)(&x, &self.data[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                y = (self.op)(&self.data[r], &y);
            }
            l >>= 1;
            r >>= 1;
        }
        (self.op)(&x, &y)
    }
    pub fn max_right<P>(&self, l: usize, f: P) -> usize
    where
        P: Fn(&T) -> bool,
    {
        assert!(l <= self.n);
        assert!(f(&self.e));
        if l == self.n {
            return self.n;
        }
        let mut l = l + self.size;
        let mut sum = self.e.clone();
        while {
            l >>= l.trailing_zeros();
            let v = (self.op)(&sum, &self.data[l]);
            if !f(&v) {
                while l < self.size {
                    l <<= 1;
                    let v = (self.op)(&sum, &self.data[l]);
                    if f(&v) {
                        sum = v;
                        l += 1;
                    }
                }
                return l - self.size;
            }
            sum = v;
            l += 1;
            l.count_ones() > 1
        } {}
        self.n
    }
    pub fn min_left<P>(&self, r: usize, f: P) -> usize
    where
        P: Fn(&T) -> bool,
    {
        assert!(r <= self.n);
        assert!(f(&self.e));
        if r == 0 {
            return 0;
        }
        let mut r = r + self.size;
        let mut sum = self.e.clone();
        while {
            r -= 1;
            while r > 1 && r & 1 == 1 {
                r >>= 1;
            }
            let v = (self.op)(&self.data[r], &sum);
            if !f(&v) {
                while r < self.size {
                    r = 2 * r + 1;
                    let v = (self.op)(&self.data[r], &sum);
                    if f(&v) {
                        sum = v;
                        r -= 1;
                    }
                }
                return r + 1 - self.size;
            }
            sum = v;
            (r & (!r + 1)) != r
        } {}
        0
    }
}

use std::collections::*;

pub struct Prime {
    prime: Vec<usize>,
    minp: Vec<usize>,
}

impl Prime {
    fn new(n: usize) -> Self {
        let mut r = Prime {
            prime: vec![],
            minp: vec![0; n],
        };
        let minp = &mut r.minp;
        let prime = &mut r.prime;

        for i in 2..n {
            if minp[i] == 0 {
                minp[i] = i;
                prime.push(i);
            }
            for &p in prime.iter() {
                let np = p as u64 * i as u64;
                if np >= n as u64 {
                    break;
                }
                if minp[np as usize] == 0 {
                    minp[np as usize] = p;
                }
                if i % p == 0 {
                    break;
                }
            }
        }
        r
    }

    fn pfac_slow(&self, mut n: usize) -> HashMap<usize, usize> {
        let mut r: HashMap<usize, usize> = HashMap::new();
        for &p in self.prime.iter() {
            if n % p == 0 {
                let it = r.entry(p).or_insert(0);
                let mut cnt = 0;
                while n % p == 0 {
                    n /= p;
                    cnt += 1;
                }
                // println!("{} {} {}", n, p, cnt);
                *it = cnt;
            }
            if n < p {
                assert!(n == 1);
                break;
            }
        }
        if n > 1 {
            *r.entry(n).or_insert(0) = 1;
        }
        r
    }

    fn pfac(&self, mut n: usize) -> HashMap<usize, usize> {
        let len64 = self.minp.len() as u64;
        assert!((n as u64) < (len64 * len64));
        if n >= self.minp.len() {
            return self.pfac_slow(n);
        }
        let mut r: HashMap<usize, usize> = HashMap::new();
        while n > 1 {
            let p = self.minp[n];
            let mut cnt = 0;
            while n % p == 0 {
                cnt += 1;
                n /= p;
            }
            *r.entry(p).or_insert(0) = cnt;
        }
        r
    }
}

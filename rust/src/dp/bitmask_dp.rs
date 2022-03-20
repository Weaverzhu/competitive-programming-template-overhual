use i64 as T;
pub struct BitMaskDP {
    n: usize,
    sp: usize,
    dp: Vec<T>,
}

impl Clone for BitMaskDP {
    fn clone(&self) -> BitMaskDP {
        return BitMaskDP {
            n: self.n,
            sp: self.sp,
            dp: self.dp.to_vec(),
        };
    }
}

impl BitMaskDP {
    pub fn new(n: usize) -> BitMaskDP {
        let ret = BitMaskDP {
            n: 1 << n,
            sp: n,
            dp: vec![0 as T; 1 << n],
        };

        return ret;
    }

    pub fn zeta(&mut self) {
        let n = self.n;
        let dp = &mut self.dp;
        let sp = self.sp;

        for i in 0..sp {
            for mask in 0..n {
                if mask >> i & 1 == 1 {
                    dp[mask] += dp[mask ^ (1 << i)];
                }
            }
        }
    }

    pub fn odd_neg(&mut self) {
        let n = self.n;
        let dp = &mut self.dp;
        for i in 0..n {
            if i.count_ones() & 1 == 1 {
                dp[i] = -dp[i];
            }
        }
    }

    pub fn mu(&mut self) {
        self.odd_neg();
        self.zeta();
        self.odd_neg();
    }

    pub fn inv(&mut self) {
        self.mu();
    }

    pub fn subset_conv(&self, og: &BitMaskDP) -> BitMaskDP {
        let n = self.n;
        let sp = self.sp;
        let mut f = vec![BitMaskDP::new(sp); sp + 1];
        let mut g = vec![BitMaskDP::new(sp); sp + 1];
        let mut h = vec![BitMaskDP::new(sp); sp + 1];
        for mask in 0..n {
            let co = mask.count_ones() as usize;
            f[co].dp[mask] = self.dp[mask];
            g[co].dp[mask] = og.dp[mask];
        }

        for i in 0..sp + 1 {
            f[i].zeta();
            g[i].zeta();
        }

        let mut ret = BitMaskDP::new(sp);
        for mask in 0..n {
            for i in 0..sp + 1 {
                for j in 0..i + 1 {
                    h[i].dp[mask] += f[j].dp[mask] * g[i - j].dp[mask];
                }
            }
        }
        for i in 0..sp + 1 {
            h[i].inv();
        }

        for mask in 0..n {
            let idx = mask.count_ones() as usize;
            ret.dp[mask] = h[idx].dp[mask];
        }
        return ret;
    }
}

#[cfg(test)]
mod tests {
    use super::BitMaskDP;
    use rand::prelude::*;
    use rand_chacha::ChaCha20Rng;

    fn randomize(f: &mut BitMaskDP, rng: &mut ChaCha20Rng) {
        for x in f.dp.iter_mut() {
            *x = rng.gen_range(1..10000);
        }
    }

    #[test]
    fn subset_conv_test() {
        let mut rng = ChaCha20Rng::from_entropy();

        let sp = 10;
        let n = 1 << sp;
        let mut f = BitMaskDP::new(sp);
        let mut g = BitMaskDP::new(sp);
        randomize(&mut f, &mut rng);
        randomize(&mut g, &mut rng);

        let mut credit_h = BitMaskDP::new(sp);
        let f = &f;
        let g = &g;
        for mask in 0..n {
            for mf in 0..n {
                if mask & mf == mf {
                    let mg = mask ^ mf;
                    credit_h.dp[mask] += f.dp[mf] * g.dp[mg];
                }
            }
        }

        let h = f.subset_conv(&g);

        for mask in 0..n {
            assert_eq!(
                h.dp[mask], credit_h.dp[mask],
                "mask={}, dp={}, expected={}",
                mask, h.dp[mask], credit_h.dp[mask]
            );
        }
    }

    #[test]
    fn zeta_transform_random_test() {
        let sp = 10;
        let mut x = BitMaskDP::new(sp);

        let mut rng = ChaCha20Rng::from_entropy();
        randomize(&mut x, &mut rng);
        let mut credit = BitMaskDP::new(sp);
        let n = 1 << sp;
        for mask in 0..n {
            for submask in 0..n {
                if mask & submask == submask {
                    credit.dp[mask] += x.dp[submask];
                }
            }
        }

        x.zeta();
        for mask in 0..n {
            assert_eq!(
                x.dp[mask], credit.dp[mask],
                "x={}, expected={}",
                x.dp[mask], credit.dp[mask]
            );
        }
    }

    #[test]
    fn sos_dp_simple_test() {
        let mut x = BitMaskDP::new(10);
        assert_eq!(x.dp.len(), 1024);

        for i in 0..10 {
            x.dp[1 << i] = 1 << i;
        }
        x.zeta();
        for i in 0..1024 {
            assert_eq!(x.dp[i], i as i64, "i={}, dp={}", i, x.dp[i]);
        }
    }
}

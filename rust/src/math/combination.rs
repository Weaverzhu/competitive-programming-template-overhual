const MOD: u64 = 1_000_000_007;
pub struct PreCalc {
    f: Vec<u64>,
    invf: Vec<u64>,
}

use crate::misc::misc::bin;

impl PreCalc {
    fn new(n: usize) -> Self {
        let mut r = PreCalc {
            f: vec![1; n],
            invf: vec![1; n],
        };
        let f = &mut r.f;
        let invf = &mut r.invf;
        for i in 1..n {
            f[i] = f[i - 1] * (i as u64) % MOD;
        }
        invf[n - 1] = bin(f[n - 1], MOD - 2, MOD);
        for i in (2..(n - 1)).rev() {
            invf[i] = invf[i + 1] * (i as u64 + 1) % MOD;
        }
        r
    }

    fn binom(&self, n: usize, k: usize) -> u64 {
        if n < k {
            return 0;
        }
        return self.f[n] * self.invf[k] % MOD * self.invf[n - k] % MOD;
    }
}

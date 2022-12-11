pub fn bin(mut a: u64, mut b: u64, p: u64) -> u64 {
    let mut r = 1;
    while b > 0 {
        if b & 1 == 1 {
            r = r * a % p;
        }
        a = a * a % p;
        b /= 2;
    }
    r
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b > 0 {
        let c = a % b;
        a = b;
        b = c;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    return a / gcd(a, b) * b;
}

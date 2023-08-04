pub fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();
    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }

    let mut d: u64 = 3;
    while d * d <= n {
        if n % d == 0 {
            factors.push(d);
            n /= d;
        } else {
            d += 2;
        }
    }

    if n > 1 {
        factors.push(n);
    }

    factors
}

use super::*;

const P: u64 = 7;

#[snippet]
fn solo_prime_factorize(mut n: i64) -> Vec<(i64, u32)> {
    let m = n;
    let mut res = Vec::with_capacity(65);
    let mut p = 2;
    let mut e = 0u32;
    while p * p <= m {
        if n % p == 0 {
            e += 1;
            n /= p;
        } else if e > 0 {
            res.push((p, e));
            e = 0;
            p += 1;
        } else { p += 1; }
    }
    if n > 1 { res.push((n, 1u32)); }
    res
}

// エラトステネスのふるいを前処理で作ることで個別のクエリを効率よく処理できる
// osa_k法?
#[snippet]
fn multi_prime_factorize(_n: i64) -> Vec<(i64, usize)> {
    unimplemented!();
}

#[snippet]
fn gcd(x: i64, y: i64) -> i64 {
    if y == 0 { x }
    else { gcd(y, x % y) }
}

#[snippet]
fn lcm(x: i64, y: i64) -> i64 {
    x / gcd(x, y) * y
}

#[snippet]
fn sieve(n: i64) -> Vec<bool> {
    let mut is_prime = vec![true; n as usize];
    is_prime[0] = false;
    is_prime[1] = false;
    if n < 2 { return is_prime }
    let mut p = 2;
    while p*p <= n-1 {
        if !is_prime[p as usize] {
            p += 1;
            continue;
        }
        let mut pos = p*p;
        for _ in 0..=((n-1 - pos)/p) {
            is_prime[pos as usize] = false;
            pos += p;
        }
        p += 1;
    }
    is_prime
}

#[snippet]
fn mod_pow(r: u64, mut n: u64) -> u64 {
    let mut t = 1;
    let mut s = r % P;
    while n > 0 {
        if n & 1 == 1 {
            t = t * s % P;
        }
        s = s * s % P;
        n >>= 1;
    }
    t
}

#[snippet]
fn comb(n: u64, k: u64) -> u64 {
    let k = k.min(n - k);
    let mut nu = 1;
    let mut de = 1;
    for i in 0..k {
        nu = nu * (n - i) % P;
        de = de * (i + 1) % P;
    }
    nu * mod_pow(de, P - 2) % P
}

// 区間r(両端含む)に含まれるnの倍数の個数を求める
// todo: a < 0 のときでも大丈夫かどうか調べる
#[snippet]
fn multiples(r: std::ops::RangeInclusive<i64>, n: i64) -> i64 {
    *r.end() / n - ((((*r.start() + n - 1))/ n) - 1)
}

mod tests {
    use super::*;

    #[test]
    fn test_mod_pow() {
        assert_eq!(mod_pow(2, 2), 4);
        assert_eq!(mod_pow(3, 2), 2);
        assert_eq!(mod_pow(7, 10101010), 0);
    }
}

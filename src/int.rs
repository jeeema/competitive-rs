type SomeInt = i32;
type PositiveInt = usize;

fn gcd(x: SomeInt, y: SomeInt) -> SomeInt {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn lcm(x: SomeInt, y: SomeInt) -> SomeInt {
    x * y / gcd(x, y)
}

// 区間r(両端含む)に含まれるnの倍数の個数を求める
use std::ops::RangeInclusive;
fn multiples(r: RangeInclusive<SomeInt>, n: SomeInt) -> SomeInt {
    *r.end() / n - ((((*r.start() + n - 1))/ n) - 1)
}

// 10進数を各桁の列にする
fn each_digit10(mut n: SomeInt, k: u8) -> Vec<SomeInt> {
    let mut res: Vec<SomeInt> = vec![];
    for _ in 0..k {
        res.push(n % 10);
        n /= 10;
    }
    res
}

// 平方数か判定
fn is_square(n: PositiveInt) -> bool {
    n == (n.sqrt().floor() as PositiveInt).pow(2)
}

struct Facto(Vec<PositiveInt>);

impl Facto {
    fn facto(n: PositiveInt) -> PositiveInt {
        if n == 0 { 1 }
        else { (1..(n + 1)).product() }
    }

    fn new(n: PositiveInt) -> Facto {
        Facto(
            (0..(n + 1)).scan(1, |state, i| {
                if i == 0 { *state = 1; }
                else { *state *= i; }
                Some(*state)
            }).collect()
        )
    }

    fn facto_memo(&mut self, n: PositiveInt) -> PositiveInt {
        let l = self.0.len();
        (self.0).append(&mut ((l..(n + 1))
            .scan(*((self.0).last().unwrap()), |state, i| {
                *state *= i;
                Some(*state)
            })
            .collect())
        );
        self.0[n]
    }
}

#[cfg(test)]
mod tests {
}

use super::*;

#[snippet("UnionFind")]
pub struct UnionFind(Vec<isize>);

#[allow(dead_code)]
#[snippet("UnionFind")]
impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind(vec![-1; n])
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.0[x] < 0 { x }
        else {
            let root = self.find(self.0[x] as usize);
            self.0[x] = root as isize;
            root
        }
    }

    pub fn unite(&mut self, x: usize, y: usize) {
        let (px, py) = (self.find(x), self.find(y));
        if px == py { return; }
        if self.0[px] <= self.0[py] {
            self.0[px] += self.0[py];
            self.0[py] = px as isize;
        } else {
            self.0[py] += self.0[px];
            self.0[px] = py as isize;
        }
    }

    pub fn size(&mut self, x: usize) -> i64 {
        let root = self.find(x) as usize;
        (-self.0[root]) as i64
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

//source:: https://github.com/tanakh/competitive-rs/blob/master/src/union_find.rs
#[test]
fn union_find_test() {
    let mut uf = UnionFind::new(5);
    uf.unite(0, 1);
    uf.unite(2, 3);
    uf.unite(0, 4);

    assert_eq!(uf.find(0), uf.find(1));
    assert_eq!(uf.find(2), uf.find(3));
    assert_eq!(uf.find(0), uf.find(4));
    assert_eq!(uf.find(1), uf.find(4));
    assert_ne!(uf.find(0), uf.find(2));
    assert_ne!(uf.find(3), uf.find(4));
}

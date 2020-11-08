use super::*;

#[snippet]
fn rle<T>(v: &[T]) -> Vec<(T, usize)>
where T: std::cmp::PartialEq + std::marker::Copy,
{
    if v.len() == 1 { return vec![(v[0], 1)] }
    let mut tups = Vec::with_capacity(v.len()+1);
    let mut acc = 1;
    for t in v.iter().copied().tuple_windows().with_position() {
        match t {
            Position::First((curr, next)) | Position::Middle((curr, next)) => {
                if curr == next { acc += 1; }
                else {
                    tups.push((curr, acc));
                    acc = 1;
                }
            },
            Position::Last((curr, next)) | Position::Only((curr, next))=> {
                if curr == next { tups.push((curr, acc + 1)); }
                else {
                    tups.push((curr, acc));
                    tups.push((next, 1));
                }
            },
        }
    }
    tups
}

mod tests {
    use super::*;

    #[test]
    fn test_for_rle() {
        let empty_vec: Vec<i32> = vec![];
        assert_eq!(rle(&empty_vec).len(), 0);

        assert_eq!(rle(&vec![0]), vec![(0, 1)]);

        assert_eq!(rle(&vec![0, 0]), vec![(0, 2)]);
        assert_eq!(rle(&vec![0, 1]), vec![(0, 1), (1, 1)]);

        assert_eq!(rle(&vec![0, 0, 1]), vec![(0, 2), (1, 1)]);
        assert_eq!(rle(&vec![0, 1, 1]), vec![(0, 1), (1, 2)]);

        assert_eq!(rle(&vec![0, 1, 1, 0]), vec![(0, 1), (1, 2), (0, 1)]);
        assert_eq!(rle(&vec![0, 1, 1, 1, 0, 0]), vec![(0, 1), (1, 3), (0, 2)]);
    }
}

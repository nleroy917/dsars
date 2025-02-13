pub fn linear_search<T>(lst: &[T], x: T) -> Option<usize>
    where T: PartialOrd + Eq
{
    for (i, v) in lst.iter().enumerate() {
        if *v == x {
            return Some(i)
        } else if *v > x {
            return None
        }
    }

    None
}

pub fn binary_search<T>(lst: &[T], x: T) -> Option<usize>
    where T: PartialOrd + Eq
{
    let mut l = 0_usize;
    let mut r = lst.len() - 1;
    let mut m: usize;

    while l <= r {
        m = (l + r) / 2;

        if lst[m] == x {
            return Some(m)
        } else if lst[m] < x {
            l = m + 1
        } else {
            r = m - 1;
        }
    }

    None

}
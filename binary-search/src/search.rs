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
    let mut lo = 0_usize;
    let mut hi = lst.len() - 1;
    let mut mid: usize;

    while lo <= hi {
        
        mid = (lo + hi) / 2;

        if lst[mid] == x {
            return Some(mid)
        } else if lst[mid] < x {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }

    None

}
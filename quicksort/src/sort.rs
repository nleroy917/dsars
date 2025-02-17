pub fn quicksort<T>(lst: &mut [T])
    where T: PartialOrd 
{
    if !lst.is_empty() {
        _quicksort(lst, 0, lst.len() - 1);
    }
}

fn _quicksort<T>(lst: &mut [T], lo: usize, hi: usize)
    where T: PartialOrd 
{
    if lo < hi {
        let p = partition(lst, lo, hi);
        _quicksort(lst, lo, p - 1);
        _quicksort(lst, p + 1, hi);
    }
}

fn partition<T>(lst: &mut [T], lo: usize, hi: usize) -> usize
    where T: PartialOrd
{
    let mut i = lo;

    for j in lo..hi {
        if lst[j] <= lst[hi] {
            lst.swap(i, j);
            i += 1;
        }
    }
    lst.swap(i, hi);
    i
}
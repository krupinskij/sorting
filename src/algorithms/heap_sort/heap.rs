pub fn create_heap_by_predicate<T, P>(data: &mut [T], predicate: &P)
where
    P: Fn(&T, &T) -> bool,
{
    let l = data.len();
    for i in (0..l / 2).rev() {
        down_heap_by_predicate(data, i, l, predicate)
    }
}

pub fn down_heap_by_predicate<T, P>(data: &mut [T], i: usize, l: usize, predicate: &P)
where
    P: Fn(&T, &T) -> bool,
{
    let mut max = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < l && !predicate(&data[left], &data[max]) {
        max = left;
    }
    if right < l && !predicate(&data[right], &data[max]) {
        max = right;
    }

    if max != i {
        data.swap(max, i);
        down_heap_by_predicate(data, max, l, predicate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_heap_by_predicate_test() {
        let mut arr = [3, 2, 4, 2, 5, 7, 1, 0, 6];
        create_heap_by_predicate(&mut arr, &|a, b| a < b);
        assert_eq!(arr, [7, 6, 4, 2, 5, 3, 1, 0, 2]);
    }
}

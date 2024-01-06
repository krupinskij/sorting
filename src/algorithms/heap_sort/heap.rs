use crate::Order;
use std::cmp::PartialOrd;

pub fn create_heap<T: PartialOrd>(data: &mut [T], order: &Order) {
    let l = data.len();
    for i in (0..l / 2).rev() {
        down_heap(data, i, l, &order)
    }
}

pub fn down_heap<T: PartialOrd>(data: &mut [T], i: usize, l: usize, order: &Order) {
    let mut max = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    match order {
        Order::Asc => {
            if left < l && data[left] > data[max] {
                max = left;
            }
            if right < l && data[right] > data[max] {
                max = right;
            }
        }
        Order::Desc => {
            if left < l && data[left] < data[max] {
                max = left;
            }
            if right < l && data[right] < data[max] {
                max = right;
            }
        }
    }

    if max != i {
        data.swap(max, i);
        down_heap(data, max, l, order)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_heap_test() {
        let mut arr = [3, 2, 4, 2, 5, 7, 1, 0, 6];
        create_heap(&mut arr, &Order::Asc);
        assert_eq!(arr, [7, 6, 4, 2, 5, 3, 1, 0, 2]);
    }
}

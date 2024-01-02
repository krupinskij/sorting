mod partition;

use self::partition::partition as exec_partition;
use crate::{Order, Partition};

pub fn sort<T: PartialOrd>(data: &mut [T], order: Order, partition: Partition) {
    sort_rec(data, 0, data.len() - 1, &order, &partition)
}

fn sort_rec<T: PartialOrd>(
    data: &mut [T],
    l: usize,
    r: usize,
    order: &Order,
    partition: &Partition,
) {
    let pivot = exec_partition(data, l, r, order, partition);
    if pivot > l + 1 {
        sort_rec(data, l, pivot - 1, order, partition);
    }
    if pivot + 1 < r {
        sort_rec(data, pivot + 1, r, order, partition);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_array_asc() {
        let mut arr = [2, 3, 1, 5, 4];
        sort(&mut arr, Order::Asc, Partition::First);
        assert_eq!([1, 2, 3, 4, 5], arr);

        let mut arr = [2, 3, 1, 5, 4];
        sort(&mut arr, Order::Asc, Partition::Last);
        assert_eq!([1, 2, 3, 4, 5], arr);

        let mut arr = [2, 3, 1, 5, 4];
        sort(&mut arr, Order::Asc, Partition::Center);
        assert_eq!([1, 2, 3, 4, 5], arr);
    }

    #[test]
    fn sort_array_desc() {
        let mut arr = [2, 3, 1, 5, 4];
        sort(&mut arr, Order::Desc, Partition::First);
        assert_eq!([5, 4, 3, 2, 1], arr);

        let mut arr = [2, 3, 1, 5, 4];
        sort(&mut arr, Order::Desc, Partition::Last);
        assert_eq!([5, 4, 3, 2, 1], arr);

        let mut arr = [2, 3, 1, 5, 4];
        sort(&mut arr, Order::Desc, Partition::Center);
        assert_eq!([5, 4, 3, 2, 1], arr);
    }
}

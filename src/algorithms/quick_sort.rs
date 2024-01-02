use crate::{Order, Partition};

fn partition<T: PartialOrd>(
    data: &mut [T],
    l: usize,
    r: usize,
    order: Order,
    partition: Partition,
) -> usize {
    let pivot = match partition {
        Partition::First => l,
        Partition::Last => r,
        Partition::Center => usize::from((l + r) / 2),
    };
    let mut i = l;

    for j in l..=r {
        if i == pivot {
            i += 1;
        }
        if j == pivot {
            continue;
        }

        match order {
            Order::Asc => {
                if data[j] < data[pivot] {
                    data.swap(i, j);
                    i += 1;
                }
            }
            Order::Desc => {
                if data[j] > data[pivot] {
                    data.swap(i, j);
                    i += 1;
                }
            }
        }
    }

    if i <= pivot {
        data.swap(i, pivot);
        return i;
    } else {
        data.swap(i - 1, pivot);
        return i - 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partition_first() {
        let mut arr = [4, 1, 3, 2, 5];
        let pivot = partition(&mut arr, 0, 4, Order::Asc, Partition::First);

        assert!(arr[0] <= 4);
        assert!(arr[1] <= 4);
        assert!(arr[2] <= 4);
        assert_eq!(arr[3], 4);
        assert_eq!(arr[4], 5);
        assert_eq!(pivot, 3);

        let mut arr = [4, 1, 3, 2, 5];
        let pivot = partition(&mut arr, 0, 4, Order::Desc, Partition::First);

        assert_eq!(arr[0], 5);
        assert_eq!(arr[1], 4);
        assert!(arr[2] <= 4);
        assert!(arr[3] <= 4);
        assert!(arr[4] <= 4);
        assert_eq!(pivot, 1);
    }

    #[test]
    fn partition_last() {
        let mut arr = [5, 1, 3, 2, 4];
        let pivot = partition(&mut arr, 0, 4, Order::Asc, Partition::Last);

        assert!(arr[0] <= 4);
        assert!(arr[1] <= 4);
        assert!(arr[2] <= 4);
        assert_eq!(arr[3], 4);
        assert_eq!(arr[4], 5);
        assert_eq!(pivot, 3);

        let mut arr = [5, 1, 3, 2, 4];
        let pivot = partition(&mut arr, 0, 4, Order::Desc, Partition::Last);

        assert_eq!(arr[0], 5);
        assert_eq!(arr[1], 4);
        assert!(arr[2] <= 4);
        assert!(arr[3] <= 4);
        assert!(arr[4] <= 4);
        assert_eq!(pivot, 1);
    }

    #[test]
    fn partition_center() {
        let mut arr = [5, 1, 4, 2, 3];
        let pivot = partition(&mut arr, 0, 4, Order::Asc, Partition::Center);

        assert!(arr[0] <= 4);
        assert!(arr[1] <= 4);
        assert!(arr[2] <= 4);
        assert_eq!(arr[3], 4);
        assert_eq!(arr[4], 5);
        assert_eq!(pivot, 3);

        let mut arr = [5, 1, 4, 2, 3];
        let pivot = partition(&mut arr, 0, 4, Order::Desc, Partition::Center);

        assert_eq!(arr[0], 5);
        assert_eq!(arr[1], 4);
        assert!(arr[2] <= 4);
        assert!(arr[3] <= 4);
        assert!(arr[4] <= 4);
        assert_eq!(pivot, 1);
    }

    #[test]
    fn partition_inside() {
        let mut arr = [6, 5, 1, 4, 2, 3, 0];
        partition(&mut arr, 1, 5, Order::Asc, Partition::First);

        assert_eq!(arr[0], 6);
        assert_eq!(arr[5], 5);
        assert_eq!(arr[6], 0);
    }

    #[test]
    fn partition_duplicate() {
        let mut arr = [4, 1, 4, 2, 5];
        let pivot = partition(&mut arr, 0, 4, Order::Asc, Partition::First);

        assert!(arr[0] <= 4);
        assert!(arr[1] <= 4);
        assert_eq!(arr[2], 4);
        assert!(arr[3] >= 4);
        assert!(arr[4] >= 4);
        assert_eq!(pivot, 2);

        let mut arr = [5, 4, 3, 2, 4];
        let pivot = partition(&mut arr, 0, 4, Order::Asc, Partition::Last);

        assert!(arr[0] <= 4);
        assert!(arr[1] <= 4);
        assert_eq!(arr[2], 4);
        assert!(arr[3] >= 4);
        assert!(arr[4] >= 4);
        assert_eq!(pivot, 2);

        let mut arr = [5, 4, 4, 2, 3];
        let pivot = partition(&mut arr, 0, 4, Order::Asc, Partition::Center);

        assert_eq!(pivot, 2);
        assert!(arr[0] <= 4);
        assert!(arr[1] <= 4);
        assert_eq!(arr[2], 4);
        assert!(arr[3] >= 4);
        assert!(arr[4] >= 4);
    }

    #[test]
    fn partition_smallest() {
        let mut arr = [1, 4, 3, 2, 5];
        let pivot = partition(&mut arr, 0, 4, Order::Asc, Partition::First);

        assert_eq!(arr[0], 1);
        assert!(arr[1] >= 1);
        assert!(arr[2] >= 1);
        assert!(arr[3] >= 1);
        assert!(arr[4] >= 1);
        assert_eq!(pivot, 0);

        let mut arr = [1, 4, 3, 2, 5];
        let pivot = partition(&mut arr, 0, 4, Order::Desc, Partition::First);

        assert!(arr[0] >= 1);
        assert!(arr[1] >= 1);
        assert!(arr[2] >= 1);
        assert!(arr[3] >= 1);
        assert_eq!(arr[4], 1);
        assert_eq!(pivot, 4);

        let mut arr = [5, 3, 4, 2, 1];
        let pivot = partition(&mut arr, 0, 4, Order::Asc, Partition::Last);

        assert_eq!(arr[0], 1);
        assert!(arr[1] >= 1);
        assert!(arr[2] >= 1);
        assert!(arr[3] >= 1);
        assert!(arr[4] >= 1);
        assert_eq!(pivot, 0);

        let mut arr = [5, 3, 4, 2, 1];
        let pivot = partition(&mut arr, 0, 4, Order::Desc, Partition::Last);

        assert!(arr[0] >= 1);
        assert!(arr[1] >= 1);
        assert!(arr[2] >= 1);
        assert!(arr[3] >= 1);
        assert_eq!(arr[4], 1);
        assert_eq!(pivot, 4);

        let mut arr = [5, 3, 1, 2, 4];
        let pivot = partition(&mut arr, 0, 4, Order::Asc, Partition::Center);

        assert_eq!(arr[0], 1);
        assert!(arr[1] >= 1);
        assert!(arr[2] >= 1);
        assert!(arr[3] >= 1);
        assert!(arr[4] >= 1);
        assert_eq!(pivot, 0);

        let mut arr = [5, 3, 1, 2, 4];
        let pivot = partition(&mut arr, 0, 4, Order::Desc, Partition::Center);

        assert!(arr[0] >= 1);
        assert!(arr[1] >= 1);
        assert!(arr[2] >= 1);
        assert!(arr[3] >= 1);
        assert_eq!(arr[4], 1);
        assert_eq!(pivot, 4);
    }
}

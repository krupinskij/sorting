use super::partition::partition_by_predicate;
use crate::{Order, Partition};

pub fn sort<T: PartialOrd>(data: &mut [T], order: Order, partition: Partition) {
    let predicate = match order {
        Order::Asc => |a: &T, b: &T| a < b,
        Order::Desc => |a: &T, b: &T| a > b,
    };
    sort_by_predicate(data, predicate, partition);
}

pub fn sort_by_predicate<T, P>(data: &mut [T], predicate: P, partition: Partition)
where
    P: Fn(&T, &T) -> bool,
{
    sort_by_predicate_rec(data, 0, data.len() - 1, &predicate, &partition)
}

fn sort_by_predicate_rec<T, P>(
    data: &mut [T],
    l: usize,
    r: usize,
    predicate: &P,
    partition: &Partition,
) where
    P: Fn(&T, &T) -> bool,
{
    let pivot = partition_by_predicate(data, l, r, predicate, partition);
    if pivot > l + 1 {
        sort_by_predicate_rec(data, l, pivot - 1, predicate, partition);
    }
    if pivot + 1 < r {
        sort_by_predicate_rec(data, pivot + 1, r, predicate, partition);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{Car, Person};

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

        let mut arr = [
            5, 3, 8, 2, 1, 5, 7, 2, 6, 3, 0, 2, 4, 6, 8, 9, 9, 1, 5, 4, 3, 2, 4,
        ];
        sort(&mut arr, Order::Asc, Partition::First);
        assert_eq!(
            [0, 1, 1, 2, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 7, 8, 8, 9, 9],
            arr
        );
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

    #[test]
    fn sort_struct_vector() {
        let person1 = Person::new(25);
        let person2 = Person::new(15);
        let person3 = Person::new(35);

        let mut vec = vec![person1, person2, person3];
        sort(&mut vec, Order::Asc, Partition::First);
        assert_eq!(vec![person2, person1, person3], vec);
    }

    #[test]
    fn sort_array_by_predicate() {
        let mut arr = [
            5, 3, 8, 2, 1, 5, 7, 2, 6, 3, 0, 2, 4, 6, 8, 9, 9, 1, 5, 4, 3, 2, 4,
        ];
        sort_by_predicate(&mut arr, |a, b| a < b, Partition::First);
        assert_eq!(
            [0, 1, 1, 2, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 7, 8, 8, 9, 9],
            arr
        );
    }

    #[test]
    fn sort_struct_vector_by_predicate() {
        let car1 = Car { age: 25 };
        let car2 = Car { age: 15 };
        let car3 = Car { age: 35 };

        let mut vec = vec![car1, car2, car3];
        sort_by_predicate(&mut vec, |c1, c2| c1.age < c2.age, Partition::First);
        assert_eq!(vec![car2, car1, car3], vec);
    }
}

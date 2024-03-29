use std::cmp::PartialOrd;

use crate::Order;

pub fn sort<T: PartialOrd>(data: &mut [T], order: Order) {
    let predicate = match order {
        Order::Asc => |a: &T, b: &T| a < b,
        Order::Desc => |a: &T, b: &T| a > b,
    };
    sort_by_predicate(data, predicate);
}

pub fn sort_by_predicate<T, P>(data: &mut [T], predicate: P)
where
    P: Fn(&T, &T) -> bool,
{
    for i in 0..data.len() {
        let mut i_min = i;
        for j in i + 1..data.len() {
            if predicate(&data[j], &data[i_min]) {
                i_min = j;
            }
        }

        data.swap(i, i_min);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{Car, Person};

    #[test]
    fn sort_array_asc() {
        let mut arr = [2, 3, 1, 5, 4];
        sort(&mut arr, Order::Asc);
        assert_eq!([1, 2, 3, 4, 5], arr);
    }

    #[test]
    fn sort_array_desc() {
        let mut arr = [2, 3, 1, 5, 4];
        sort(&mut arr, Order::Desc);
        assert_eq!([5, 4, 3, 2, 1], arr);
    }

    #[test]
    fn sort_vector() {
        let mut vec = vec![2, 3, 1, 5, 4];
        sort(&mut vec, Order::Asc);
        assert_eq!(vec![1, 2, 3, 4, 5], vec);
    }

    #[test]
    fn sort_struct_vector() {
        let person1 = Person::new(25);
        let person2 = Person::new(15);
        let person3 = Person::new(35);

        let mut vec = vec![person1, person2, person3];
        sort(&mut vec, Order::Asc);
        assert_eq!(vec![person2, person1, person3], vec);
    }

    #[test]
    fn sort_by_predicate_array_asc() {
        let mut arr = [2, 3, 1, 5, 4];
        sort_by_predicate(&mut arr, |a, b| a < b);
        assert_eq!([1, 2, 3, 4, 5], arr);
    }

    #[test]
    fn sort_by_predicate_array_desc() {
        let mut arr = [2, 3, 1, 5, 4];
        sort_by_predicate(&mut arr, |a, b| a > b);
        assert_eq!([5, 4, 3, 2, 1], arr);
    }

    #[test]
    fn sort_by_predicate_vector() {
        let mut vec = vec![2, 3, 1, 5, 4];
        sort_by_predicate(&mut vec, |a, b| a < b);
        assert_eq!(vec![1, 2, 3, 4, 5], vec);
    }

    #[test]
    fn sort_by_predicate_struct_vector() {
        let car1 = Car { age: 25 };
        let car2 = Car { age: 15 };
        let car3 = Car { age: 35 };

        let mut vec = vec![car1, car2, car3];
        sort_by_predicate(&mut vec, |c1, c2| c1.age < c2.age);
        assert_eq!(vec![car2, car1, car3], vec);
    }
}

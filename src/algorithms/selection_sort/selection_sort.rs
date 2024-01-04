use std::cmp::PartialOrd;

use crate::Order;

pub fn sort<T: PartialOrd>(data: &mut [T], order: Order) {
    for i in 0..data.len() {
        let mut i_min = i;
        for j in i + 1..data.len() {
            match order {
                Order::Asc => {
                    if data[j] < data[i_min] {
                        i_min = j;
                    }
                }
                Order::Desc => {
                    if data[j] > data[i_min] {
                        i_min = j;
                    }
                }
            }
        }

        data.swap(i, i_min);
    }
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
    use crate::helpers::Person;

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
        let person1 = Person { age: 25 };
        let person2 = Person { age: 15 };
        let person3 = Person { age: 35 };

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
        let person1 = Person { age: 25 };
        let person2 = Person { age: 15 };
        let person3 = Person { age: 35 };

        let mut vec = vec![person1, person2, person3];
        sort_by_predicate(&mut vec, |person1, person2| person1.age < person2.age);
        assert_eq!(vec![person2, person1, person3], vec);
    }
}

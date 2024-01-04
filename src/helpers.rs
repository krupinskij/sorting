#[derive(Clone, Copy, Debug)]
pub struct Person {
    age: u8,
}

impl Person {
    pub fn new(age: u8) -> Person {
        Person { age }
    }
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.age.cmp(&other.age))
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.age == other.age
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Car {
    pub age: u8,
}

impl PartialEq for Car {
    fn eq(&self, other: &Self) -> bool {
        self.age == other.age
    }
}

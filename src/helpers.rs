#[derive(Clone, Copy, Debug)]
pub struct Person {
    pub age: u8,
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

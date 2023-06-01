use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

crate::entry_point!("custom_array", main);

struct Person {
    name: String,
    age: u8,
    oldest_child: Option<Rc<RefCell<Person>>>,
}

#[derive(Debug)]
struct FixedSizeArray {
    data: [u8; 10],
    current: usize,
}

impl FixedSizeArray {
    fn new() -> Self {
        FixedSizeArray {
            data: [0; 10],
            current: 0,
        }
    }

    fn push(&mut self, value: u8) {
        if self.current < 10 {
            self.data[self.current] = value;
            self.current += 1;
        } else {
            panic!("Too many children");
        }
    }
}

impl Deref for FixedSizeArray {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &self.data[0..self.current]
    }
}

impl Person {
    fn new(name: &str, age: u8, oldest_child: Option<Rc<RefCell<Person>>>) -> Rc<RefCell<Person>> {
        Rc::new(RefCell::new(Person {
            name: name.to_string(),
            age,
            oldest_child,
        }))
    }

    fn collect_ages(&self, ages: &mut FixedSizeArray) {
        ages.push(self.age);

        if let Some(child) = &self.oldest_child {
            child.borrow().collect_ages(ages);
        }
    }
}

fn rsf(xs: &[u8]) -> u8 {
    if xs.len() > 0 {
        xs[0]
    } else {
        255
    }
}

pub fn main() {
    let child = Person::new("Child", 12, None);
    let parent = Person::new("Parent", 35, Some(child.clone()));
    let grandparent = Person::new("Grandparent", 60, Some(parent.clone()));

    let mut ages = FixedSizeArray::new();
    grandparent.borrow().collect_ages(&mut ages);
    println!("{:?}", ages); // prints: [60, 35, 12]

    println!("{} is {} years old.", grandparent.borrow().name, rsf(&ages));
}

// Tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_size_array() {
        let mut ages = FixedSizeArray::new();
        ages.push(1);
        ages.push(2);
        ages.push(3);
        ages.push(4);
        ages.push(5);
        ages.push(6);
        ages.push(7);
        ages.push(8);
        ages.push(9);
        ages.push(10);

        assert_eq!(ages[0], 1);
        assert_eq!(ages[1], 2);
        assert_eq!(ages[2], 3);
        assert_eq!(ages[3], 4);
        assert_eq!(ages[4], 5);
        assert_eq!(ages[5], 6);
        assert_eq!(ages[6], 7);
        assert_eq!(ages[7], 8);
        assert_eq!(ages[8], 9);
        assert_eq!(ages[9], 10);
    }

    #[test]
    #[should_panic]
    fn test_fixed_size_array_panic() {
        let mut ages = FixedSizeArray::new();
        ages.push(1);
        ages.push(2);
        ages.push(3);
        ages.push(4);
        ages.push(5);
        ages.push(6);
        ages.push(7);
        ages.push(8);
        ages.push(9);
        ages.push(10);
        ages.push(11);
    }

    #[test]
    fn grandparent_has_three_children() {
        let child = Person::new("Child", 12, None);
        let parent = Person::new("Parent", 35, Some(child.clone()));
        let grandparent = Person::new("Grandparent", 60, Some(parent.clone()));

        let mut ages = FixedSizeArray::new();
        grandparent.borrow().collect_ages(&mut ages);

        assert_eq!(ages[0], 60);
        assert_eq!(ages[1], 35);
        assert_eq!(ages[2], 12);
    }

    #[test]
    fn ages_can_be_used_as_slice() {
        let mut ages = FixedSizeArray::new();
        ages.push(1);
        assert_eq!(rsf(&ages), 1);
    }
}

use crate::LinkedList::*;
use std::cmp::PartialEq;

#[derive(Debug)]
enum LinkedList {
    Cons(u32, Box<LinkedList>),
    Nil,
}

impl PartialEq for LinkedList {
    fn eq(&self, other: &Self) -> bool {
        self.stringify() == other.stringify()
    }
}

impl LinkedList {
    fn new() -> LinkedList {
        Nil
    }

    fn prepend(self, elem: u32) -> LinkedList {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn empty_linked_list() {
    let list = LinkedList::new();

    assert_eq!(list, LinkedList::Nil);
}

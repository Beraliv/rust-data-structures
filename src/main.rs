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
    pub fn new() -> LinkedList {
        Nil
    }

    pub fn prepend(self, elem: u32) -> LinkedList {
        Cons(elem, Box::new(self))
    }

    pub fn clear(self) -> LinkedList {
        Nil
    }

    pub fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    pub fn stringify(&self) -> String {
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

#[test]
fn prepend_linked_list() {
    let mut list = LinkedList::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    assert_eq!(
        list,
        LinkedList::Cons(
            3,
            Box::new(LinkedList::Cons(
                2,
                Box::new(LinkedList::Cons(1, Box::new(LinkedList::Nil)))
            ))
        )
    )
}

#[test]
fn linked_list_len() {
    let mut list = LinkedList::new();
    assert_eq!(list.len(), 0);

    list = list.prepend(3);
    assert_eq!(list.len(), 1);

    list = list.prepend(2);
    assert_eq!(list.len(), 2);

    list = list.prepend(1);
    assert_eq!(list.len(), 3);

    list = list.clear();
    assert_eq!(list.len(), 0);
}

#[test]
fn stringify_linked_list() {
    let mut list = LinkedList::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    assert_eq!(list.stringify(), "3, 2, 1, Nil")
}

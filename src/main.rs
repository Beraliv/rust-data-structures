use crate::LinkedList::*;

enum LinkedList {
    Cons(u32, Box<LinkedList>),
    Nil,
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

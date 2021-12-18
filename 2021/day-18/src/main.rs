#![feature(test)]
extern crate test;

use std::{
    cell::RefCell,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    rc::{Rc, Weak},
};

enum Contents {
    Regular(i32),
    Pair(Rc<RefCell<Node>>, Rc<RefCell<Node>>),
}

struct Node {
    parent: Option<Weak<RefCell<Node>>>,
    contents: Contents,
}

impl Node {
    fn new_regular(value: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            parent: None,
            contents: Contents::Regular(value),
        }))
    }

    fn new_pair(left: Rc<RefCell<Node>>, right: Rc<RefCell<Node>>) -> Rc<RefCell<Self>> {
        let pair = Rc::new(RefCell::new(Node {
            parent: None,
            contents: Contents::Pair(left, right),
        }));
        if let Contents::Pair(left, right) = &mut pair.borrow_mut().contents {
            left.borrow_mut().parent = Some(Rc::downgrade(&pair));
            right.borrow_mut().parent = Some(Rc::downgrade(&pair));
        }
        pair
    }

    fn parse_from_bytes(bytes: &[u8]) -> (Rc<RefCell<Self>>, usize) {
        match bytes[0] {
            b'[' => {
                let (left, left_size) = Self::parse_from_bytes(&bytes[1..]);
                let (right, right_size) = Self::parse_from_bytes(&bytes[1 + left_size + 1..]);
                (
                    Self::new_pair(left, right),
                    1 + left_size + 1 + right_size + 1,
                )
            }
            _ => (Self::new_regular((bytes[0] - b'0') as i32), 1),
        }
    }

    fn visit_regular_nodes(&self, visitor: &dyn Fn(&Node)) {
        match &self.contents {
            Contents::Regular(_) => visitor(self),
            Contents::Pair(left, right) => {
                left.borrow().visit_regular_nodes(&visitor);
                right.borrow().visit_regular_nodes(&visitor);
            }
        }
    }

    fn visit_pair_nodes(&self, visitor: &dyn Fn(&Node)) {
        match &self.contents {
            Contents::Regular(_) => return,
            Contents::Pair(left, right) => {
                let mut visited_child = false;
                if let Contents::Pair(_, _) = left.borrow().contents {
                    left.borrow().visit_pair_nodes(visitor);
                    visited_child = true;
                }
                if let Contents::Pair(_, _) = right.borrow().contents {
                    right.borrow().visit_pair_nodes(visitor);
                    visited_child = true;
                }

                if !visited_child {
                    visitor(self);
                }
            }
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.contents {
            Contents::Regular(value) => f.write_str(&format!("{}", value)),
            Contents::Pair(left, right) => {
                f.write_str(&format!("[{},{}]", left.borrow(), right.borrow()))
            }
        }
    }
}

fn main() {
    // let file = File::open("input.txt").unwrap();
    // let reader = BufReader::new(file);

    let (longer, _size) =
        Node::parse_from_bytes(String::from("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]").as_bytes());
    println!("{}", longer.borrow());

    longer
        .borrow()
        .visit_regular_nodes(&|node| println!("{}", node));
    longer
        .borrow()
        .visit_pair_nodes(&|node| println!("{}", node));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    // #[test]
    // fn test_sample() {
    //     assert_eq!(get_possible_values(20..=30, -10..=-5).len(), 112);
    // }

    // #[bench]
    // fn bench_input(b: &mut Bencher) {
    //     b.iter(|| {
    //         assert_eq!(get_possible_values(241..=273, -97..=-63).len(), 1908);
    //     })
    // }
}

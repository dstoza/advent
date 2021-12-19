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

#[derive(Eq, PartialEq)]
enum Position {
    Left,
    Right,
}

struct Node {
    parent: Option<(Weak<RefCell<Node>>, Position)>,
    weak_self: Weak<RefCell<Node>>,
    contents: Contents,
}

impl Node {
    fn new_regular(value: i32) -> Rc<RefCell<Self>> {
        let node = Rc::new(RefCell::new(Node {
            parent: None,
            weak_self: Weak::new(),
            contents: Contents::Regular(value),
        }));
        node.borrow_mut().weak_self = Rc::downgrade(&node);
        node
    }

    fn new_pair(left: Rc<RefCell<Node>>, right: Rc<RefCell<Node>>) -> Rc<RefCell<Self>> {
        let pair = Rc::new(RefCell::new(Node {
            parent: None,
            weak_self: Weak::new(),
            contents: Contents::Pair(left, right),
        }));
        pair.borrow_mut().weak_self = Rc::downgrade(&pair);
        if let Contents::Pair(left, right) = &mut pair.borrow_mut().contents {
            left.borrow_mut().parent = Some((Rc::downgrade(&pair), Position::Left));
            right.borrow_mut().parent = Some((Rc::downgrade(&pair), Position::Right));
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

    // Visitor returns whether to continue visiting
    fn visit_regular_nodes(&self, visitor: &dyn Fn(&Node) -> bool) -> bool {
        match &self.contents {
            Contents::Regular(_) => return visitor(self),
            Contents::Pair(left, right) => {
                if !left.borrow().visit_regular_nodes(&visitor) {
                    return false;
                }
                if !right.borrow().visit_regular_nodes(&visitor) {
                    return false;
                }
            }
        }
        true
    }

    // Visitor returns whether to continue visiting
    fn visit_pair_nodes(&self, visitor: &dyn Fn(&Node, usize) -> bool, depth: usize) -> bool {
        match &self.contents {
            Contents::Regular(_) => return true,
            Contents::Pair(left, right) => {
                let mut visited_child = false;
                if let Contents::Pair(_, _) = left.borrow().contents {
                    if !left.borrow().visit_pair_nodes(visitor, depth + 1) {
                        return false;
                    }
                    visited_child = true;
                }
                if let Contents::Pair(_, _) = right.borrow().contents {
                    if !right.borrow().visit_pair_nodes(visitor, depth + 1) {
                        return false;
                    }
                    visited_child = true;
                }

                if !visited_child {
                    return visitor(self, depth);
                }
            }
        };
        true
    }

    fn get_farthest_node_by_position(&self, position: Position) -> Rc<RefCell<Node>> {
        match &self.contents {
            Contents::Regular(_) => Weak::upgrade(&self.weak_self).unwrap(),
            Contents::Pair(left, right) => match position {
                Position::Left => left.borrow().get_farthest_node_by_position(Position::Left),
                Position::Right => right
                    .borrow()
                    .get_farthest_node_by_position(Position::Right),
            },
        }
    }

    fn get_next_regular_node_past_position(
        &self,
        past_position: Position,
    ) -> Option<Rc<RefCell<Node>>> {
        if let Some((parent, position)) = &self.parent {
            if *position == past_position {
                Weak::upgrade(&parent)
                    .unwrap()
                    .borrow()
                    .get_next_regular_node_past_position(past_position)
            } else {
                if let Contents::Pair(left, right) =
                    &Weak::upgrade(&parent).unwrap().borrow().contents
                {
                    match past_position {
                        Position::Left => {
                            Some(left.borrow().get_farthest_node_by_position(Position::Right))
                        }
                        Position::Right => {
                            Some(right.borrow().get_farthest_node_by_position(Position::Left))
                        }
                    }
                } else {
                    None
                }
            }
        } else {
            None
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

    longer.borrow().visit_regular_nodes(&|node| {
        println!(
            "{} <- {} -> {}",
            if let Some(node) = node.get_next_regular_node_past_position(Position::Left) {
                format!("{}", node.borrow())
            } else {
                String::from("None")
            },
            node,
            if let Some(node) = node.get_next_regular_node_past_position(Position::Right) {
                format!("{}", node.borrow())
            } else {
                String::from("None")
            }
        );
        true
    });
    longer.borrow().visit_pair_nodes(
        &|node, depth| {
            println!(
                "{} <- {} {} -> {}",
                if let Some(node) = node.get_next_regular_node_past_position(Position::Left) {
                    format!("{}", node.borrow())
                } else {
                    String::from("None")
                },
                node,
                depth,
                if let Some(node) = node.get_next_regular_node_past_position(Position::Right) {
                    format!("{}", node.borrow())
                } else {
                    String::from("None")
                }
            );
            true
        },
        0,
    );
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

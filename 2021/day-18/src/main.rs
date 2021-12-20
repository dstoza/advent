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

#[derive(Debug, Eq, PartialEq)]
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
    fn visit_regular_nodes(&self, visitor: &mut dyn FnMut(&Node) -> bool) -> bool {
        match &self.contents {
            Contents::Regular(_) => return visitor(self),
            Contents::Pair(left, right) => {
                if !left.borrow().visit_regular_nodes(visitor) {
                    return false;
                }
                if !right.borrow().visit_regular_nodes(visitor) {
                    return false;
                }
            }
        }
        true
    }

    // Visitor returns whether to continue visiting
    fn visit_pair_nodes(
        &self,
        visitor: &mut dyn FnMut(&Node, usize) -> bool,
        depth: usize,
    ) -> bool {
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
                Weak::upgrade(parent)
                    .unwrap()
                    .borrow()
                    .get_next_regular_node_past_position(past_position)
            } else if let Contents::Pair(left, right) =
                &Weak::upgrade(parent).unwrap().borrow().contents
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
        } else {
            None
        }
    }

    fn get_magnitude(&self) -> i32 {
        match &self.contents {
            Contents::Regular(value) => *value,
            Contents::Pair(left, right) => {
                3 * left.borrow().get_magnitude() + 2 * right.borrow().get_magnitude()
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

// Returns whether a pair exploded
fn explode(root: &Rc<RefCell<Node>>) -> bool {
    let mut node_to_explode = None;
    root.borrow().visit_pair_nodes(
        &mut |node, depth| {
            if depth < 4 {
                return true;
            }

            node_to_explode = Some(Weak::upgrade(&node.weak_self).unwrap());
            false
        },
        0,
    );

    if let Some(node) = node_to_explode {
        let (left_value, right_value) = if let Contents::Pair(left, right) = &node.borrow().contents
        {
            let left_value = if let Contents::Regular(value) = &left.borrow().contents {
                *value
            } else {
                unreachable!()
            };
            let right_value = if let Contents::Regular(value) = &right.borrow().contents {
                *value
            } else {
                unreachable!()
            };
            (left_value, right_value)
        } else {
            unreachable!()
        };

        if let Some(next_node_left) = node
            .borrow()
            .get_next_regular_node_past_position(Position::Left)
        {
            if let Contents::Regular(value) = &mut next_node_left.borrow_mut().contents {
                *value += left_value;
            }
        }
        if let Some(next_node_right) = node
            .borrow()
            .get_next_regular_node_past_position(Position::Right)
        {
            if let Contents::Regular(value) = &mut next_node_right.borrow_mut().contents {
                *value += right_value;
            }
        }
        node.borrow_mut().contents = Contents::Regular(0);
        true
    } else {
        false
    }
}

// Returns whether a node split
fn split(root: &Rc<RefCell<Node>>) -> bool {
    let mut node_to_split = None;
    root.borrow().visit_regular_nodes(&mut |node| {
        if let Contents::Regular(value) = &node.contents {
            if *value >= 10 {
                node_to_split = Some(Weak::upgrade(&node.weak_self).unwrap());
                return false;
            }
        }

        true
    });

    if let Some(node) = node_to_split {
        let value = if let Contents::Regular(value) = &node.borrow().contents {
            *value
        } else {
            unreachable!()
        };
        let left = Node::new_regular(value / 2);
        left.borrow_mut().parent = Some((node.borrow().weak_self.clone(), Position::Left));
        let right = Node::new_regular(value - value / 2);
        right.borrow_mut().parent = Some((node.borrow().weak_self.clone(), Position::Right));
        node.borrow_mut().contents = Contents::Pair(left, right);

        true
    } else {
        false
    }
}

fn reduce(root: &Rc<RefCell<Node>>) {
    loop {
        if explode(root) {
            continue;
        }
        if split(root) {
            continue;
        }
        break;
    }
}

fn reduce_list<I: Iterator<Item = String>>(mut list: I) -> Rc<RefCell<Node>> {
    let mut left = Node::parse_from_bytes(list.next().unwrap().as_bytes()).0;
    for item in list {
        let root = Node::new_pair(left, Node::parse_from_bytes(item.as_bytes()).0);
        reduce(&root);
        left = root;
    }
    left
}

fn get_maximum_magnitude(numbers: &[String]) -> i32 {
    let mut maximum = 0;
    for (index, a) in numbers.iter().enumerate() {
        for b in &numbers[index + 1..] {
            let forwards = Node::new_pair(
                Node::parse_from_bytes(a.as_bytes()).0,
                Node::parse_from_bytes(b.as_bytes()).0,
            );
            reduce(&forwards);
            maximum = maximum.max(forwards.borrow().get_magnitude());

            let backwards = Node::new_pair(
                Node::parse_from_bytes(b.as_bytes()).0,
                Node::parse_from_bytes(a.as_bytes()).0,
            );
            reduce(&backwards);
            maximum = maximum.max(backwards.borrow().get_magnitude());
        }
    }
    maximum
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    // println!(
    //     "Magnitude: {}",
    //     reduce_list(reader.lines().map(|line| line.unwrap()))
    //         .borrow()
    //         .get_magnitude()
    // )
    let lines: Vec<_> = reader.lines().map(|line| line.unwrap()).collect();
    println!("Maximum magnitude: {}", get_maximum_magnitude(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    fn explode_helper(before: String, after: &str) {
        let root = Node::parse_from_bytes(before.as_bytes()).0;
        assert_eq!(explode(&root), true);
        assert_eq!(format!("{}", root.borrow()), after);
    }

    #[test]
    fn test_explode() {
        explode_helper(String::from("[[[[[9,8],1],2],3],4]"), "[[[[0,9],2],3],4]");
        explode_helper(String::from("[7,[6,[5,[4,[3,2]]]]]"), "[7,[6,[5,[7,0]]]]");
        explode_helper(String::from("[[6,[5,[4,[3,2]]]],1]"), "[[6,[5,[7,0]]],3]");
        explode_helper(
            String::from("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"),
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        );
        explode_helper(
            String::from("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"),
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        );
    }

    #[test]
    fn test_split() {
        let root = Node::new_pair(Node::new_regular(11), Node::new_regular(11));
        assert_eq!(split(&root), true);
        assert_eq!(format!("{}", root.borrow()), "[[5,6],11]");
        assert_eq!(split(&root), true);
        assert_eq!(format!("{}", root.borrow()), "[[5,6],[5,6]]");
        assert_eq!(split(&root), false);
    }

    #[test]
    fn test_reduce() {
        let root = Node::parse_from_bytes(
            String::from("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]").as_bytes(),
        )
        .0;
        reduce(&root);
        assert_eq!(
            format!("{}", root.borrow()),
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
        );
    }

    #[test]
    fn test_reduce_list_4() {
        let list = [
            String::from("[1,1]"),
            String::from("[2,2]"),
            String::from("[3,3]"),
            String::from("[4,4]"),
        ];
        assert_eq!(
            format!("{}", reduce_list(list.into_iter()).borrow()),
            "[[[[1,1],[2,2]],[3,3]],[4,4]]"
        );
    }

    #[test]
    fn test_reduce_list_5() {
        let list = [
            String::from("[1,1]"),
            String::from("[2,2]"),
            String::from("[3,3]"),
            String::from("[4,4]"),
            String::from("[5,5]"),
        ];
        assert_eq!(
            format!("{}", reduce_list(list.into_iter()).borrow()),
            "[[[[3,0],[5,3]],[4,4]],[5,5]]"
        );
    }

    #[test]
    fn test_reduce_list_6() {
        let list = [
            String::from("[1,1]"),
            String::from("[2,2]"),
            String::from("[3,3]"),
            String::from("[4,4]"),
            String::from("[5,5]"),
            String::from("[6,6]"),
        ];
        assert_eq!(
            format!("{}", reduce_list(list.into_iter()).borrow()),
            "[[[[5,0],[7,4]],[5,5]],[6,6]]"
        );
    }

    #[test]
    fn test_larger_example_step() {
        let list = [
            String::from("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"),
            String::from("[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]"),
        ];
        assert_eq!(
            format!("{}", reduce_list(list.into_iter()).borrow()),
            "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]"
        );
    }

    #[test]
    fn test_larger_example() {
        let list = [
            String::from("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]"),
            String::from("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]"),
            String::from("[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]"),
            String::from("[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]"),
            String::from("[7,[5,[[3,8],[1,4]]]]"),
            String::from("[[2,[2,2]],[8,[8,1]]]"),
            String::from("[2,9]"),
            String::from("[1,[[[9,3],9],[[9,0],[0,7]]]]"),
            String::from("[[[5,[7,4]],7],1]"),
            String::from("[[[[4,2],2],6],[8,7]]"),
        ];
        assert_eq!(
            format!("{}", reduce_list(list.into_iter()).borrow()),
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
        );
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(
            Node::parse_from_bytes(String::from("[[1,2],[[3,4],5]]").as_bytes())
                .0
                .borrow()
                .get_magnitude(),
            143
        );
    }

    #[test]
    fn test_example_magnitude() {
        let list = [
            String::from("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]"),
            String::from("[[[5,[2,8]],4],[5,[[9,9],0]]]"),
            String::from("[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]"),
            String::from("[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]"),
            String::from("[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]"),
            String::from("[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]"),
            String::from("[[[[5,4],[7,7]],8],[[8,3],8]]"),
            String::from("[[9,3],[[9,9],[6,[4,9]]]]"),
            String::from("[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]"),
            String::from("[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"),
        ];
        assert_eq!(reduce_list(list.into_iter()).borrow().get_magnitude(), 4140);
    }

    #[bench]
    fn bench_input(b: &mut Bencher) {
        let file = File::open("input.txt").unwrap();
        let reader = BufReader::new(file);
        let lines: Vec<_> = reader.lines().map(|line| line.unwrap()).collect();

        b.iter(|| {
            assert_eq!(get_maximum_magnitude(&lines), 4638);
        })
    }
}

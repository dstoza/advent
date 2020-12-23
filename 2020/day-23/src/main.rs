#![deny(clippy::all, clippy::pedantic)]
#![feature(test)]

extern crate test;

use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use clap::{crate_name, App, Arg};

struct ListNode {
    value: u32,
    next: Weak<RefCell<ListNode>>,
}

impl ListNode {
    fn new(value: u32) -> Self {
        Self {
            value,
            next: Weak::new(),
        }
    }
}

fn main() {
    let args = App::new(crate_name!())
        .arg(Arg::from_usage("<INPUT>"))
        .arg(Arg::from_usage("<STEPS>"))
        .get_matches();

    let mut cups = HashMap::new();

    let mut max = 0;
    let mut head = None;
    let mut tail: Option<Rc<RefCell<ListNode>>> = None;
    for value in args.value_of("INPUT").unwrap().chars().map(|character| {
        String::from(character)
            .parse::<u32>()
            .expect("Failed to parse cup as u8")
    }) {
        max = max.max(value);
        let cup = Rc::new(RefCell::new(ListNode::new(value)));
        cups.insert(value, cup.clone());
        if let None = head {
            head = Some(cup.clone());
        }
        match &mut tail {
            Some(tail_node) => {
                tail_node.borrow_mut().next = Rc::downgrade(&cup);
                tail = Some(cup.clone());
            }
            None => {
                tail = Some(cup.clone());
            }
        }
    }

    // let cup_count = max;

    let cup_count = 1_000_000;
    for value in max + 1..=cup_count {
        let cup = Rc::new(RefCell::new(ListNode::new(value)));
        if let Some(tail_node) = &mut tail {
            tail_node.borrow_mut().next = Rc::downgrade(&cup);
            tail = Some(cup.clone());
        }
        cups.insert(value, cup);
    }

    // Complete the circular list
    let head = head.unwrap();
    let tail = tail.unwrap();
    tail.borrow_mut().next = Rc::downgrade(&head);

    let steps: usize = args.value_of("STEPS").unwrap().parse().unwrap();

    let mut current = head.clone();
    for _ in 0..steps {
        let mut picked_up = Vec::new();
        let pickup_head = current.borrow().next.upgrade().unwrap();
        picked_up.push(pickup_head.borrow().value);
        let mut pickup_tail = pickup_head.borrow().next.upgrade().unwrap();
        picked_up.push(pickup_tail.borrow().value);
        let next = pickup_tail.borrow().next.upgrade().unwrap();
        pickup_tail = next;
        picked_up.push(pickup_tail.borrow().value);
        current.borrow_mut().next = pickup_tail.borrow().next.clone();

        // println!("Picked up: {:?}", picked_up);

        let mut destination = (current.borrow().value + cup_count - 2) % cup_count + 1;
        while picked_up.iter().any(|value| *value == destination) {
            destination = (destination + cup_count - 2) % cup_count + 1
        }

        // println!("Destination: {}", destination);

        let destination_node = &cups[&destination];
        let destination_next = destination_node.borrow().next.clone();
        destination_node.borrow_mut().next = Rc::downgrade(&pickup_head);
        pickup_tail.borrow_mut().next = destination_next;

        let next = current.borrow().next.clone();
        current = next.upgrade().unwrap();
    }

    while current.borrow().value != 1 {
        let next = current.borrow().next.upgrade().unwrap();
        current = next;
    }

    let next = current.borrow().next.upgrade().unwrap();
    current = next;

    let mut product = 1;
    for _ in 0..2 {
        product *= current.borrow().value as u64;
        let next = current.borrow().next.upgrade().unwrap();
        current = next;
    }

    println!("Product: {}", product);

    /*


    while current.borrow().value != 1 {
        print!("{}", current.borrow().value);
        let next = current.borrow().next.upgrade().unwrap();
        current = next;
    }
    
    println!();
    */

    /*
    let mut cursor = 0;
    while cups[cursor] != 1 {
        cursor += 1;
    }
    cursor += 1;
    for _ in 0..cup_count - 1 {
        print!("{}", cups[cursor]);
        cursor = (cursor + 1) % cup_count;
    }
    println!();
    */
}

#[cfg(test)]
mod tests {
    // use test::Bencher;
}

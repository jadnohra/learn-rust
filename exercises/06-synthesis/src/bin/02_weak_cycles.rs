//! Exercise 2: Breaking Cycles with Weak
//!
//! Design a tree with parent pointers

use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,      // Weak: doesn't keep parent alive
    children: RefCell<Vec<Rc<Node>>>, // Strong: parent owns children
}

impl Node {
    fn new(value: i32) -> Rc<Self> {
        Rc::new(Node {
            value,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(Vec::new()),
        })
    }

    fn add_child(parent: &Rc<Node>, child: Rc<Node>) {
        *child.parent.borrow_mut() = Rc::downgrade(parent);
        parent.children.borrow_mut().push(child);
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping node with value: {}", self.value);
    }
}

fn example() {
    let root = Node::new(1);
    let child1 = Node::new(2);
    let child2 = Node::new(3);

    Node::add_child(&root, child1);
    Node::add_child(&root, child2);

    println!("Tree created. Root has {} children.",
             root.children.borrow().len());

    // Access parent from child
    if let Some(parent) = root.children.borrow()[0].parent.borrow().upgrade() {
        println!("Child's parent value: {}", parent.value);
    }

    println!("Dropping tree...");
    // Watch the drop order: children first, then root
}

fn exercise() {
    // TODO: Create a doubly-linked list using Weak for prev pointers
    //
    // struct ListNode {
    //     value: i32,
    //     prev: RefCell<Weak<ListNode>>,
    //     next: RefCell<Option<Rc<ListNode>>>,
    // }
    //
    // Build a list: 1 <-> 2 <-> 3
    // Navigate forward and backward
    // Verify proper cleanup (no memory leaks)
    //
    // Question: What would happen if prev used Rc instead of Weak?

    todo!("Exercise incomplete");
}

fn main() {
    example();
    exercise();
}

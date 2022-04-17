use std::{cmp::PartialOrd, fmt::Debug};

#[derive(Debug)]
pub struct BinaryTree<T> {
    node: Option<Box<Node<T>>>,
}

#[derive(Debug)]
struct Node<T> {
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    item: T,
}

impl<T> Node<T> {
    fn new(item: T) -> Self {
        Node {
            left: None,
            right: None,
            item,
        }
    }
}

impl<T> BinaryTree<T>
where
    T: PartialOrd<T>,
    T: PartialEq<T>,
    T: Debug,
{
    pub fn new() -> Self {
        BinaryTree { node: None }
    }

    pub fn add(&mut self, item: T) {
        if let Some(ref mut first) = self.node {
            let mut last_node = first;
            loop {
                if item > last_node.item {
                    if let Some(ref mut right) = last_node.right {
                        last_node = right
                    } else {
                        last_node.right = Some(Box::new(Node::new(item)));
                        break;
                    }
                } else {
                    if let Some(ref mut left) = last_node.left {
                        last_node = left
                    } else {
                        last_node.left = Some(Box::new(Node::new(item)));
                        break;
                    }
                }
            }
        } else {
            self.node = Some(Box::new(Node::new(item)))
        }
    }

    pub fn contains(&self, item: T) -> bool {
        fn contains<T>(option: &Option<Box<Node<T>>>, item: &T) -> bool
        where
            T: PartialEq<T>,
            T: PartialOrd<T>,
        {
            if let Some(node) = option {
                return if *item == node.item {
                    true
                } else if *item > node.item {
                    contains(&node.right, item)
                } else {
                    contains(&node.left, item)
                };
            }
            false
        }
        contains(&self.node, &item)
    }

    pub fn print(&self) {
        fn print<T: Debug>(node: &Node<T>) {
            if let Some(ref left) = node.left {
                print(left);
            }
            print!("{:?}, ", node.item);
            if let Some(ref right) = node.right {
                print(right);
            }
        }
        if let Some(ref node) = self.node {
            print(node);
        }
        println!();
    }
}

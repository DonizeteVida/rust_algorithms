use std::cmp::PartialOrd;

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
}

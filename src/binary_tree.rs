pub trait Comparator<T> {
    fn compare(&self, to_compare: &Self) -> bool;
}

pub struct BinaryTree<T> {
    node: Option<Box<Node<T>>>,
}

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

    fn right(&mut self, node: Node<T>) {
        self.right = Some(Box::new(node))
    }

    fn left(&mut self, node: Node<T>) {
        self.left = Some(Box::new(node))
    }
}

impl<T> BinaryTree<T>
where
    T: Comparator<T>,
{
    pub fn new() -> Self {
        BinaryTree { node: None }
    }

    pub fn add(&mut self, item: T) {
        if let Some(ref first) = self.node {
            let mut last_node = first;
            loop {
                if (last_node.item.compare(&item)) {
                    
                }
            }
        } else {
            self.node = Some(Box::new(Node::new(item)))
        }
    }
}

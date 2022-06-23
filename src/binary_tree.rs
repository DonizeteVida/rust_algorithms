use std::{fmt::Debug};

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
        fn last<'a, T>(
            option: &'a mut Option<Box<Node<T>>>,
            item: &T,
        ) -> &'a mut Option<Box<Node<T>>>
        where
            T: PartialEq<T>,
            T: PartialOrd<T>,
        {
            if let Some(node) = option {
                if *item > node.item {
                    last(&mut node.right, item)
                } else {
                    last(&mut node.left, item)
                }
            } else {
                option
            }
        }
        let last = last(&mut self.node, &item);
        *last = Some(Box::new(Node::new(item)));
    }

    pub fn inorder<F: Fn(&T)>(&self, f: F) {
        fn inorder<T, F: Fn(&T)>(f: &F, option: &Option<Box<Node<T>>>) {
            if let Some(node) = option {
                inorder(f, &node.left);
                f(&node.item);
                inorder(f, &node.right);
            }
        }
        inorder(&f, &self.node);
    }

    pub fn preorder<F: Fn(&T)>(&self, f: F) {
        fn preorder<T, F: Fn(&T)>(f: &F, option: &Option<Box<Node<T>>>) {
            if let Some(node) = option {
                f(&node.item);
                preorder(f, &node.left);
                preorder(f, &node.right);
            }
        }
        preorder(&f, &self.node);
    }

    pub fn postorder<F: Fn(&T)>(&self, f: F) {
        fn postorder<T, F: Fn(&T)>(f: &F, option: &Option<Box<Node<T>>>) {
            if let Some(node) = option {
                postorder(f, &node.left);
                postorder(f, &node.right);
                f(&node.item);
            }
        }
        postorder(&f, &self.node);
    }

    pub fn remove(&mut self, item: T) -> Option<T> {
        None
    }

    pub fn contains(&self, item: T) -> bool {
        fn contains<T>(option: &Option<Box<Node<T>>>, item: &T) -> bool
        where
            T: PartialEq<T>,
            T: PartialOrd<T>,
        {
            if let Some(node) = option {
                if *item == node.item {
                    true
                } else if *item > node.item {
                    contains(&node.right, item)
                } else {
                    contains(&node.left, item)
                }
            } else {
                false
            }
        }
        contains(&self.node, &item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_contain_value() {
        let mut binary_tree = BinaryTree::<i8>::new();
        binary_tree.add(2);
        binary_tree.add(3);
        binary_tree.add(-1);
        binary_tree.add(-4);

        assert_eq!(binary_tree.contains(-1), true);
    }

    #[test]
    fn should_remove_value() {
        let mut binary_tree = BinaryTree::<i8>::new();
        binary_tree.add(2);
        binary_tree.add(3);
        binary_tree.add(-1);
        binary_tree.add(-4);

        assert_eq!(binary_tree.contains(-1), true);

        assert_eq!(binary_tree.remove(-1), Some(-1));

        assert_eq!(binary_tree.contains(2), true);
        assert_eq!(binary_tree.contains(3), true);
        assert_eq!(binary_tree.contains(-4), true);
    }

    #[test]
    fn should_inorder() {
        let mut binary_tree = BinaryTree::<i8>::new();
        binary_tree.add(2);
        binary_tree.add(3);
        binary_tree.add(-1);
        binary_tree.add(-4);

        binary_tree.inorder(|item| println!("{}", item));
        assert!(true)
    }

    #[test]
    fn should_preorder() {
        let mut binary_tree = BinaryTree::<i8>::new();
        binary_tree.add(2);
        binary_tree.add(3);
        binary_tree.add(-1);
        binary_tree.add(-4);

        binary_tree.preorder(|item| println!("{}", item));
        assert!(true)
    }

    #[test]
    fn should_postorder() {
        let mut binary_tree = BinaryTree::<i8>::new();
        binary_tree.add(2);
        binary_tree.add(3);
        binary_tree.add(-1);
        binary_tree.add(-4);

        binary_tree.postorder(|item| println!("{}", item));
        assert!(true)
    }
}

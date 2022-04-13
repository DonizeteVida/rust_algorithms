mod binary_tree;

use binary_tree::{BinaryTree, Comparator};

impl Comparator<i16> for i16 {
    fn compare(&self, to_compare: &Self) -> bool {
        self > to_compare
    }
}

fn main() {
    let mut binary_tree = BinaryTree::<i16>::new();
    binary_tree.add(2);
    binary_tree.add(3);
    binary_tree.add(-1);
    binary_tree.add(-4);

    println!("Hello, world!");
}

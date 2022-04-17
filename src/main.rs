mod binary_tree;

use binary_tree::BinaryTree;

fn main() {
    let mut binary_tree = BinaryTree::<i16>::new();
    binary_tree.add(2);
    binary_tree.add(3);
    binary_tree.add(-1);
    binary_tree.add(-4);

    println!("{:#?}", binary_tree);
    binary_tree.print();
    println!("{}", binary_tree.contains(1));
}

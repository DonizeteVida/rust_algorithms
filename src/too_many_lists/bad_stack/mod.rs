use std::mem::replace;

pub struct List<T> {
    head: Link<T>,
}

enum Link<T> {
    Empty,
    More(Box<Node<T>>),
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Node<T> {
        Node {
            elem,
            next: Link::Empty,
        }
    }
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node)
    }

    pub fn pop(&mut self) -> Option<T> {
        match replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_add_item() {}
}

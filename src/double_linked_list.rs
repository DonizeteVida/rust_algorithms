use std::{mem::replace, rc::Rc, cell::RefCell};

type Typo<T> = Option<Box<Rc<RefCell<Node<T>>>>>;

#[derive(Debug)]
struct DoubleLinkedList<T> {
    head: Typo<T>
}

#[derive(Debug)]
struct Node<T> {
    prev: Typo<T>,
    data: T,
    next: Typo<T>
}

impl <T> Node<T> {
    fn new(data: T) -> Self {
        Node { prev: None, data, next: None}
    }
}

impl <T> DoubleLinkedList<T> {
    fn new() -> Self {
        DoubleLinkedList { head: None }
    }

    fn push(&mut self, data: T) {
        let node = Box::new(Rc::new(RefCell::new(Node::new(data))));

        let head = match replace(&mut self.head, None) {
            Some(head) => {
                node.borrow_mut().next = Some(Box::new(Rc::clone(&head)));
                head.borrow_mut().prev = Some(Box::new(Rc::clone(&node)));
                Some(node)
            },
            None => Some(node),
        };

        self.head = head;
    }

    fn pop(&mut self) -> Option<T> {
        match replace(&mut self.head, None) {
            Some(head) => {
                let next = replace(&mut head.borrow_mut().next, None);
                if let Some(ref node) = next {
                    node.borrow_mut().prev = None;
                }
                self.head = next;
                if let Ok(ref_cell) = Rc::try_unwrap(*head) {
                    return Some(ref_cell.into_inner().data);
                }
                panic!("Data could not be returned");
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_insert_properly() {
        let mut double_linked_list = DoubleLinkedList::<u8>::new();
        double_linked_list.push(1);
        double_linked_list.push(2);
        println!("{:?}", double_linked_list);
    }

    #[test]
    fn should_pop_properly() {
        let mut double_linked_list = DoubleLinkedList::<u8>::new();
        double_linked_list.push(1);
        double_linked_list.push(2);
        double_linked_list.push(3);
        double_linked_list.push(4);
        assert_eq!(double_linked_list.pop(), Some(4));
        assert_eq!(double_linked_list.pop(), Some(3));
        assert_eq!(double_linked_list.pop(), Some(2));
        assert_eq!(double_linked_list.pop(), Some(1));
        assert_eq!(double_linked_list.pop(), None);
    }
}
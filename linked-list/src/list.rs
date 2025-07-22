use std::fmt::{Debug, Display};

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    current_size: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            current_size: 0,
        }
    }

    pub fn push(&mut self, data: T) -> Result<(), Box<dyn std::error::Error>> {
        let new_node = Node {
            data,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
        self.current_size += 1;
        Ok(())
    }

    pub fn append(&mut self, data: T) -> Result<(), Box<dyn std::error::Error>> {
        let new_node = Node { data, next: None };

        match &mut self.head {
            Some(_current) => {
                let mut current_node = self.head.as_mut().unwrap();
                while let Some(ref mut next_node) = current_node.next {
                    current_node = next_node;
                }

                // Insert the new node at the end of the linked list
                current_node.next = Some(Box::new(new_node));
            }
            None => {
                self.head = Some(Box::new(new_node));
            }
        }

        self.current_size += 1;

        Ok(())
    }
}

impl<T: Debug> Display for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut current_node = &self.head;
        let mut result = String::new();

        while let Some(node) = current_node {
            result.push_str(&format!("{:?} -> ", node.data));
            current_node = &node.next;
        }

        write!(f, "{}", result)
    }
}

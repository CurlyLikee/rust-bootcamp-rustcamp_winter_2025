use std::sync::{Arc, RwLock};


#[derive(Clone)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Arc<RwLock<Node<T>>>>,
    pub prev: Option<Arc<RwLock<Node<T>>>>,
}


pub struct LinkedList<T> {
    head: Option<Arc<RwLock<Node<T>>>>,
    tail: Option<Arc<RwLock<Node<T>>>>,
}

impl<T: Clone> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
        }
    }

    pub fn push_back(&mut self, value: T) {
        let new_node = Arc::new(RwLock::new(Node {
            value,
            next: None,
            prev: self.tail.clone(),
        }));

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.write().unwrap().next = Some(new_node.clone());
            }
            None => {
                self.head = Some(new_node.clone());
            }
        }

        self.tail = Some(new_node);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            let mut old_head = old_head.write().unwrap();
            let next_node = old_head.next.clone();
            self.head = next_node.clone();

            if let Some(new_head) = self.head.as_ref() {
                new_head.write().unwrap().prev = None;
            } else {
                self.tail = None;
            }
            old_head.value.clone()
        })
    }
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, RwLock};
    use std::thread;

    #[test]
    fn test_single_threaded() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
        assert!(list.is_empty());
    }

    #[test]
    fn test_multi_threaded() {
        let list = Arc::new(RwLock::new(LinkedList::new()));

        let list1 = list.clone();
        let handle1 = thread::spawn(move || {
            let mut list = list1.write().unwrap();
            for i in 0..50 {
                list.push_back(i);
            }
        });

        let list2 = list.clone();
        let handle2 = thread::spawn(move || {
            let mut list = list2.write().unwrap();
            for i in 50..100 {
                list.push_back(i);
            }
        });

        handle1.join().unwrap();
        handle2.join().unwrap();

        let mut list = list.write().unwrap();
        let mut values = vec![];
        while let Some(value) = list.pop_front() {
            values.push(value);
        }
        values.sort();
        assert_eq!(values, (0..100).collect::<Vec<_>>());
    }
}

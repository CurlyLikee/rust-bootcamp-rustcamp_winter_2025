use std::sync::{Arc, Mutex};


pub struct GlobalStack<T> {
    stack: Arc<Mutex<Vec<T>>>,
}



impl<T> GlobalStack<T> {
    pub fn new() -> Self {
        Self {
            stack: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn push(&self, value: T) {
        let mut stack = self.stack.lock().unwrap();
        stack.push(value);
    }

    pub fn pop(&self) -> Option<T> {
        let mut stack = self.stack.lock().unwrap();
        stack.pop()
    }

    pub fn size(&self) -> usize {
        let stack = self.stack.lock().unwrap();
        stack.len()
    }

    pub fn clear(&self) {
        let mut stack = self.stack.lock().unwrap();
        stack.clear();
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_push_pop() {
        let stack = Arc::new(GlobalStack::new());
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
    }



    #[test]
    fn test_size() {
        let stack = Arc::new(GlobalStack::new());
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.size(), 2);
        stack.pop();
        assert_eq!(stack.size(), 1);
    }




    #[test]
    fn test_clear() {
        let stack = Arc::new(GlobalStack::new());
        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack.clear();
        assert_eq!(stack.size(), 0);
    }




    #[test]
    fn test_multithreaded() {
        let stack = Arc::new(GlobalStack::new());
        let stack_clone = Arc::clone(&stack);
        let handle = thread::spawn(move || {
            stack_clone.push(1);
            stack_clone.push(2);
        });
        handle.join().unwrap();
        let stack_clone2 = Arc::clone(&stack);
        let handle2 = thread::spawn(move || {
            stack_clone2.push(3);
        });
        handle2.join().unwrap();
        assert_eq!(stack.size(), 3);
    }
}




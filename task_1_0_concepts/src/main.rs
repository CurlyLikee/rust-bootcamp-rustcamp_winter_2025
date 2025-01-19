use task_1_0::{LinkedList};


fn single() {
    let mut list = LinkedList::new();
    list.push_back(10);
    list.push_back(20);
    println!("{:?}", list.pop_front());
    println!("{:?}", list.pop_front());
    println!("{:?}", list.pop_front());
}



fn multi() {
    use std::sync::{Arc, RwLock};
    use std::thread;

    let list = Arc::new(RwLock::new(LinkedList::new()));
    let list1 = list.clone();
    let handle1 = thread::spawn(move || {
        let mut list = list1.write().unwrap();
        for i in 0..5 {
            list.push_back(i);
        }
    });

    let list2 = list.clone();
    let handle2 = thread::spawn(move || {
        let mut list = list2.write().unwrap();
        for i in 5..10 {
            list.push_back(i);
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
    let mut list = list.write().unwrap();
    while let Some(value) = list.pop_front() {
        println!("{}", value);
    }
}



fn main() {
    println!("Single-threaded example:");
    single();

    println!("\nMulti-threaded example:");
    multi();
}

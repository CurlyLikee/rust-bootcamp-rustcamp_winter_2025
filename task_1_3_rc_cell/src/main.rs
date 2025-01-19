use task_1_3::{GlobalStack};
use std::sync::Mutex;



fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    println!("{:?}", vec);
    vec.pop();
    println!("{:?}", vec);

    let data = Mutex::new(0);
    {
        let mut num = data.lock().unwrap();
        *num += 1;
    }

    println!("{:?}", data);
}



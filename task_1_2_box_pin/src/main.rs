use task_1_2::{SayHi, MutMeSomehow,MeasurableFuture};
use std::pin::Pin;
use std::rc::Rc;
use std::future::Future;
use std::task::Poll;
use std::time::Instant;
use tokio::time::Duration;





#[tokio::main]
async fn main() {
    let boxed = Box::new(42);
    Pin::new(&boxed).say_hi();

    let shared = Rc::new(123);
    Pin::new(&shared).say_hi();

    let vec = vec![1, 2, 3];
    Pin::new(&vec).say_hi();

    let string = String::from("Hello");
    Pin::new(&string).say_hi();

    let my_future = async {
        tokio::time::sleep(Duration::from_millis(500)).await;
        42
    };

    let measurable_future = MeasurableFuture::new(my_future);
    let result = measurable_future.await;
    println!("Future result: {}", result);
}




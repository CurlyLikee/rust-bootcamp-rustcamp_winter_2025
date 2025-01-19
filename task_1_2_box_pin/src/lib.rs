use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use std::{fmt, rc::Rc};
use pin_project::pin_project;



pub trait SayHi: fmt::Debug {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self);
    }
}

pub trait MutMeSomehow {
    fn mut_me_somehow(&mut self);
}



impl<T> SayHi for Box<T> where T: fmt::Debug {}

impl<T> MutMeSomehow for Box<T>
where
    T: Default,
{
    fn mut_me_somehow(&mut self) {
        *self = Box::new(T::default());
    }
}



impl<T> SayHi for Rc<T> where T: fmt::Debug {}

impl<T> MutMeSomehow for Rc<T>
where
    T: Default,
{
    fn mut_me_somehow(&mut self) {
        println!("Cannot mutate Rc directly");
    }
}





impl<T> SayHi for Vec<T> where T: fmt::Debug {}

impl<T> MutMeSomehow for Vec<T>
where
    T: Default,
{
    fn mut_me_somehow(&mut self) {
        self.push(T::default());
    }
}




impl SayHi for String {}

impl MutMeSomehow for String {
    fn mut_me_somehow(&mut self) {
        self.push('!');
    }
}



impl SayHi for &[u8] {}

impl MutMeSomehow for &[u8] {
    fn mut_me_somehow(&mut self) {
        println!("Cannot mutate &[u8] directly");
    }
}





#[pin_project]
pub struct MeasurableFuture<Fut> {
    #[pin]
    inner_future: Fut,
    started_at: Option<Instant>,
}

impl<Fut> MeasurableFuture<Fut> {
    pub fn new(inner_future: Fut) -> Self {
        Self {
            inner_future,
            started_at: None,
        }
    }
}

impl<Fut> std::future::Future for MeasurableFuture<Fut>
where
    Fut: std::future::Future,
{
    type Output = Fut::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();

        if this.started_at.is_none() {
            *this.started_at = Some(Instant::now());
        }

        match this.inner_future.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(output) => {
                if let Some(started_at) = this.started_at.take() {
                    let elapsed = started_at.elapsed();
                    println!(
                        "Future completed in {} nanoseconds",
                        elapsed.as_nanos()
                    );
                }
                Poll::Ready(output)
            }
        }
    }
}






#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Duration;

    #[tokio::test]
    async fn test_measurable_future() {
        let my_future = async {
            tokio::time::sleep(Duration::from_millis(500)).await;
            42
        };

        let measurable_future = MeasurableFuture::new(my_future);
        let result = measurable_future.await;
        assert_eq!(result, 42);
    }

    #[test]
    fn test_mut_me_somehow() {
        let mut boxed = Box::new(42);
        boxed.mut_me_somehow();
        assert_eq!(*boxed, 0);

        let mut vec = vec![1, 2, 3];
        vec.mut_me_somehow();
        assert_eq!(vec, vec![1, 2, 3, 0]);

        let mut string = String::from("Hello");
        string.mut_me_somehow();
        assert_eq!(string, "Hello!");
    }
}
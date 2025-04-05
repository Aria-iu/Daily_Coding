use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::task::JoinHandle;

pub struct CounterFuture {
    pub count: u32,
}

/*
state1:
        self.count +1
        print
        sleep
        wake_to_go_forward
state2:
        self.count +1
        print
        sleep
        wake_to_go_forward
state3:
        self.count +1
        print
        sleep
        wake_to_go_forward
state4:
        self.count +1
        print
        sleep
        wake_to_go_forward
state5:
        self.count +1
        print
        sleep

*/

impl Future for CounterFuture {
    type Output = u32;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        self.count += 1;
        println!("Polling with result : {}", self.count);
        // tokio::time::sleep(Duration::from_secs(1)); this is s future
        std::thread::sleep(Duration::from_secs(1));
        if self.count < 5 {
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(self.count)
        }
    }
}

pub mod executor;
pub mod waker;
pub mod receiver;
pub mod sleep;
pub mod sender;
//
// pub struct CountingFuture {
//     pub count: i32,
// }
// impl Future for CountingFuture {
//     type Output = i32;
//     fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>)
//             -> Poll<Self::Output> {
//         self.count += 1;
//         if self.count == 4 {
//             println!("CountingFuture is done!");
//             Poll::Ready(self.count)
//         } else {
//             cx.waker().wake_by_ref();
//             println!(
//                 "CountingFuture is not done yet! {}",
//                 self.count
//             );
//             Poll::Pending
//         }
//     }
// }


//
// fn main() {
//     let counter = CountingFuture { count: 0 };
//     let counter_two = CountingFuture { count: 0 };
//     let mut executor = executor::Executor::new();
//     let handle = executor.spawn(counter);
//     let _handle_two = executor.spawn(counter_two);
//     std::thread::spawn(move || {
//         loop {
//             executor.poll();
//         }
//     });
//     let result = handle.recv().unwrap();
//     println!("Result: {}", result);
// }
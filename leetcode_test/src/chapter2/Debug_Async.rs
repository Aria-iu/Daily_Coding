#![allow(unused)]

mod CounterFuture;

use reqwest::Error;
use serde::Deserialize;
use serde_json::json;
use std::path::PathBuf;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use std::time::Instant;
use tokio::fs::File as AsyncFile;
use tokio::io::AsyncReadExt;
use tokio::sync::watch;
use tokio::task::JoinHandle;
use tokio::time::{Duration, sleep};

async fn read_file(filename: &str) -> Result<String, std::io::Error> {
    let mut file = AsyncFile::open(&filename).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}

async fn watch_file_changes(tx: watch::Sender<bool>) {
    let path = PathBuf::from("data.txt");
    let mut last_modified = None;
    loop {
        if let Ok(metadata) = path.metadata() {
            let modified = metadata.modified().unwrap();

            if last_modified != Some(modified) {
                last_modified = Some(modified);
                let _ = tx.send(true);
            }
            sleep(Duration::from_millis(100)).await;
        }
    }
}

#[derive(Deserialize, Debug)]
struct Response {
    url: String,
    args: serde_json::Value,
}

async fn fetch_data(seconds: u64) -> Result<Response, Error> {
    let request_url = format!("https://httpbin.org/delay/{}", seconds);
    let response = reqwest::get(&request_url).await?;
    let delayed_response: Response = response.json().await?;
    Ok(delayed_response)
}

async fn calculate_last_login() {
    sleep(Duration::from_secs(1)).await;
    println!("Logged in 2 days ago");
}

use std::sync::{Arc,Mutex};
struct MyFuture {
    state : Arc<Mutex<MyFutureState>>,
}
struct MyFutureState {
    data: Option<Vec<u8>>,
    waker: Option<Waker>,
}

impl MyFuture {
    fn new() -> (Self,Arc<Mutex<MyFutureState>>) {
        let state = Arc::new(Mutex::new(
            MyFutureState{
                data: None,
                waker: None,
            }
        ));
        (MyFuture{state : state.clone()},state,)
    }
}

impl Future for MyFuture {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("MyFuture::poll!!");
        let mut state = self.state.lock().unwrap();

        if state.data.is_some() {
            let data = state.data.take().unwrap();
            Poll::Ready(String::from_utf8(data).unwrap())
        }else {
            state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

// #[tokio::main(flavor = "current_thread")] only 1 thread , std::thread::sleep really sleep , 相当于串行。
#[tokio::main]
async fn main() -> Result<(), Error> {
    // let url = "https://jsonplaceholder.typicode.com/posts/1";
    // let start_time = Instant::now();

    // let _ = reqwest::get(url).await?;

    // 2219 ms
    /*let first = reqwest::get(url);
    let second = reqwest::get(url);
    let third = reqwest::get(url);
    let fourth = reqwest::get(url);
    let first = first.await?;
    let second = second.await?;
    let third = third.await?;
    let fourth = fourth.await?;*/

    // 601 ms
    /*let (_, _, _, _) = tokio::join!(
        reqwest::get(url),
        reqwest::get(url),
        reqwest::get(url),
        reqwest::get(url),
    );*/

    // let (tx, mut rx) = watch::channel(false);
    // tokio::spawn(watch_file_changes(tx));
    // loop {
    //     // Wait for a change in the file
    //     let _ = rx.changed().await;
    //     // Read the file and print its contents to the console
    //     if let Ok(contents) = read_file("data.txt").await {
    //         println!("{}", contents);
    //     }
    // }

    // let elapsed_time = start_time.elapsed();
    // println!("Request took {} ms",elapsed_time.as_millis());
    // Ok(())

    // let start_time = Instant::now();
    // let data = fetch_data(5);
    // let data2 = fetch_data(5);
    // let time_since = calculate_last_login();
    // let (posts, _ , _) = tokio::join!(
    //                             data, time_since,data2
    //                                 );
    // let duration = start_time.elapsed();
    // println!("Fetched {:?}", posts);
    // println!("Time taken: {:?}", duration);

    // use crate::CounterFuture::*;
    // let start_time = Instant::now();
    // let counter_one = CounterFuture { count: 0 };
    // let counter_two = CounterFuture { count: 0 };
    // let handle_one: JoinHandle<u32> = tokio::task::spawn(async move { counter_one.await });
    // let handle_two: JoinHandle<u32> = tokio::task::spawn(async move { counter_two.await });
    // tokio::join!(handle_one, handle_two);
    // let duration = start_time.elapsed();
    // println!("Time taken: {:?}", duration);

    use tokio::sync::mpsc;
    use tokio::task;
    let (my_future, state) = MyFuture::new();
    let (tx, mut rx) = mpsc::channel::<()>(1);
    let task_handle = task::spawn(async {
        my_future.await
    });
    // tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    println!("spawning trigger task");
    let trigger_task = task::spawn(async move {
        rx.recv().await;
        let mut state = state.lock().unwrap();
        state.data = Some(b"Hello from the outside".to_vec());
        loop {
            if let Some(waker) = state.waker.take() {
                waker.wake();
                break;
            }
        }}
    );
    tx.send(()).await.unwrap();

    let outcome = task_handle.await.unwrap();
    println!("Task completed with outcome: {}", outcome);

    trigger_task.await.unwrap();
    Ok(())
}

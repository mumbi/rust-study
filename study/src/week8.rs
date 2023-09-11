use crate::commands::*;

pub fn week8_category() {
    let command_handler = CommandHandler::new("command", [
        Command::new("async await", async_await),
        Command::new("tokio", tokio),
        Command::new("task", task),
        Command::new("async channel", async_channel),
        Command::new("join", join),
        Command::new("select", select),
        Command::new("blocking", blocking),
        Command::new("pin", pin),
        Command::new("async trait", async_trait),
        Command::new("cancellation", cancellation),
        Command::new("philosopher", philosopher),
    ].into_iter());

    command_handler.handle();
}

use futures::executor::block_on;

async fn count_to(count: i32) {
    for i in 1..=count {
        println!("Count is: {i}!");
    }
}

async fn async_main(count: i32) {
    count_to(count).await;
}

fn async_await() {
    block_on(async_main(10));
}

use tokio::time;

async fn count_to_task(count: i32) {
    for i in 1..=count {
        println!("Count in task: {i}!");
        time::sleep(time::Duration::from_millis(5)).await;
    }
}

#[tokio::main]
async fn tokio() {
    let join_handle = tokio::spawn(count_to_task(10));
    // count_to_task(10).await;

    for i in 1..5 {
        println!("Main task: {i}");
        time::sleep(time::Duration::from_millis(5)).await;
    }

    join_handle.await;
}

use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::TcpStream;

async fn who_are_you(mut socket: TcpStream) {
    if let Err(e) = socket.write_all(b"Who are you?\n").await {
        println!("socket error: {e:?}");
        return;
    }

    let mut buf = vec![0; 1024];
    let reply = match socket.read(&mut buf).await {
        Ok(n) => {
            let name = std::str::from_utf8(&buf[..n]).unwrap().trim();
            format!("Thanks for dialing in, {name}!\n")
        }
        Err(e) => {
            println!("socket error: {e:?}");
            return;
        }
    };

    if let Err(e) = socket.write_all(reply.as_bytes()).await {
        println!("socket error: {e:?}");
    }

}

async fn task_result() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;
    println!("listening on port 6142");

    loop {
        let (mut socket, addr) = listener.accept().await?;

        println!("connection from {addr:?}");

        // tokio::spawn(async move {
        //     if let Err(e) = socket.write_all(b"Who are you?\n").await {
        //         println!("socket error: {e:?}");
        //         return;
        //     }

        //     let mut buf = vec![0; 1024];
        //     let reply = match socket.read(&mut buf).await {
        //         Ok(n) => {
        //             let name = std::str::from_utf8(&buf[..n]).unwrap().trim();
        //             format!("Thanks for dialing in, {name}!\n")
        //         }
        //         Err(e) => {
        //             println!("socket error: {e:?}");
        //             return;
        //         }
        //     };

        //     if let Err(e) = socket.write_all(reply.as_bytes()).await {
        //         println!("socket error: {e:?}");
        //     }
        // });
        tokio::spawn(who_are_you(socket));
    }
}

#[tokio::main]
async fn task() {
    task_result().await;
}

use tokio::sync::mpsc::{self, Receiver};

async fn ping_handler(mut input: Receiver<()>) {
    let mut count: usize = 0;

    while let Some(_) = input.recv().await {
        count += 1;
        println!("Received {count} pings so far.");
    }

    println!("ping_handler complete");
}

#[tokio::main]
async fn async_channel() {
    let (sender, receiver) = mpsc::channel(3);
    let ping_handler_task = tokio::spawn(ping_handler(receiver));
    for i in 0..10 {
        sender.send(()).await.expect("Failed to send ping.");
        println!("Sent {} pings so far.", i + 1);
    }

    drop(sender);
    ping_handler_task.await.expect("Something went wrong in ping handler task.");
}

use anyhow::Result;
use futures::{future, FutureExt};
use reqwest;
use std::borrow::BorrowMut;
use std::collections::HashMap;

async fn size_of_page(url: &str) -> Result<usize> {
    let resp = reqwest::get(url).await?;
    Ok(resp.text().await?.len())
}

#[tokio::main]
async fn join() {
    let urls: [&str; 4] = [
        "https://google.com",
        "https://httpbin.org/ip",
        "https://play.rust-lang.org/",
        "BAD_URL",
    ];
    let futures_iter = urls.into_iter().map(size_of_page);
    let results = future::join_all(futures_iter).await;
    let page_sizes_dict: HashMap<&str, Result<usize>> =
        urls.into_iter().zip(results.into_iter()).collect();
    println!("{:?}", page_sizes_dict);
}

use tokio::time::{sleep, Duration};

#[derive(Debug, PartialEq)]
enum Animal {
    Cat { name: String },
    Dog { name: String },
}

impl Drop for Animal {
    fn drop(&mut self) {
        match self {
            Animal::Cat { name } => println!("drop {}", name),
            Animal::Dog { name } => println!("drop {}", name)
        }
    }
}

async fn first_animal_to_finish_race(
    mut cat_rcv: Receiver<String>,
    mut dog_rcv: Receiver<String>,
) -> Option<Animal> {
    let timeout = sleep(Duration::from_millis(1));
    tokio::select! {
        cat_name = cat_rcv.recv() => {
            println!("cat");
            // sleep(Duration::from_millis(5000)).await;
            Some(Animal::Cat { name: cat_name? })
        },
        dog_name = dog_rcv.recv() => {
            println!("dog");
            // sleep(Duration::from_millis(1000)).await;
            Some(Animal::Dog { name: dog_name? })
        },
        _ = timeout => { 
            println!("timeout");
            None
        } 
    }
}

#[tokio::main]
async fn select() {
    let (cat_sender, cat_receiver) = mpsc::channel(32);
    let (dog_sender, dog_receiver) = mpsc::channel(32);
    tokio::spawn(async move {
        sleep(Duration::from_millis(500)).await;
        cat_sender
            .send(String::from("Felix"))
            .await
            .expect("Failed to send cat.");
    });
    tokio::spawn(async move {
        sleep(Duration::from_millis(50)).await;
        dog_sender
            .send(String::from("Rex"))
            .await
            .expect("Failed to send dog.");
    });

    let winner = first_animal_to_finish_race(cat_receiver, dog_receiver)
        .await;
        // .expect("Failed to receive winner");

    println!("Winner is {winner:?}");
}

use futures::future::join_all;
use std::time::Instant;

async fn sleep_ms(start: &Instant, id: u64, duration_ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(duration_ms));
    // tokio::time::sleep(std::time::Duration::from_millis(duration_ms)).await;
    println!(
        "future {id} slept for {duration_ms}ms, finished after {}ms",
        start.elapsed().as_millis()
    );
}

#[tokio::main(flavor = "current_thread")]
async fn blocking() {
    let start = Instant::now();
    let mut sleep_futures = (1..=10).map(|t| async move { 
        println!("{} start", t);
        sleep_ms(&start, t, t * 10).await;
        println!("{} end", t);
    });
    println!("map end");
    join_all(sleep_futures).await;
    // let s0 = sleep_futures.next();
    // s0.unwrap().await;

    // std::thread::sleep(std::time::Duration::from_millis(10000));
}

use tokio::sync::{oneshot};
use tokio::task::spawn;

// 작업 항목. 이 경우 지정된 시간 동안 절전 모드로 있다가
// `respond_on` 채널의 메시지로 응답합니다.
#[derive(Debug)]
struct Work {
    input: u32,
    respond_on: oneshot::Sender<u32>,
}

// 대기열에서 작업을 리슨하고 실행하는 worker입니다.
async fn worker(mut work_queue: mpsc::Receiver<Work>) {
    let mut iterations = 0;
    // let mut timeout_fut = sleep(Duration::from_millis(100));
    let mut timeout_fut;

    loop {
        timeout_fut = Box::pin(sleep(Duration::from_millis(100)));

        tokio::select! {
            _ = &mut timeout_fut => { println!("timeout") },
            Some(work) = work_queue.recv() => {
                sleep(Duration::from_millis(10)).await; // Pretend to work.
                work.respond_on
                    .send(work.input * 1000)
                    .expect("failed to send response");
                iterations += 1;
            }
            // TODO: 100ms마다 반복 횟수를 보고합니다.
        }
    }
}

// 작업을 요청하고 작업이 완료될 때까지 기다리는 요청자입니다.
async fn do_work(work_queue: &mpsc::Sender<Work>, input: u32) -> u32 {
    let (tx, rx) = oneshot::channel();
    work_queue
        .send(Work {
            input,
            respond_on: tx,
        })
        .await
        .expect("failed to send on work queue");
    rx.await.expect("failed waiting for response")
}

#[tokio::main]
async fn pin() {
    let (tx, rx) = mpsc::channel(10);
    spawn(worker(rx));
    for i in 0..100 {
        let resp = do_work(&tx, i).await;
        println!("work result for iteration {i}: {resp}");
    }
}

use async_trait::async_trait;

#[async_trait]
trait Sleeper {
    async fn sleep(&self);
}

struct FixedSleeper {
    sleep_ms: u64,
}

#[async_trait]
impl Sleeper for FixedSleeper {
    async fn sleep(&self) {
        sleep(Duration::from_millis(self.sleep_ms)).await;
    }
}

async fn run_all_sleepers_multiple_times(sleepers: Vec<Box<dyn Sleeper>>, n_times: usize) {
    for _ in 0..n_times {
        println!("running all sleepers..");
        for sleeper in &sleepers {
            let start = Instant::now();
            sleeper.sleep().await;
            println!("slept for {}ms", start.elapsed().as_millis());
        }
    }
}

#[tokio::main]
async fn async_trait() {
    let sleepers: Vec<Box<dyn Sleeper>> = vec![
        Box::new(FixedSleeper { sleep_ms: 50 }),
        Box::new(FixedSleeper { sleep_ms: 100 }),
    ];
    run_all_sleepers_multiple_times(sleepers, 5).await;
}

use std::io::{ErrorKind};
use tokio::io::{DuplexStream};

struct LinesReader {
    stream: DuplexStream,
}

impl LinesReader {
    fn new(stream: DuplexStream) -> Self {
        Self { stream }
    }

    async fn next(&mut self) -> io::Result<Option<String>> {
        let mut bytes = Vec::new();
        let mut buf = [0];
        while self.stream.read(&mut buf[..]).await? != 0 {
            bytes.push(buf[0]);
            if buf[0] == b'\n' {
                break;
            }
        }
        if bytes.is_empty() {
            return Ok(None)
        }
        let s = String::from_utf8(bytes)
            .map_err(|_| io::Error::new(ErrorKind::InvalidData, "not UTF-8"))?;
        Ok(Some(s))
    }
}

async fn slow_copy(source: String, mut dest: DuplexStream) -> std::io::Result<()> {
    for b in source.bytes() {
        dest.write_u8(b).await?;
        tokio::time::sleep(Duration::from_millis(10)).await
    }
    Ok(())
}

#[tokio::main]
async fn cancellation() {
    let (client, server) = tokio::io::duplex(5);
    let handle = tokio::spawn(slow_copy("hi\nthere\n".to_owned(), client));

    let mut lines = LinesReader::new(server);
    let mut interval = tokio::time::interval(Duration::from_millis(60));
    loop {
        tokio::select! {
            _ = interval.tick() => println!("tick!"),
            line = lines.next() => if let Some(l) = line.unwrap() {
                print!("{}", l)
            } else {
                break
            },
        }
    }
    handle.await.unwrap();
}

// Copyright 2023 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// ANCHOR: Philosopher
use std::sync::Arc;
use tokio::sync::mpsc::{Sender};
use tokio::sync::Mutex;

struct Fork;

struct Philosopher {
    name: String,
    // ANCHOR_END: Philosopher
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: Sender<String>,
}

// ANCHOR: Philosopher-think
impl Philosopher {
    async fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name)).await
            .unwrap();
    }
    // ANCHOR_END: Philosopher-think

    // ANCHOR: Philosopher-eat
    async fn eat(&self) {
        // Pick up forks...
        // ANCHOR_END: Philosopher-eat
        let _first_lock = self.left_fork.lock().await;
        // Add a delay before picking the second fork to allow the execution
        // to transfer to another task
        time::sleep(time::Duration::from_millis(1)).await;
        let _second_lock = self.right_fork.lock().await;

        // ANCHOR: Philosopher-eat-body
        println!("{} is eating...", &self.name);
        time::sleep(time::Duration::from_millis(5)).await;
        // ANCHOR_END: Philosopher-eat-body

        // The locks are dropped here
        // ANCHOR: Philosopher-eat-end
    }
}

static PHILOSOPHERS: &[&str] =
    &["Socrates", "Plato", "Aristotle", "Thales", "Pythagoras"];

#[tokio::main]
async fn philosopher() {
    // ANCHOR_END: Philosopher-eat-end
    // Create forks
    let mut forks = vec![];
    (0..PHILOSOPHERS.len()).for_each(|_| forks.push(Arc::new(Mutex::new(Fork))));

    // Create philosophers
    let (philosophers, mut rx) = {
        let mut philosophers = vec![];
        let (tx, rx) = mpsc::channel(10);
        for (i, name) in PHILOSOPHERS.iter().enumerate() {
            let mut left_fork = Arc::clone(&forks[i]);
            let mut right_fork = Arc::clone(&forks[(i + 1) % PHILOSOPHERS.len()]);
            // To avoid a deadlock, we have to break the symmetry
            // somewhere. This will swap the forks without deinitializing
            // either of them.
            if i  == 0 {
                std::mem::swap(&mut left_fork, &mut right_fork);
            }
            philosophers.push(Philosopher {
                name: name.to_string(),
                left_fork,
                right_fork,
                thoughts: tx.clone(),
            });
        }
        (philosophers, rx)
        // tx is dropped here, so we don't need to explicitly drop it later
    };

    // Make them think and eat
    for phil in philosophers {
        tokio::spawn(async move {
            for _ in 0..100 {
                phil.think().await;
                phil.eat().await;
            }
        });

    }

    // Output their thoughts
    while let Some(thought) = rx.recv().await {
        println!("Here is a thought: {thought}");
    }
}
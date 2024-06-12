use crossbeam::{select, unbounded};
use std::thread;

pub fn run_single_channel() {
    let (tx, rx) = unbounded();

    thread::spawn(move || {
        tx.send(42).unwrap();
    });

    select! {
        recv(rx) -> msg => println!("{:?}", msg),
    }
}

#[derive(Debug)]
enum ConnectivtyCheck {
    Ping,
    Pong,
    Pang,
}
pub fn run_two_way_channel() {
    let n_message = 3;
    let (requests_tx, requests_rx) = unbounded();
    let (responses_tx, responses_rx) = unbounded();

    thread::spawn(move || loop {
        match requests_rx.recv().unwrap() {
            ConnectivtyCheck::Pong => eprintln!("unexpected pong response"),
            ConnectivtyCheck::Ping => responses_tx.send(ConnectivtyCheck::Pong).unwrap(),
            ConnectivtyCheck::Pang => return,
        }
    });

    for _ in 0..n_message {
        requests_tx.send(ConnectivtyCheck::Ping).unwrap();
    }
    requests_tx.send(ConnectivtyCheck::Pang).unwrap();

    for _ in 0..n_message {
        select! {
            recv(responses_rx) -> msg => println!("{:?}", msg),
        }
    }
}

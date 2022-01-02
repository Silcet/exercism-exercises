use std::{
    collections::HashMap,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

type ThreadChannel = (Sender<HashMap<char, usize>>, Receiver<HashMap<char, usize>>);

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    println!("Workers: {} {:?}", worker_count, input);
    let (tx, rx): ThreadChannel = mpsc::channel();

    let mut channels: Vec<Sender<Option<String>>> = Vec::with_capacity(worker_count);
    let mut children = Vec::new();
    for _ in 0..worker_count {
        let (tx1, rx1) = mpsc::channel();
        channels.push(tx1);
        let thread_tx = tx.clone();

        let child = thread::spawn(|| word_counter(thread_tx, rx1));

        children.push(child);
    }

    let mut thread_counter = 0;
    for s in input.iter() {
        channels[thread_counter]
            .send(Some(s.to_string()))
            .expect("Failed to send str to thread");
        thread_counter += 1;
        if thread_counter >= worker_count {
            thread_counter = 0;
        }
    }

    for c in channels {
        c.send(None).unwrap();
    }

    let mut res = HashMap::new();

    for _ in 0..input.len() {
        let map = rx.recv().unwrap();
        for (k, v) in map {
            let value = res.entry(k).or_insert(0);
            *value += v;
        }
    }

    res
}

fn word_counter(response_tx: Sender<HashMap<char, usize>>, order_rx: Receiver<Option<String>>) {
    loop {
        let slice = order_rx.recv().expect("Failed to receive str in thread");
        if slice.is_none() {
            return;
        }

        let mut map = HashMap::new();

        for c in slice.unwrap().chars() {
            if c.is_alphabetic() {
                let value = map.entry(c.to_ascii_lowercase()).or_insert(0);
                *value += 1;
            }
        }

        response_tx.send(map).unwrap()
    }
}

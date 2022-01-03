use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let slice_size = input.len() / worker_count;
    let leftover = input.len() % worker_count;

    let map: Arc<RwLock<HashMap<char, usize>>> = Arc::new(RwLock::new(HashMap::new()));

    crossbeam::scope(|s| {
        let mut threads = Vec::new();
        for i in 1..=worker_count {
            let range = (
                slice_size * (i - 1),
                if i < worker_count {
                    slice_size * i
                } else {
                    slice_size * i + leftover
                },
            );

            let map = map.clone();

            threads.push(s.spawn(move |_| word_counter(&input[range.0..range.1], map)));
        }

        for thread in threads {
            thread.join().unwrap();
        }
    })
    .unwrap();

    let res = map.read().unwrap().clone();
    res
}

fn word_counter(input: &[&str], map: Arc<RwLock<HashMap<char, usize>>>) {
    let mut slice_map: HashMap<char, usize> = HashMap::new();
    for slice in input {
        for c in slice.chars().filter(|c| c.is_alphabetic()) {
            (*slice_map.entry(c.to_ascii_lowercase()).or_insert(0)) += 1;
        }
    }

    let mut temp_map = map.write().unwrap();
    for (k, v) in slice_map {
        (*temp_map.entry(k).or_insert(0)) += v;
    }
}

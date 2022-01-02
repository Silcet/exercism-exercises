use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let slice_size = input.len() / worker_count;
    let leftover = input.len() % worker_count;

    crossbeam::scope(|s| {
        (1..=worker_count)
            .map(|i| {
                (
                    slice_size * (i - 1),
                    if i < worker_count {
                        slice_size * i
                    } else {
                        slice_size * i + leftover
                    },
                )
            })
            .map(|(start, end)| s.spawn(move |_| word_counter(&input[start..end])))
            .map(|c| c.join().unwrap())
            .reduce(|mut map, slice| {
                for (k, v) in slice {
                    (*map.entry(k).or_insert(0)) += v;
                }
                map
            })
            .unwrap()
    })
    .unwrap()
}

fn word_counter(input: &[&str]) -> HashMap<char, usize> {
    let mut slice_map: HashMap<char, usize> = HashMap::new();
    for slice in input {
        for c in slice.chars().filter(|c| c.is_alphabetic()) {
            (*slice_map.entry(c.to_ascii_lowercase()).or_insert(0)) += 1;
        }
    }

    slice_map
}

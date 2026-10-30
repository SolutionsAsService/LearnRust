//! Lesson 09: fearless concurrency: threads, channels and shared state.

use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

/// Sum a big slice by splitting it across threads and sending partial sums over a channel.
fn parallel_sum(numbers: &[u64], workers: usize) -> u64 {
    let (tx, rx) = mpsc::channel();
    let chunk = numbers.len().div_ceil(workers.max(1)).max(1);
    thread::scope(|s| {
        for part in numbers.chunks(chunk) {
            let tx = tx.clone();
            s.spawn(move || tx.send(part.iter().sum::<u64>()).unwrap());
        }
    });
    drop(tx); // close the channel so the receiver's iterator ends
    rx.iter().sum()
}

/// Count words across documents with threads sharing one map behind Arc<Mutex<_>>.
fn word_counts(docs: Vec<String>) -> HashMap<String, usize> {
    let counts = Arc::new(Mutex::new(HashMap::new()));
    let handles: Vec<_> = docs
        .into_iter()
        .map(|doc| {
            let counts = Arc::clone(&counts);
            thread::spawn(move || {
                for word in doc.split_whitespace() {
                    *counts.lock().unwrap().entry(word.to_lowercase()).or_insert(0) += 1;
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    Arc::try_unwrap(counts).unwrap().into_inner().unwrap()
}

fn main() {
    let numbers: Vec<u64> = (1..=1_000_000).collect();
    println!("sum = {}", parallel_sum(&numbers, 8));

    let counts = word_counts(vec!["the cat sat".into(), "The dog sat".into(), "a cat".into()]);
    let mut top: Vec<_> = counts.into_iter().collect();
    top.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    println!("top words: {:?}", &top[..3]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parallel_sum_matches_the_formula() {
        let n: u64 = 10_000;
        let v: Vec<u64> = (1..=n).collect();
        assert_eq!(parallel_sum(&v, 7), n * (n + 1) / 2);
    }

    #[test]
    fn words_are_counted_across_threads() {
        let m = word_counts(vec!["a b".into(), "A".into()]);
        assert_eq!(m["a"], 2);
        assert_eq!(m["b"], 1);
    }
}

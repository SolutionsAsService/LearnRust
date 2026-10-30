//! Lesson 07: iterators and closures: a tiny log analyser.

use std::collections::HashMap;

const SAMPLE: &str = "\
INFO server started on 8080
WARN slow request /books 912ms
INFO GET /books 200
ERROR db timeout after 3000ms
INFO GET /books 200
WARN slow request /authors 1204ms
ERROR db timeout after 3000ms
INFO shutdown";

/// Count lines per level, e.g. {"INFO": 4, ...}.
fn count_levels(log: &str) -> HashMap<&str, usize> {
    log.lines()
        .filter_map(|l| l.split_whitespace().next())
        .fold(HashMap::new(), |mut m, level| {
            *m.entry(level).or_insert(0) += 1;
            m
        })
}

/// Pull every "<n>ms" duration out of WARN lines.
fn slow_durations(log: &str) -> Vec<u32> {
    log.lines()
        .filter(|l| l.starts_with("WARN"))
        .filter_map(|l| l.split_whitespace().last())
        .filter_map(|w| w.strip_suffix("ms"))
        .filter_map(|n| n.parse().ok())
        .collect()
}

/// A closure that captures `min` from its environment.
fn longer_than(min: usize) -> impl Fn(&&str) -> bool {
    move |line| line.len() > min
}

fn main() {
    let mut levels: Vec<_> = count_levels(SAMPLE).into_iter().collect();
    levels.sort();
    println!("levels: {levels:?}");
    let slow = slow_durations(SAMPLE);
    let avg = slow.iter().sum::<u32>() as f64 / slow.len().max(1) as f64;
    println!("slow requests: {slow:?}, average {avg:.0} ms");
    let long: Vec<&str> = SAMPLE.lines().filter(longer_than(25)).collect();
    println!("{} lines longer than 25 characters", long.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_each_level() {
        let m = count_levels(SAMPLE);
        assert_eq!((m["INFO"], m["WARN"], m["ERROR"]), (4, 2, 2));
    }

    #[test]
    fn extracts_slow_durations() {
        assert_eq!(slow_durations(SAMPLE), vec![912, 1204]);
    }

    #[test]
    fn closure_captures_its_threshold() {
        let f = longer_than(3);
        assert!(f(&"abcd"));
        assert!(!f(&"abc"));
    }
}

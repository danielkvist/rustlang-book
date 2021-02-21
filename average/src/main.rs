// Exercise:
// Given a list of integers, use a vector and return
// the mean (average value), median (when sorted), and
// mode (the value that occurs most often).
use std::collections::HashMap;

fn main() {
    let list = vec![5, 6, 5, 7, 7, 8, 9, 2, 1, 3, 4, 5, 5, 4];

    println!(
        "average: {}, median: {}, mode: {}",
        average(&list),
        median(&list),
        mode(&list)
    )
}

fn average(list: &Vec<i32>) -> i32 {
    let mut count = 0;
    for n in list {
        count += n;
    }

    count / list.len() as i32
}

fn median(list: &Vec<i32>) -> i32 {
    let mut copy = list.clone();
    copy.sort();
    copy[copy.len() / 2]
}

fn mode(list: &Vec<i32>) -> i32 {
    let mut repeated = HashMap::new();

    for n in list {
        let count = repeated.entry(n).or_insert(0);
        *count += 1;
    }

    *repeated
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of zero numbers")
}

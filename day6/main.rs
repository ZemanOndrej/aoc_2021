use std::fs;

extern crate queues;
use queues::*;

fn main() {
    //part1
    day6(80);
    //part2
    day6(256);
}

fn day6(days: u32) {
    let contents = fs::read_to_string("./input2.txt").unwrap();

    let mut arr = vec![0, 0, 0, 0, 0, 0, 0];
    for i in contents.split(",") {
        let num = i.trim().parse::<usize>().unwrap();
        arr[num] += 1;
    }

    eprintln!("arr = {:#?}", arr);

    let mut q: Queue<u128> = queue![];
    let mut q2: Queue<u128> = queue![];
    q2.add(0).unwrap();
    q2.add(0).unwrap();

    for i in arr {
        q.add(i).unwrap();
    }
    for _ in 0..days {
        let el = q.remove().unwrap();
        let el2 = q2.remove().unwrap();
        q.add(el + el2).unwrap();
        q2.add(el).unwrap();
    }

    let mut count = 0;
    while q.peek().is_ok() {
        count += q.remove().unwrap();
    }
    while q2.peek().is_ok() {
        count += q2.remove().unwrap();
    }

    eprintln!("count = {:#?}", count);
}

use std::{collections::HashMap, fs};
fn main() {
    part_1();
    part_2();
}

type Line = ((u32, u32), (u32, u32));

fn parse_boards(file: &str) -> Vec<Line> {
    let contents = fs::read_to_string(file).unwrap();
    let lines = contents.lines();

    let mut parsed_lines: Vec<Line> = Vec::new();

    for line in lines {
        let vec: Vec<u32> = line
            .replace(" -> ", ",")
            .split(",")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        parsed_lines.push(((vec[0], vec[1]), (vec[2], vec[3])))
    }

    return parsed_lines;
}

fn get_key(i: u32, j: u32) -> String {
    format!("{};{}", i, j)
}

fn part_1() {
    let lines = parse_boards("./input2.txt");
    let mut dict = HashMap::<String, u32>::new();
    for line in &lines {
        let p1 = &line.0;
        let p2 = &line.1;
        if p1.0 == p2.0 || p1.1 == p2.1 {
            traverse(p1, p2, &mut dict);
        }
    }
    eprintln!("counter = {:#?}", get_res(dict));
}

fn part_2() {
    let lines = parse_boards("./input2.txt");
    let mut dict = HashMap::<String, u32>::new();

    for line in &lines {
        traverse(&line.0, &line.1, &mut dict);
    }
    eprintln!("counter = {:#?}", get_res(dict));
}

fn get_res(dict: HashMap<String, u32>) -> i32 {
    let mut counter = 0;
    for (_, val) in dict {
        if val > 1 {
            counter += 1;
        }
    }
    counter
}

fn traverse(p1: &(u32, u32), p2: &(u32, u32), dict: &mut HashMap<String, u32>) {
    let mut s0 = p1.0;
    let mut s1 = p1.1;
    while s0 != p2.0 || s1 != p2.1 {
        let key = get_key(s0, s1);
        *dict.entry(key).or_insert(0) += 1;

        if s0 > p2.0 {
            s0 -= 1;
        } else if s0 < p2.0 {
            s0 += 1;
        }
        if s1 > p2.1 {
            s1 -= 1;
        } else if s1 < p2.1 {
            s1 += 1;
        }
    }
    let key = get_key(s0, s1);
    *dict.entry(key).or_insert(0) += 1;
}

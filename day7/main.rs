use std::fs;
fn main() {
    //part1
    let mut res = part1();
    if res == 353800 {
        eprintln!("res = {:#?} success", res);
    }
    //353800
    //part2
    res = part2();
    if res == 98119739 {
        eprintln!("res = {:#?} success", res);
    }
    //98119739
}

fn parse_board(file_name: &str) -> (usize, Vec<u32>) {
    let contents = fs::read_to_string(file_name).unwrap();
    let numbers: Vec<u32> = contents
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let max = *numbers.iter().max().unwrap() as usize;
    let mut arr: Vec<u32> = vec![0; max + 1];
    for i in numbers {
        arr[i as usize] += 1;
    }
    (max, arr)
}

fn part1() -> u32 {
    let (max, arr) = parse_board("./input2.txt");
    let mut min = u32::MAX;

    for j in 0..max {
        let mut count: u32 = 0;
        for i in 0..arr.len() {
            let index_dif = ((j - i) as i32).abs();
            count += arr[i] * index_dif as u32;
        }

        if min > count {
            min = count;
        }
    }

    return min;
}

fn part2() -> u32 {
    let (max, arr) = parse_board("./input2.txt");
    let mut min = u32::MAX;

    let mut difs: Vec<u32> = vec![0];
    for j in 0..max {
        let mut count: u32 = 0;
        for i in 0..arr.len() {
            let i_dif = ((j - i) as i32).abs() as usize;
            if difs.len() <= i_dif {
                for k in difs.len()..i_dif + 1 {
                    difs.push(difs[k - 1] + k as u32);
                }
            }
            count += arr[i] * difs[i_dif];
        }

        if min > count {
            min = count;
        }
    }

    return min;
}

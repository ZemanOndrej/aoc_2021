use itertools::Itertools;
use std::{collections::HashMap, fs};
fn main() {
    //part1
    let res = part1();
    eprintln!("res = {:#?}", res);
    //
    let res = part2();
    eprintln!("res = {:#?}", res);
    //part2
}

fn part1() -> u32 {
    let contents = fs::read_to_string("./input2.txt").unwrap();
    let lines = contents.lines();
    let mut dict = HashMap::<String, u32>::new();

    for line in lines {
        let parts: Vec<&str> = line.split("|").collect();
        let output = parts[1];
        print!("{}", output);

        let words: Vec<&str> = output.split(" ").collect();
        for word in words {
            let key = word.len();
            println!("{}{}", word, key);
            *dict.entry(key.to_string()).or_insert(0) += 1;
        }
    }

    let known_words = vec![2, 4, 3, 7];
    let mut count = 0;
    for k in known_words {
        println!("{}", k);
        count += dict.get(&k.to_string()).unwrap_or(&0);
    }

    count
}

fn sort_string(str: &str) -> String {
    str.chars().sorted().collect::<String>()
}

fn decode_numbers(words: Vec<&str>) -> HashMap<String, i32> {
    let one = words.iter().find(|x| x.len() == 2).unwrap();
    let four = words.iter().find(|x| x.len() == 4).unwrap();
    let seven = words.iter().find(|x| x.len() == 3).unwrap();
    let mut six_len: Vec<&&str> = words.iter().filter(|x| x.len() == 6).collect(); // 6, 9, 0
    let mut five_len: Vec<&&str> = words.iter().filter(|x| x.len() == 5).collect(); // 5,3,2
    let mut seven_four = seven.to_string();
    seven_four.push_str(four);

    let seven_four_arr: String = seven_four.chars().unique().collect();
    let mut nine = "";
    for i in &six_len {
        let mut count_not_in_seven_four = 0;

        for j in i.chars() {
            if seven_four_arr.find(|c| c == j).is_none() {
                count_not_in_seven_four += 1;
            }
        }
        if count_not_in_seven_four == 1 {
            nine = i;
            break;
        }
    }
    six_len.remove(six_len.iter().position(|x| x.to_string() == nine).unwrap());

    let mut six = "";

    let mut missing_six_char = '_';

    for i in &six_len {
        for j in one.chars() {
            if i.find(|c| c == j).is_none() {
                missing_six_char = j;
                six = i;
                break;
            }
        }
    }
    six_len.remove(six_len.iter().position(|x| x.to_string() == six).unwrap());
    let zero = six_len[0];

    let mut three = "";
    for i in &five_len {
        let res = one
            .chars()
            .into_iter()
            .map(|x| i.find(x))
            .all(|x| x.is_some());
        if res {
            three = i;
            break;
        }
    }
    five_len.remove(
        five_len
            .iter()
            .position(|x| x.to_string() == three)
            .unwrap(),
    );

    let mut five = "";
    for i in &five_len {
        if i.find(missing_six_char).is_none() {
            five = i;
            break;
        }
    }
    five_len.remove(five_len.iter().position(|x| x.to_string() == five).unwrap());

    let two = five_len[0];

    let numbers = HashMap::from([
        ("abcdefg".to_string(), 8),
        (sort_string(seven), 7),
        (sort_string(four), 4),
        (sort_string(one), 1),
        (sort_string(five), 5),
        (sort_string(two), 2),
        (sort_string(three), 3),
        (sort_string(nine), 9),
        (sort_string(six), 6),
        (sort_string(zero), 0),
    ]);

    numbers
}

fn part2() -> i32 {
    let contents = fs::read_to_string("./input2.txt").unwrap();
    let lines = contents.lines();

    let mut arr: Vec<i32> = vec![];

    for line in lines {
        let parts: Vec<&str> = line.split("|").collect();
        let output = parts[1];
        let numbers = decode_numbers(parts[0].trim().split(" ").collect::<Vec<&str>>());
        eprintln!("numbers = {:#?}", numbers);
        println!("{}", output);

        let words: Vec<&str> = output.trim().split(" ").collect();
        let mut num_builder: String = String::new();
        for word in words {
            let key = sort_string(word);
            println!("<{}>:<{}>", word, key);
            let s = numbers[&key].to_string();
            num_builder.push_str(&s);
        }

        arr.push(num_builder.parse::<i32>().unwrap());
        eprintln!("num_builder = {:#?}", num_builder);
    }

    arr.iter().sum()
}

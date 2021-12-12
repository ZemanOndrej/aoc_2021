use std::fs;
fn main() {
    part_1();
    part_2();
}

fn part_1<'a>() -> (Vec<&'a str>, Vec<&'a str>) {
    let contents = fs::read_to_string("./input2.txt").unwrap();
    let lines = contents.lines();
    let mut vec: Vec<u32> = Vec::new();
    let line_count = lines.collect::<Vec<&str>>().len() as u32;

    for ele in contents.lines().into_iter() {
        let char_count = ele.chars().count();
        for (i, c) in ele.chars().enumerate() {
            let number = c.to_digit(10).unwrap();
            if vec.len() < char_count {
                vec.push(number);
            } else {
                vec[i] = vec[i] + number;
            }
        }
    }
    let mut gamma_vec: Vec<&str> = Vec::new();
    let mut epsilon_vec: Vec<&str> = Vec::new();
    for num in vec {
        if num > line_count / 2 {
            gamma_vec.push("1");
            epsilon_vec.push("0")
        } else {
            gamma_vec.push("0");
            epsilon_vec.push("1")
        }
    }
    let gamma = usize::from_str_radix(&gamma_vec.join(""), 2).unwrap();
    let epsilon = usize::from_str_radix(&epsilon_vec.join(""), 2).unwrap();

    eprintln!("res_gam = {:#?}", gamma);
    eprintln!("res_eps = {:#?}", epsilon);
    eprintln!("res = {:#?}", epsilon * gamma);

    return (gamma_vec, epsilon_vec);
}

fn part_2() {
    let contents = fs::read_to_string("./input2.txt").unwrap();

    let most_common_bin = get_number(&contents, |one_count, zero_count| {
        if one_count == zero_count {
            return '1';
        }
        if one_count > zero_count {
            '1'
        } else {
            '0'
        }
    });
    eprintln!("most_common_bin = {:#?}", most_common_bin);
    let least_common_bin = get_number(&contents, |one_count, zero_count| {
        if one_count == zero_count {
            return '0';
        }
        if one_count > zero_count {
            '0'
        } else {
            '1'
        }
    });

    eprintln!("least_common_bin = {:#?}", least_common_bin);
    let most_common_number = usize::from_str_radix(most_common_bin, 2).unwrap();
    let least_common_number = usize::from_str_radix(least_common_bin, 2).unwrap();

    eprintln!("most_common_number = {:#?}", most_common_number);
    eprintln!("least_common_number = {:#?}", least_common_number);

    eprintln!("result = {:#?}", most_common_number * least_common_number);
}

fn get_number<'a, T>(contents: &'a String, func: T) -> &'a str
where
    T: Fn(u32, u32) -> char,
{
    let mut lines: Vec<&str> = contents.lines().collect();
    let mut index = 0;
    while lines.len() > 1 {
        let char_sum = lines.as_mut_slice().into_iter().fold(0, |sum, line| {
            sum + line.chars().nth(index).unwrap().to_digit(10).unwrap()
        });
        let bit_to_find = func(char_sum, lines.len() as u32 - char_sum);

        let (common, _): (Vec<&str>, Vec<&str>) = lines
            .into_iter()
            .partition(|x| x.chars().nth(index).unwrap() == bit_to_find);
        lines = common;
        index += 1;
    }
    lines[0]
}

//5410338

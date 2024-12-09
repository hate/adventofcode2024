use crate::helpers::get_input::*;

pub fn run() {
    let input = get_input("src/days/one/input.txt");

    let mut left_list = Vec::<i32>::new();
    let mut right_list = Vec::<i32>::new();

    for line in input.lines() {
        let mut values = line.split_whitespace();
        left_list.push(values.next().unwrap().parse::<i32>().unwrap());
        right_list.push(values.next().unwrap().parse::<i32>().unwrap());
    }

    left_list.sort();
    right_list.sort();

    let result = left_list
        .iter()
        .zip(right_list)
        .map(|(l, r)| (l - r).abs())
        .sum::<i32>();

    println!("{}", result);
}

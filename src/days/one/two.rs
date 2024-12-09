use crate::helpers::get_input::*;

pub fn run() {
    let input = get_input("src/days/one/input.txt");

    let mut left_list = Vec::<usize>::new();
    let mut right_list = Vec::<usize>::new();

    for line in input.lines() {
        let mut values = line.split_whitespace();
        left_list.push(values.next().unwrap().parse::<usize>().unwrap());
        right_list.push(values.next().unwrap().parse::<usize>().unwrap());
    }

    let result = left_list
        .iter()
        .map(|l| l * right_list.iter().filter(|r| &l == r).count())
        .sum::<usize>();

    println!("{}", result);
}

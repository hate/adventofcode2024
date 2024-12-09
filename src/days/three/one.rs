use crate::helpers::get_input::*;
use regex::Regex;

pub fn run() {
    let input = get_input("src/days/three/input.txt");

    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

    let sum = re
        .find_iter(&input)
        .filter_map(|m| {
            let capture = m.as_str();

            let numbers = capture[4..capture.len() - 1]
                .split(',')
                .filter_map(|n| n.parse().ok())
                .collect::<Vec<usize>>();

            if numbers.len() == 2 {
                Some(numbers[0] * numbers[1])
            } else {
                None
            }
        })
        .sum::<usize>();

    println!("{}", sum);
}

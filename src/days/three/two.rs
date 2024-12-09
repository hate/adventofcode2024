use crate::helpers::get_input::*;
use regex::Regex;

pub fn run() {
    let input = get_input("src/days/three/input.txt");

    let mul_re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();

    let mut sum = 0;
    let mut enabled = true;
    let mut pos = 0;

    while pos < input.len() {
        let next = [
            mul_re
                .find_at(&input, pos)
                .map(|m| (m.start(), "mul", m.as_str())),
            do_re
                .find_at(&input, pos)
                .map(|m| (m.start(), "do", m.as_str())),
            dont_re
                .find_at(&input, pos)
                .map(|m| (m.start(), "dont", m.as_str())),
        ]
        .iter()
        .filter_map(|&x| x)
        .min_by_key(|&(pos, _, _)| pos);

        match next {
            Some((new_pos, "mul", capture)) => {
                if enabled {
                    let numbers = capture[4..capture.len() - 1]
                        .split(',')
                        .filter_map(|n| n.parse::<usize>().ok())
                        .collect::<Vec<_>>();

                    if numbers.len() == 2 {
                        sum += numbers[0] * numbers[1];
                    }
                }
                pos = new_pos + capture.len();
            }
            Some((new_pos, "do", capture)) => {
                enabled = true;
                pos = new_pos + capture.len();
            }
            Some((new_pos, "dont", capture)) => {
                enabled = false;
                pos = new_pos + capture.len();
            }
            _ => break,
        }
    }

    println!("{}", sum);
}

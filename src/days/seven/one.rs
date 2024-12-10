use crate::helpers::get_input::*;

fn evaluate(numbers: &[i64], operators: &[char]) -> i64 {
    let mut result = numbers[0];

    for i in 0..operators.len() {
        match operators[i] {
            '+' => result += numbers[i + 1],
            '*' => result *= numbers[i + 1],
            _ => unreachable!(),
        }
    }

    result
}

fn try_combinations(numbers: &[i64], target: i64) -> bool {
    let number_of_operators = numbers.len() - 1;

    for i in 0..(1 << number_of_operators) {
        let mut current_ops = Vec::with_capacity(number_of_operators);

        for j in 0..number_of_operators {
            current_ops.push(if (i & (1 << j)) == 0 { '+' } else { '*' });
        }

        if evaluate(numbers, &current_ops) == target {
            return true;
        }
    }

    false
}

pub fn run() {
    let input = get_input("src/days/seven/input.txt");
    let mut sum = 0;

    for line in input.lines() {
        let mut line = line.split(": ");

        let target = line.next().unwrap().parse::<i64>().unwrap();

        let numbers = line
            .next()
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect::<Vec<i64>>();

        if try_combinations(&numbers, target) {
            sum += target;
        }
    }

    println!("{}", sum);
}

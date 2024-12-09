use crate::helpers::get_input::*;

// A report only counts as safe if both of the following are true:

// The levels are either all increasing or all decreasing.
// Any two adjacent levels differ by at least one and at most three.

pub fn run() {
    let input = get_input("src/days/two/input.txt");

    let mut safe_reports = 0;

    'outer: for line in input.lines() {
        let values = line
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let mut direction: Option<bool> = None;

        for pair in values.windows(2) {
            if direction.is_none() {
                direction = Some(pair[0] < pair[1]);
            }

            if (pair[0] < pair[1]) != direction.unwrap() {
                continue 'outer;
            }

            let diff = pair[0].abs_diff(pair[1]);

            if diff > 3 || diff < 1 {
                continue 'outer;
            }
        }

        safe_reports += 1;
    }

    println!("{}", safe_reports);
}

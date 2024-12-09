use crate::helpers::get_input::*;

fn is_sequence_safe(values: &[usize]) -> bool {
    let mut direction: Option<bool> = None;

    for pair in values.windows(2) {
        if direction.is_none() {
            direction = Some(pair[0] < pair[1]);
        }

        if (pair[0] < pair[1]) != direction.unwrap() {
            return false;
        }

        let diff = pair[0].abs_diff(pair[1]);
        if diff > 3 || diff < 1 {
            return false;
        }
    }

    true
}

pub fn run() {
    let input = get_input("src/days/two/input.txt");
    let mut safe_reports = 0;

    for line in input.lines() {
        let values = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect::<Vec<usize>>();

        if is_sequence_safe(&values) {
            safe_reports += 1;
            continue;
        }

        for skip_idx in 0..values.len() {
            let filtered = values
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != skip_idx)
                .map(|(_, v)| *v)
                .collect::<Vec<usize>>();

            if is_sequence_safe(&filtered) {
                safe_reports += 1;
                break;
            }
        }
    }

    println!("{}", safe_reports);
}

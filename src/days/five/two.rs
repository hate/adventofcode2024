use crate::helpers::get_input::*;

fn is_valid_order(pages: &[i32], rules: &[(i32, i32)]) -> bool {
    for &(before, after) in rules {
        if let (Some(pos1), Some(pos2)) = (
            pages.iter().position(|&x| x == before),
            pages.iter().position(|&x| x == after),
        ) {
            if pos1 > pos2 {
                return false;
            }
        }
    }
    true
}

fn compare_pages(a: &i32, b: &i32, rules: &[(i32, i32)]) -> std::cmp::Ordering {
    for &(before, after) in rules {
        if *a == before && *b == after {
            return std::cmp::Ordering::Less;
        }
        if *a == after && *b == before {
            return std::cmp::Ordering::Greater;
        }
    }
    a.cmp(b)
}

pub fn run() {
    let input = get_input("src/days/five/input.txt");
    let mut parts = input.split("\n\n");

    let mut result = 0;
    let mut rules = Vec::new();

    for line in parts.next().unwrap().lines() {
        let mut numbers = line
            .split('|')
            .filter_map(|part| part.trim().parse::<i32>().ok())
            .take(2);

        if let (Some(a), Some(b)) = (numbers.next(), numbers.next()) {
            rules.push((a, b));
        }
    }

    for line in parts.next().unwrap().lines() {
        let mut update = line
            .split(',')
            .filter_map(|n| n.parse::<i32>().ok())
            .collect::<Vec<i32>>();

        if !is_valid_order(&update, &rules) {
            update.sort_by(|a, b| compare_pages(a, b, &rules));
            result += update[update.len() / 2];
        }
    }

    println!("{}", result);
}

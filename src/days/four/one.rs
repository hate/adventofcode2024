use crate::helpers::get_input::*;

const DIRECTIONS: [(i8, i8); 8] = [
    (0, 1),   // right
    (1, 0),   // down
    (0, -1),  // left
    (-1, 0),  // up
    (1, 1),   // down-right
    (-1, 1),  // up-right
    (1, -1),  // down-left
    (-1, -1), // up-left
];

fn check_word(grid: &[Vec<char>], row: usize, col: usize, dir: (i8, i8)) -> bool {
    let word = "XMAS";
    let length = grid.len() as i32;

    for (i, c) in word.chars().enumerate() {
        let current_row = row as i32 + (dir.0 as i32 * i as i32);
        let current_col = col as i32 + (dir.1 as i32 * i as i32);

        if current_row < 0 || current_row >= length || current_col < 0 || current_col >= length {
            return false;
        }

        if grid[current_row as usize][current_col as usize] != c {
            return false;
        }
    }

    true
}

pub fn run() {
    let input = get_input("src/days/four/input.txt");

    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut count = 0;
    let height = grid.len();
    let width = grid[0].len();

    for row in 0..height {
        for col in 0..width {
            for &dir in &DIRECTIONS {
                if check_word(&grid, row, col, dir) {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}

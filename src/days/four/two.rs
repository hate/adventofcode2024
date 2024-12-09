use crate::helpers::get_input::*;

const CORNERS: [((i8, i8), (i8, i8)); 2] = [((1, -1), (-1, 1)), ((-1, -1), (1, 1))];

fn check_shape(
    grid: &[Vec<char>],
    row: usize,
    col: usize,
    corners: [((i8, i8), (i8, i8)); 2],
) -> bool {
    let chars = ['M', 'S'];
    let length = grid.len() as i32;

    for (c1, c2) in corners {
        let (row1, col1) = (row as i32 + c1.0 as i32, col as i32 + c1.1 as i32);
        let (row2, col2) = (row as i32 + c2.0 as i32, col as i32 + c2.1 as i32);

        if row1 < 0
            || row1 >= length
            || col1 < 0
            || col1 >= length
            || row2 < 0
            || row2 >= length
            || col2 < 0
            || col2 >= length
        {
            return false;
        }

        if !(grid[row1 as usize][col1 as usize] == chars[0]
            && grid[row2 as usize][col2 as usize] == chars[1])
            && !(grid[row1 as usize][col1 as usize] == chars[1]
                && grid[row2 as usize][col2 as usize] == chars[0])
        {
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
            if grid[row][col] == 'A' && check_shape(&grid, row, col, CORNERS) {
                count += 1;
            }
        }
    }

    println!("{}", count);
}

use crate::helpers::get_input::*;

const WALL: char = '#';
const EMPTY: char = '.';
const VISITED: char = 'X';

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn is_out_of_bounds(grid: &Vec<Vec<char>>, pos: (i32, i32)) -> bool {
    let length = grid.len() as i32;
    let pos = (pos.0 as i32, pos.1 as i32);

    pos.0 < 0 || pos.1 < 0 || pos.0 >= length || pos.1 >= length
}

fn get_next_dir(current_dir: (i32, i32)) -> (i32, i32) {
    let index = DIRECTIONS
        .iter()
        .position(|&dir| dir == current_dir)
        .unwrap();

    DIRECTIONS[(index + 1) % DIRECTIONS.len()]
}

fn count_guard_path(grid: &mut Vec<Vec<char>>, start_pos: (usize, usize)) -> i32 {
    let mut seen = 0;
    let mut current_dir = DIRECTIONS[0];
    let mut current_pos = (start_pos.0 as i32, start_pos.1 as i32);

    grid[start_pos.0][start_pos.1] = EMPTY;

    while !is_out_of_bounds(&grid, current_pos) {
        if grid[current_pos.0 as usize][current_pos.1 as usize] == EMPTY {
            seen += 1;
            grid[current_pos.0 as usize][current_pos.1 as usize] = VISITED;
        }

        let next_pos = (current_pos.0 + current_dir.0, current_pos.1 + current_dir.1);

        if !is_out_of_bounds(&grid, next_pos)
            && grid[next_pos.0 as usize][next_pos.1 as usize] == WALL
        {
            current_dir = get_next_dir(current_dir);
        } else {
            current_pos = (next_pos.0 as i32, next_pos.1 as i32);
        }
    }

    seen
}

pub fn run() {
    let input = get_input("src/days/six/input.txt");

    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut guard_pos = (0, 0);
    'outer: for (row, line) in grid.iter().enumerate() {
        for (col, &c) in line.iter().enumerate() {
            if c == '^' {
                guard_pos = (row, col);
                break 'outer;
            }
        }
    }

    let result = count_guard_path(&mut grid, guard_pos);

    println!("{}", result);
}

use std::io::{BufRead, BufReader};
use std::fs::File;

#[macro_use]
mod applog;
mod startup;

/*------------------------------------------------------------------- main - */

fn main() {
    let reader = startup::get_reader().unwrap();

    if startup::is_part1() {  
        part1(reader);
    } else {
        part2(reader);
    }  

    applog::end_timestamp(startup::get_start_time());
}

/*------------------------------------------------------------------ part1 - */

fn part1(reader: BufReader<File>) {

    let mut grid: Vec<Vec<u32>> = vec![];
    read_grid(reader, &mut grid);

    let row_size: usize = grid.len();
    let col_size: usize = grid[0].len();
    
    let mut tall_trees: i32 = 0;
    for r in 0..row_size {
        for c in 0..col_size {
            if is_bigger_than_neighbours(r, c, &grid) {
                //applog!("Cell ({},{}) bigger than neighbours.", r, c);
                tall_trees += 1;
            }
        }
    }

    applog!("Number of tall trees: {}", tall_trees);
}

/*------------------------------------------------------------------ part2 - */

fn part2(reader: BufReader<File>) {

    let mut grid: Vec<Vec<u32>> = vec![];
    read_grid(reader, &mut grid);

    let row_size: usize = grid.len();
    let col_size: usize = grid[0].len();
    
    let mut max_scenic_value: i32 = 0;
    for r in 0..row_size {
        for c in 0..col_size {
            let scenic_value = compute_scenic_value(r, c, &grid);
            if scenic_value > max_scenic_value {
                applog!("Cell ({},{}) is new max scenic value: {}.", r, c, scenic_value);
                max_scenic_value = scenic_value;
            }
        }
    }

    applog!("Max scenic value: {}", max_scenic_value);
}

/*-------------------------------------------------------------- read_grid - */

fn read_grid(reader: BufReader<File>, grid: &mut Vec<Vec<u32>>) {

    // Read the file
    for line in reader.lines() {
        let line = line.unwrap();
        let mut vec: Vec<u32> = vec![];
        for t in line.chars() {
            let tree_value: u32 = t.to_digit(10).unwrap();
            vec.push(tree_value);
        }
        grid.push(vec);
    }
    
    //applog!("Grid: {:?}", grid);
}

/*---------------------------------------------- is_bigger_than_neighbours - */

fn is_bigger_than_neighbours(row: usize, column: usize, grid: &Vec<Vec<u32>>) -> bool {

    let max_row: usize = grid.len()-1;
    let max_col: usize = grid[0].len()-1; 

    // First check if perimeter tree
    if row==0 || row==max_row {
        return true;
    } else if column==0 || column==max_col {
        return true;
    }

    // Inner trees get here
    let h = grid[row][column]; // height of our tree

    // 1. Check trees above: keep c static and reduce down r to zero
    let mut r = row-1;
    let mut c = column;
    loop {
        if grid[r][c] >= h {
            break;
        }
        if r == 0 {
            return true; // got to edge of grid and still the tallest
        }
        r -= 1;
    }

    // 2. Check trees below: keep c static and increase r to max
    r = row+1;
    c = column;
    loop {
        if grid[r][c] >= h {
            break;
        }
        if r == max_row {
            return true; // got to edge of grid and still the tallest
        }
        r += 1;
    }

    // 3. Check trees to the left: keep r static and reduce c to zero
    r = row;
    c = column-1;
    loop {
        if grid[r][c] >= h {
            break;
        }
        if c == 0 {
            return true; // got to edge of grid and still the tallest
        }
        c -= 1;
    }

    // 4. Check trees to the right: keep r static and increase c to max
    r = row;
    c = column+1;
    loop {
        if grid[r][c] >= h {
            break;
        }
        if c == max_col {
            return true; // got to edge of grid and still the tallest
        }
        c += 1;
    }

    return false; // No line of sight out of the grid
}

/*---------------------------------------------- compute_scenic_value - */

fn compute_scenic_value(row: usize, column: usize, grid: &Vec<Vec<u32>>) -> i32 {

    let max_row: usize = grid.len()-1;
    let max_col: usize = grid[0].len()-1; 

    // Edge trees have no scenic value (as one of factors is zero)
    if row==0 || row==max_row || column==0 || column==max_col {
        return 0;
    }

    let h = grid[row][column]; // height of our tree

    let mut scenic_value: i32 = 1;

    // 1. Check trees above: keep c static and reduce down r to zero
    let mut r = row;
    let mut c = column;
    let mut factor: i32 = 0;
    loop {
        r -= 1;
        factor += 1;
        if grid[r][c] >= h {
            break;
        }
        if r == 0 {
            break;
        }
    }
    scenic_value *= factor;

    // 2. Check trees below: keep c static and increase r to max
    r = row;
    c = column;
    factor = 0;
    loop {
        r += 1;
        factor += 1;
        if grid[r][c] >= h {
            break;
        }
        if r == max_row {
            break;
        }
    }
    scenic_value *= factor;

    // 3. Check trees to the left: keep r static and reduce c to zero
    r = row;
    c = column;
    factor = 0;
    loop {
        c -= 1;
        factor += 1;
        if grid[r][c] >= h {
            break;
        }
        if c == 0 {
            break;
        }
    }
    scenic_value *= factor;

    // 4. Check trees to the right: keep r static and increase c to max
    r = row;
    c = column;
    factor = 0;
    loop {
        c += 1;
        factor += 1;
        if grid[r][c] >= h {
            break;
        }
        if c == max_col {
            break;
        }
    }
    scenic_value *= factor;

    return scenic_value;
}

/*--------------------------------------------------------- End of main.rs - */
use std::io::{BufRead, BufReader};
use std::fs::File;

#[macro_use]
mod applog;
mod startup;

#[derive(Debug, Clone)]
struct Attempt {
    moves: Vec<char>,
    options_remaining: Vec<u32>,
    success: bool
}
impl Default for Attempt {
    fn default () -> Attempt {
        Attempt {
            moves: vec![], 
            options_remaining: vec![],
            success: false
        }
    }
}

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

    // Read input grid
    let mut reference_grid: Vec<Vec<u8>> = vec![];
    let (s_row, s_col, e_row, e_col) = read_grid(reader, &mut reference_grid);

    // Keep traversing maze until we find the end position
    let mut traversal_attempts: Vec<Attempt> = vec![];
    let mut some_movement = true;

    let mut least_moves_so_far = 9999999;

    while some_movement {
        some_movement = false;

        let mut grid: Vec<Vec<u8>> = reference_grid.clone();
        let (mut r, mut c) = (s_row, s_col); // start pos
        let mut attempt: Attempt = Attempt::default();

        loop {
            // Traverse grid until we find end
            let (success, n_r, n_c) = find_next_step(&grid, r, c, &mut traversal_attempts, &mut attempt);
            if !success {
                break;
            }
            some_movement = true;

            if startup::is_debug() {
                applog!("({},{}) --> ({},{})", r, c, n_r, n_c);
            }

            // Overwrite current cell, so we don't go around in circles
            grid[r][c] = b'.';
            (r, c) = (n_r, n_c);

            // Found the end yet?

            if (r, c) == (e_row, e_col) {
                attempt.success = true;
                break;
            }
        }

        traversal_attempts.push(attempt.clone());
        if attempt.success {
            if least_moves_so_far > attempt.moves.len() {
                least_moves_so_far = attempt.moves.len();
                applog!("Least moves so far: {}", least_moves_so_far);
                applog!("${:?}", attempt);
            }
        }
    }

    applog!("{:?}", traversal_attempts);

    for attempt in &traversal_attempts {
        if attempt.success {
            applog!("Number of steps required: {}", attempt.moves.len());
        }
    }
}

/*------------------------------------------------------------------ part2 - */

fn part2(reader: BufReader<File>) {

    // Read the file
    for line in reader.lines() {
        let _line = line.unwrap();
    }

    applog!("Not yet implemented.");
}

/*-------------------------------------------------------------- read_grid - */

fn read_grid(reader: BufReader<File>, grid: &mut Vec<Vec<u8>>) -> (usize, usize, usize, usize) {

    let (mut s_row, mut s_col, mut e_row, mut e_col): (usize, usize, usize, usize) = (0, 0, 0, 0);

    // Read the file
    for (r, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut vec: Vec<u8> = vec![];
        for (c, t) in line.chars().enumerate() {
            if t == 'S' {
                s_row = r;
                s_col = c;
            } else if t == 'E' {
                e_row = r;
                e_col = c;
            }

            vec.push(t as u8);
        }
        grid.push(vec);
    }

    if startup::is_debug() {
        let row_size: usize = grid.len();
        let col_size: usize = grid[0].len();

        applog!("Read grid of {}x{}, S=({},{}), E=({},{})", 
            row_size, col_size, 
            s_row, s_col,
            e_row, e_col);
        
        for r in 0..row_size {
            applog!("{:?}", grid[r]);
        }
    }

    return (s_row, s_col, e_row, e_col);
}

/*--------------------------------------------------------- find_next_step - */

fn find_next_step(grid: &Vec<Vec<u8>>, row: usize, col: usize, traversal_attempts: &mut Vec<Attempt>, attempt: &mut Attempt) -> (bool, usize, usize) {

    let c = if grid[row][col] == b'S' {b'a'} else {grid[row][col]};
    
    // Limits
    let (max_r, max_c): (usize, usize) = (grid.len()-1, grid[0].len()-1);

    // Adjacent cells:
    let a_n: u8 = if row>0 {grid[row-1][col]} else {0};
    let a_s: u8 = if row<max_r {grid[row+1][col]} else {0};
    let a_e: u8 = if col<max_c {grid[row][col+1]} else {0};
    let a_w: u8 = if col>0 {grid[row][col-1]} else {0};

    // Determine which moves are possible:
    let mut possible: u32 = 
        if is_next_cell_possible(c, a_n) {0b1000} else {0} +
        if is_next_cell_possible(c, a_s) {0b0100} else {0} +
        if is_next_cell_possible(c, a_e) {0b0010} else {0} +
        if is_next_cell_possible(c, a_w) {0b0001} else {0};

    // Filter out invalid outcomes
    if possible == 0b0000 {
        if startup::is_debug() {
            applog!("No moves possible from ({},{})", row, col);
        }
        return (false, 0, 0);
    }
    if possible == 0b1111 {
        applog!("possible: {}, c:{}, n:{}, s:{}, e:{}, w:{}, grid: {:?}", possible, c, a_n, a_s, a_e, a_w, grid);
        panic!("Four moves possible from ({},{})", row, col);
    }

    let mut direction: char = get_best_option(grid, row, col, &mut possible, traversal_attempts, attempt);
    if direction == '\0' && possible>0 {
        if possible & 0b1000 > 0 { direction = 'N'; }
        if possible & 0b0100 > 0 { direction = 'S'; }
        if possible & 0b0010 > 0 { direction = 'E'; }
        if possible & 0b0001 > 0 { direction = 'W'; }
    }

    let (mut row_next, mut col_next): (usize, usize) = (0, 0);
    if direction!='\0' {
        (row_next, col_next) = match direction {
            'N' => (row-1, col),
            'S' => (row+1, col),
            'E' => (row, col+1),
            'W' => (row, col-1),
            _ => panic!("Invalid direction: {}", direction),
        };

        if possible>0 {
            // Remove chosen direction from bitmap of possible directions
            possible &= 0b1111 ^ match direction {
                'N' => 0b1000,
                'S' => 0b0100,
                'E' => 0b0010,
                'W' => 0b0001,
                _ => panic!("Invalid direction: {}", direction),
            };
        }

        attempt.moves.push(direction);
        attempt.options_remaining.push(possible);
    }

    return (direction!='\0', row_next, col_next);
}

/*--------------------------------------------------- is_next_cell_possible - */

fn is_next_cell_possible(c: u8, c_next: u8) -> bool {
    
    if c_next == b'.' || c_next == 0 {
        return false;
    }

    if (c == b'y' || c == b'z') && c_next == b'E' {
        return true;
    }
    
    if c_next <= (c+1) {
        return true;
    }
    
    return false;
}

/*-------------------------------------------------------- get_best_option - */

fn get_best_option(grid: &Vec<Vec<u8>>, row: usize, col: usize, possible: &mut u32, traversal_attempts: &mut Vec<Attempt>, attempt: & Attempt) -> char {

    let (mut n, mut s, mut e, mut w): (u8, u8, u8, u8) = (0, 0, 0, 0);
    if *possible & 0b1000 > 0 { n = grid[row-1][col]; }
    if *possible & 0b0100 > 0 { s = grid[row+1][col]; }
    if *possible & 0b0010 > 0 { e = grid[row][col+1]; }
    if *possible & 0b0001 > 0 { w = grid[row][col-1]; }

    // 'E' = automatic win
    let c = grid[row][col];
    if (c == b'y' || c == b'z') && n == b'E' {
        applog!("c={}", c); return 'N';}
    if (c == b'y' || c == b'z') && s == b'E' {return 'S';}
    if (c == b'y' || c == b'z') && e == b'E' {return 'E';}
    if (c == b'y' || c == b'z') && w == b'E' {return 'W';}

    // Find matching previous attempts
    let (previous_attempts_match, next_move): (bool, u32) = get_next_direction_from_previous_attempts(traversal_attempts, attempt);

    // Disable options ruled out by next_move
    if next_move > 0 {
        if next_move & 0b1000 == 0 { n=0;} 
        if next_move & 0b0100 == 0 { s=0;} 
        if next_move & 0b0010 == 0 { e=0;}
        if next_move & 0b0001 == 0 { w=0;} 

        // we can zero alternate possibilities for this move and this attempt
        // as the previous attempt we found should already cover all the possibilities
        *possible = 0;
    } else if previous_attempts_match {
        // Also wipe here - as we've been here before
        *possible = 0;
    }

    // Prefer higher targets
    if n!=0 && (n < s || n < e || n < w) {n=0;}
    if s!=0 && (s < n || s < e || n < w) {s=0;}
    if e!=0 && (e < n || e < s || n < w) {e=0;}
    if w!=0 && (w < n || w < s || n < e) {w=0;}

    // Try routes in NSEW order
    let direction: char = 
        if n!=0 {'N'}
        else if s!=0 {'S'}
        else if e!=0 {'E'}
        else if w!=0 {'W'}
        else {'\0'};

    return direction;
}

/*------------------------------ get_next_direction_from_previous_attempts - */

fn get_next_direction_from_previous_attempts(traversal_attempts: &mut Vec<Attempt>, attempt: & Attempt) -> (bool, u32) {

    let mut previous_attempts_match: bool = false;
    let mut next_move: u32 = 0;

    let mut attempt_index: usize = 0;
    let max_attempts = traversal_attempts.len();
    let attempt_len = attempt.moves.len();

    while attempt_index < max_attempts {
        let previous = &traversal_attempts[attempt_index];
        if do_attempts_match(&previous, attempt) {
            previous_attempts_match = true;
            if do_possibilities_remain(&previous, attempt_len+1) {
                // Go with previous move for this attempt_index
                // - as we know there are other possibilities to explore ahead
                next_move =  match previous.moves[attempt_len] {
                    'N' => 0b1000,
                    'S' => 0b0100,
                    'E' => 0b0010,
                    'W' => 0b0001,
                    _ => panic!("Invalid direction."),
                };
            } else {
                let options_remaining = previous.options_remaining[attempt_len];

                // Cycle through options remaining in NSEW order
                if options_remaining>0 {
                    if options_remaining & 0b1000 > 0 { next_move = 0b1000; }
                    if options_remaining & 0b0100 > 0 { next_move = 0b0100; }
                    if options_remaining & 0b0010 > 0 { next_move = 0b0010; }
                    if options_remaining & 0b0001 > 0 { next_move = 0b0001; }

                    // Remove this path from remaining options for this move
                    (*traversal_attempts)[attempt_index].options_remaining[attempt_len] &= 0b1111 ^ next_move;
                }
            }

            if next_move>0 {
                break;
            }
        }
        attempt_index += 1;
    }

    return (previous_attempts_match, next_move);
}

/*------------------------------------------------ do_possibilities_remain - */

fn do_possibilities_remain(attempt: &Attempt, index: usize) -> bool {

    let attempt_len = attempt.options_remaining.len();
    if index < attempt_len {
        for i in index..attempt_len {
            if attempt.options_remaining[i]>0 {
                return true;
            }
        }
    }

    return false;
}

/*------------------------------------------------------ do_attempts_match - */

fn do_attempts_match(reference_attempt: &Attempt, current_attempt: &Attempt) -> bool {

    let mut attempts_match: bool = false;

    let current_len = current_attempt.moves.len();
    if reference_attempt.moves.len() > current_len {
        if current_len==0 {
            attempts_match = true;
        } else {
            for i in 0..current_len {
                if reference_attempt.moves[i] != current_attempt.moves[i] {
                    break;
                } else if i == current_len-1 { // full match
                    attempts_match = true;
                    break;
                }
            }
        }
    }

    return attempts_match;
}

/*--------------------------------------------------------- End of main.rs - */
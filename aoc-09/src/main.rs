use std::io::{BufRead, BufReader};
use std::fs::File;

#[macro_use]
mod applog;
mod startup;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}
impl Default for Position {
    fn default () -> Position {
        Position{x: 0, y: 0}
    }
}

/*------------------------------------------------------------------- main - */

fn main() {
    let reader = startup::get_reader().unwrap();

    if startup::is_part1() {  
        simulate_rope_movement(reader, 2);
    } else {
        simulate_rope_movement(reader, 10);
    }  

    applog::end_timestamp(startup::get_start_time());
}

/*------------------------------------------------- simulate_rope_movement - */

fn simulate_rope_movement(reader: BufReader<File>, number_of_knots: usize) {
    let mut rope: Vec<Position> = vec![Position::default(); number_of_knots];

    let mut tail_positions: Vec<Position> = vec![];
    tail_positions.push(rope[0]); // always at least 1 tail position - the start

    // Read the file and execute moves, keeping track of new tail positions
    for line in reader.lines() {
        let line = line.unwrap();
        execute_move(&line, &mut rope, &mut tail_positions);
    }
    
    applog!("Number of unique tail positions: {}", tail_positions.len());
}

/*----------------------------------------------------------- execute_move - */

fn execute_move(line: &str, rope: &mut Vec<Position>, tail_positions: &mut Vec<Position>) {
    let tokens: Vec<&str> = line.split(' ').collect();
    let direction: char = tokens[0].chars().nth(0).unwrap();
    let steps: u32 = tokens[1].parse::<u32>().unwrap();
    for _i in 0..steps {
        move_one_step(direction, rope);
        if !tail_positions.contains(&rope[0]) {
            tail_positions.push((*rope)[0]);
        }
    }
}

/*---------------------------------------------------------- move_one_step - */

fn move_one_step(direction: char, rope: &mut Vec<Position>) {

    // index to keep track of current head knot
    let mut index: usize = rope.len()-1;

    match direction {
        'R' => rope[index].x += 1,
        'L' => rope[index].x -= 1,
        'U' => rope[index].y += 1,
        _ => rope[index].y -= 1,
    };

    // Ensure each knot is moved so that it remains adjacent to the one in front
    while index>0 && !are_positions_adjacent(&rope[index], &rope[index-1]) {
        move_tail_to_follow(rope, index);
        index -= 1;
    }
}

/*------------------------------------------------ are_positions_adjacent - */

fn are_positions_adjacent(head: &Position, tail: &Position) -> bool {

    // In order for tail to be adjacent the head,
    // both x & y coordinates must be within 1

    let delta_x: i32 = if head.x > tail.x { head.x - tail.x} else {tail.x - head.x};
    let delta_y: i32 = if head.y > tail.y { head.y - tail.y} else {tail.y - head.y};

    if delta_x>1 || delta_y>1 {
        return false;
    }

    return true;
}

/*------------------------------------------------- move_tail_to_follow - */

fn move_tail_to_follow(rope: &mut Vec<Position>, head_index: usize) {
    
    // This fn is only called if either x-delta or y-delta >1
    // Move the tail in direction to bring the delta back to 1
    // Move diagonally closer if possible

    // shorter names for clarity
    let h = head_index;
    let t = head_index-1;

    if rope[h].x > rope[t].x {
        rope[t].x += 1;
    } else if rope[h].x < rope[t].x {
        rope[t].x -= 1;
    }

    if rope[h].y > rope[t].y {
        rope[t].y += 1;
    } else if rope[h].y < rope[t].y {
        rope[t].y -= 1;
    }
}

/*--------------------------------------------------------- End of main.rs - */
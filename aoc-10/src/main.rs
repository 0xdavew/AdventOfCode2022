use std::io::{BufRead, BufReader};
use std::fs::File;

#[macro_use]
mod applog;
mod startup;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Operation {
    NoOp,
    AddX
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    op: Operation,
    value: i32,
}
impl Default for Instruction {
    fn default () -> Instruction {
        Instruction{op: Operation::NoOp, value: 0}
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

    let mut x: i32 = 1;
    let mut cycle_number: u32 = 0; // number of cycle we have just started
    let mut signal_strength_sum: i32 = 0;

    // Read the file
    for line in reader.lines() {
        let line = line.unwrap();
        let command = read_next_instruction(&line);
        let mut cycles_this_command: u32 = 0;
        let mut command_completed: bool = false;
        while !command_completed {
            cycle_number += 1;
           
            let signal_strength: i32 = (cycle_number as i32) * x;
            
            match cycle_number {
                20 | 60 | 100 | 140 | 180 | 220 => {
                    signal_strength_sum += signal_strength;
                    applog!("Signal strength during cycle {} is {}.", cycle_number, signal_strength);
                },
                _ => {},
            }

            command_completed = execute_cpu_cycle(&command, &mut x, &mut cycles_this_command);
        }
    }
    
    applog!("Signal strength sum: {}", signal_strength_sum);
}

/*------------------------------------------------------------------ part2 - */

fn part2(reader: BufReader<File>) {

    let mut x: i32 = 1;
    let mut cycle_number: u32 = 0; // number of last cycle

    let mut screen: [String; 6] = Default::default();

    for line in reader.lines() {
        let line = line.unwrap();
        let command = read_next_instruction(&line);
        let mut cycles_this_command: u32 = 0;
        let mut command_completed: bool = false;
        while !command_completed {

            let pixel_char = get_pixel_char(cycle_number, x);
            
            let row: usize = (cycle_number as usize)/40;
            screen[row].push(pixel_char);

            command_completed = execute_cpu_cycle(&command, &mut x, &mut cycles_this_command);
            cycle_number += 1;
        }
    }

    for row in screen {
        applog!("{}", row);
    }
        
}

/*------------------------------------------------------ execute_cpu_cycle - */

// returns true if we have completed this command

fn execute_cpu_cycle(command: &Instruction, x: &mut i32, cycles_this_command: &mut u32) -> bool {

    *cycles_this_command += 1;

    if command.op == Operation::NoOp {
        return true;
    } else if command.op == Operation::AddX {
        if *cycles_this_command >= 2u32 {
            *x += command.value;
            return true;
        } else {
            return false;
        }
    } else {
        panic!("Unsupported operation!");
    }
}

/*-------------------------------------------------- read_next_instruction - */

fn read_next_instruction(line: &str) -> Instruction {
    let tokens: Vec<&str> = line.split(' ').collect();
    let command = tokens[0];
    
    let mut instruction: Instruction = Instruction::default();

    if command == "noop" {
        instruction.op = Operation::NoOp;
    } else if command == "addx" {
        instruction.op = Operation::AddX;
        instruction.value = tokens[1].parse::<i32>().unwrap();
    } else {
        panic!("Unsupported command: {}, line=[{}]", command, line);
    }

    return instruction;
}

/*--------------------------------------------------------- get_pixel_char - */

fn get_pixel_char(cycle_number: u32, x: i32) -> char {

    let screen_size: u32 = 40;
    let pixel_location: u32 = (cycle_number)%screen_size;
    let reduced_sprite: bool = if x<0 {true} else {false};
    let sprite_location: u32 = if x<0 {0} else {x as u32};

    let pixel_delta: u32 = 
        if sprite_location > pixel_location {
            sprite_location - pixel_location
        } else {
            pixel_location - sprite_location
        };

    let min_pixel_delta: u32 = if reduced_sprite {0} else {1};
    let pixel_char: char = if pixel_delta <= min_pixel_delta {'#'} else {'.'};

    return pixel_char;
}
/*--------------------------------------------------------- End of main.rs - */
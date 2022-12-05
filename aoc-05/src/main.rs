use std::io::{BufRead, BufReader};
use std::fs::File;

#[macro_use]
mod applog;
mod startup;

#[derive(Debug, Copy, Clone)]
struct Instruction {
    number: i32,
    from: usize,
    to: usize,
}
impl Default for Instruction {
    fn default () -> Instruction {
        Instruction{number: 0, from: 0, to: 0}
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
    perform_container_operations(reader, true);
}

/*------------------------------------------------------------------ part2 - */

fn part2(reader: BufReader<File>) {
    perform_container_operations(reader, false);
}

/*------------------------------------------- perform_container_operations - */

fn perform_container_operations(reader: BufReader<File>, part1: bool) {

    let mut container_stacks: Vec<Vec<char>> = vec![];
    let mut instructions: Vec<Instruction> = vec![];
    let mut reading_instructions: bool = false;
   
    // Read the file
    for line in reader.lines() {
        let line = line.unwrap();

        // Have we reached the end of the stacks?
        if line.len()==0 {
            reading_instructions = true;
            continue;
        }

        // What are we reading?
        if reading_instructions {
            read_instruction_record(&line, &mut instructions);
        } else {
            read_container_stack_record(&line, &mut container_stacks);
        }
    }

    follow_instructions(&instructions, &mut container_stacks, !part1);

    // Get top of the stacks
    let mut top_of_stacks: String = String::new();
    let num_stacks=container_stacks.len();
    for stack_index in 0..num_stacks {
        top_of_stacks.push(container_stacks[stack_index].pop().unwrap());
    }
    applog!("Top of stacks: {}", top_of_stacks);
}

/*------------------------------------------------ read_instruction_record - */

fn read_instruction_record (line: &str, instructions: &mut Vec<Instruction>) {
    let tokens: Vec<&str> = line.split(" ").collect();
    let instruction = Instruction {
        number: tokens[1].parse::<i32>().unwrap(),
        from: tokens[3].parse::<usize>().unwrap(),
        to: tokens[5].parse::<usize>().unwrap()
    };

    instructions.push(instruction);
}

/*-------------------------------------------- read_container_stack_record - */

fn read_container_stack_record(line: &str, container_stacks: &mut Vec<Vec<char>>) {
    // populate container_stacks if not already done
    let num_stacks = (line.len()+1)/4;
    while container_stacks.len() < num_stacks {
        let empty_stack: Vec<char> = vec![];
        container_stacks.push(empty_stack);
    }

    // Iterate over stacks - see which have a crate at this level
    for stack_index in 0..num_stacks {
        let stack_item = line.chars().nth(1 + stack_index*4).unwrap();
        if stack_item == '1' { // Have we reached the stack numbers?
            break;
        }

        if stack_item != ' ' { // Is stack populated at this level?
            container_stacks[stack_index].push(stack_item);
        }
    }
}

/*---------------------------------------------------- follow_instructions - */

fn follow_instructions(instructions: &Vec<Instruction>, container_stacks: &mut Vec<Vec<char>>, multi_move: bool) {

    // First thing: reverse all the stacks - we want to remove last added item first 
    // (bottom of the stack)
    let num_stacks = container_stacks.len();
    for stack_index in 0..num_stacks {
        container_stacks[stack_index].reverse();
    }

    applog!("Stacks {:?}", container_stacks);

    // Follow the instructions
    for instruction in instructions {
        applog!("{:?}", instruction);
        
        // Move specified number of crates
        if multi_move {
            let mut staging: Vec<char> = vec![];
            for _i in 0..instruction.number {
                let stack_item = container_stacks[instruction.from-1].pop().unwrap();
                staging.push(stack_item);
            }
            for _i in 0..instruction.number {
                let stack_item = staging.pop().unwrap();
                container_stacks[instruction.to-1].push(stack_item);
            }
        } else {
            for _i in 0..instruction.number {
                let stack_item = container_stacks[instruction.from-1].pop().unwrap();
                container_stacks[instruction.to-1].push(stack_item);
            }
        }
        applog!("Stacks {:?}", container_stacks);
    }
}

/*--------------------------------------------------------- End of main.rs - */
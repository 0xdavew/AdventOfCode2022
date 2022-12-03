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
    let mut priorities: Vec<i32> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        let compartment_len = line.len()/2;
        let compartment1 = &line[..compartment_len];
        let compartment2 = &line[compartment_len..];

        let mut item: char = '\0';
        for c in compartment1.chars() {
            if compartment2.contains(c) {
                item = c;
                break;
            }
        }

        let priority = get_item_priority(item);
        priorities.push(priority);
    }

    let sum: i32 = priorities.iter().sum();
    applog!("Sum of priorities: {}", sum);
}

/*------------------------------------------------------------------ part2 - */

fn part2(reader: BufReader<File>) {
    let mut priorities: Vec<i32> = vec![];
    let mut elf_group: [String; 3] = Default::default();

    for (line_number, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let elf_index = line_number%3;
        elf_group[elf_index] = line;
        if elf_index==2 {
            let (elf1, elf2, elf3) = (&elf_group[0], &elf_group[1], &elf_group[2]);
            let mut item: char = '\0';
            for c in elf1.chars() {
                if elf2.contains(c) && elf3.contains(c) {
                    item = c;
                    break;
                }
            }

            let priority = get_item_priority(item);
            priorities.push(priority);
        }
    }

    let sum: i32 = priorities.iter().sum();
    applog!("Sum of priorities: {}", sum);
}

/*------------------------------------------------------ get_item_priority - */

fn get_item_priority(item: char) -> i32 {
    if item>='a' && item<='z' {
        1 + item as i32 -'a' as i32
    } else if item>='A' && item<='Z' {
        1 + item as i32 -'A' as i32 + 26
    } else {
        0
    }
}

/*--------------------------------------------------------- End of main.rs - */
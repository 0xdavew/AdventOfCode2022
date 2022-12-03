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
    track_elf_calories(reader);
}

/*------------------------------------------------------------------ part2 - */

fn part2(reader: BufReader<File>) {
    track_elf_calories(reader);
}

/*----------------------------------------------------- track_elf_calories - */

fn track_elf_calories(reader: BufReader<File>) {

    let mut calories: Vec<i32> = vec![];
    let mut elf_index: usize = 0;
    
    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 { 
            elf_index += 1;
        }
        add_elf_calories(&mut calories, elf_index, &line);
    }

    let (len, max) = (calories.len(), calories.iter().max().unwrap());

    applog!("Elves: {}, Max calories: {}, Top elf: {}", len, max, calories.iter().position(|x| x==max).unwrap()+1);

    calories.sort();
    applog!("Sum of top 3: {}", calories[len-1] + calories[len-2] + calories[len-3]);
}

/*------------------------------------------------------- add_elf_calories - */

fn add_elf_calories(calories: &mut Vec<i32>, elf_index: usize, line: &String) {
    while calories.len()<=elf_index {
        calories.push(0);
    }
    if line.len()>0 {
        let value: i32 = line.parse().unwrap();
        calories[elf_index] += value;
    }
}

/*--------------------------------------------------------- End of main.rs - */

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

    // Read the file
    for line in reader.lines() {
        let line = line.unwrap();

        let (mut c1, mut c2, mut c3, mut c4): (char, char, char, char) = ('0', '0', '0', '0');
        let mut marker_index: usize = 0;

        for (i, c) in line.chars().enumerate() {
            match i%4 {
                0 => c1 = c,
                1 => c2 = c,
                2 => c3 = c,
                _ => c4 = c,
            }
            if i<3 {
                continue;
            }

            if c1!=c2 && c1!=c3 && c1 !=c4 && c2!=c3 && c2!=c4 && c3!=c4 {
                marker_index = i+1;
                break;
            }
        }
        applog!("Marker index: {}", marker_index);
    }   
}

/*------------------------------------------------------------------ part2 - */

fn part2(reader: BufReader<File>) {

    // Read the file
    for line in reader.lines() {
        let line = line.unwrap();

        let block_size: usize = 14;
        let max: usize = line.len()-block_size;
        let mut marker_index: usize = 0;

        for i in 0..max {
            let slice = &line[i..i+block_size];
        
            if has_dup(slice)==false {
                marker_index = i+block_size;
                break;
            }
        }
        applog!("Marker index: {}", marker_index);
    }
}

/*---------------------------------------------------------------- has_dup - */

fn has_dup(block: &str) -> bool {
    let max: usize = block.len()-1;
    for i in 0..max {
        let c = block.chars().nth(i).unwrap();
        let slice = &block[i+1..];
        if slice.contains(c) {
            return true;
        }
    }
    false
}

/*--------------------------------------------------------- End of main.rs - */
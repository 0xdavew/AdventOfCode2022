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
    get_overlaps(reader);
}

/*------------------------------------------------------------------ part2 - */

fn part2(reader: BufReader<File>) {
    get_overlaps(reader);
}

/*----------------------------------------------------------- get_overlaps - */

fn get_overlaps(reader: BufReader<File>) {
    let mut full_overlaps: i32 = 0;
    let mut partial_overlaps: i32 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let ranges: Vec<&str> = line.split(',').collect();
        let range1: Vec<&str> = ranges[0].split('-').collect();
        let range2: Vec<&str> = ranges[1].split('-').collect();

        // Each line: s1-e1,s2-e2
        let s1 = range1[0].parse::<i32>().unwrap();
        let e1 = range1[1].parse::<i32>().unwrap();
        let s2 = range2[0].parse::<i32>().unwrap();
        let e2 = range2[1].parse::<i32>().unwrap();

        // Full overlaps:       Partial overlaps:
        // ...s1........e1...   ...s1.....e1......
        // ......s2..e2......   ......s2.....e2...

        if (s1<=s2 && e1>=e2) || (s2<=s1 && e2>=e1) {
            full_overlaps += 1;
        } else if (s1<=s2 && s2<=e1 && e2>=e1) || (s2<=s1 && s1<=e2 && e1>=e2) {
            partial_overlaps += 1;
        }
    }

    applog!("Overlaps: full={}, partial={}, total={}", full_overlaps, partial_overlaps, full_overlaps+partial_overlaps);
}

/*--------------------------------------------------------- End of main.rs - */
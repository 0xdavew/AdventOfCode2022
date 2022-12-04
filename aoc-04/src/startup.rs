use std::fs::File;
use std::io::BufReader;
use std::env;
use std::path::Path;
use std::time::Instant;
use once_cell::sync::OnceCell;

use crate::applog;

static START_TIME: OnceCell<Instant> = OnceCell::new();
static mut PART1: bool = false;

/*------------------------------------------------------------- get_reader - */

pub fn get_reader() -> Result<BufReader<File>, &'static str> {
    let _start_time = START_TIME.get_or_init(Instant::now);
    let args: Vec<String> = env::args().collect();
    let exe_name = Path::new(&args[0]).file_stem().unwrap().to_str().unwrap();
    let test: bool = args.contains(&String::from("-test"));
    let part1: bool = args.contains(&String::from("-part1"));

    applog!("Starting [{}], [test={}, part1={}]...", exe_name, test, part1);

    unsafe {
        PART1 = part1;
    }
    
    let prefix = if test {"test_"} else {""};
    let filename = format!("{}input.txt", prefix);

    if !Path::new(&filename).exists() {
        applog!("File {} does not exist.", filename);
        return Err("Input file does not exist")
    }

    applog!("Reading file: {} ...", filename);

    let file = File::open(filename).unwrap();
    return Ok(BufReader::new(file));
}

/*--------------------------------------------------------------- is_part1 - */

pub fn is_part1() -> bool {
    unsafe {
        return PART1;
    }
}

/*---------------------------------------------------------- get_start_time - */

pub fn get_start_time() -> Instant {
    return *START_TIME.get().unwrap();
}

/*------------------------------------------------------- End of startup.rs - */
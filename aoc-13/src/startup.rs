use std::fs::File;
use std::io::BufReader;
use std::env;
use std::path::Path;
use std::time::Instant;
use once_cell::sync::OnceCell;

use crate::applog;

// Define a structure for global data

#[derive(Debug, Clone)]
struct StartupInfo {
    start_time: Instant,
    part1: bool,
    debug: bool,
    mono: bool,
    notime: bool
}
impl Default for StartupInfo {
    fn default () -> StartupInfo {
        StartupInfo {
            start_time: Instant::now(), 
            part1: env::args().collect::<String>().contains("-part1"), 
            debug: env::args().collect::<String>().contains("-debug"),
            mono: env::args().collect::<String>().contains("-mono"),
            notime: env::args().collect::<String>().contains("-notime"),
        }
    }
}

// Our global object - may only be set once
static APP_GLOBALS: OnceCell<StartupInfo> = OnceCell::new();

/*------------------------------------------------------------- get_reader - */

pub fn get_reader() -> Result<BufReader<File>, &'static str> {
    let _startup_info: &StartupInfo = APP_GLOBALS.get_or_init(StartupInfo::default);
    
    let args: Vec<String> = env::args().collect();
    let exe_name = Path::new(&args[0]).file_stem().unwrap().to_str().unwrap();

    let filename = build_input_filename();
    applog!("Starting [{}], [part1={}, debug={}, input={}]...", exe_name, is_part1(), is_debug(), filename);
    applog!("Reading file: {} ...", filename);

    let file = File::open(filename).unwrap();
    return Ok(BufReader::new(file));
}

/*--------------------------------------------------- build_input_filename - */

fn build_input_filename() -> String {
    let args: Vec<String> = env::args().collect();
    let test2: bool = args.contains(&String::from("-test2"));
    let test: bool = if test2 {false} else {args.contains(&String::from("-test"))};

    let prefix = if test {"test_"} else if test2 {"test2_"} else {""};
    let filename = format!("{}input.txt", prefix);

    if !Path::new(&filename).exists() {
        panic!("Input file {} does not exist.", filename);
    }

    return filename;
}

/*--------------------------------------------------------------------- is - */

pub fn is(name: &str) -> bool {
    let flag: bool = match name {
        "part1" => APP_GLOBALS.get().unwrap().part1,
        "debug" => APP_GLOBALS.get().unwrap().debug,
        "mono" => APP_GLOBALS.get().unwrap().mono,
        "notime" => APP_GLOBALS.get().unwrap().notime,
        _ => panic!("Unsupported flag: -{}", name),
    };

    return flag;
}

/*--------------------------------------------------------------- is_part1 - */

pub fn is_part1() -> bool {
    return is("part1");
}

/*-------------------------------------------------------------- is_debug - */

pub fn is_debug() -> bool {
    return is("debug");
}

/*---------------------------------------------------------- get_start_time - */

pub fn get_start_time() -> Instant {
    return APP_GLOBALS.get().unwrap().start_time;
}

/*------------------------------------------------------- End of startup.rs - */
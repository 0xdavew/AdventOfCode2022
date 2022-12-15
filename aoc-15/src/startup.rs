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
    args: Vec<String>,
    start_time: Instant
}
impl StartupInfo {
    fn default () -> StartupInfo {
        StartupInfo {
            args: env::args().collect(),
            start_time: Instant::now(), 
        }
    }
    fn is_option_on(&self, option: &str) -> bool {
        let command = format!("-{}", option);
        return self.args.contains(&command);
    }
    fn get_exe_name(&self) -> &str {
        return Path::new(&self.args[0]).file_stem().unwrap().to_str().unwrap();
    }
}

// Our global object - may only be set once
static APP_GLOBALS: OnceCell<StartupInfo> = OnceCell::new();

/*------------------------------------------------------------- get_reader - */

pub fn get_reader() -> Result<BufReader<File>, &'static str> {
    
    let startup: &StartupInfo = APP_GLOBALS.get_or_init(StartupInfo::default);
    let filename = build_input_filename();

    applog!("Starting [{}], [part1={}, debug={}, input={}]...", 
        startup.get_exe_name(), 
        is("part1"), 
        is("debug"), 
        filename);
    
    if is("debug") {
        applog!("Reading file: {} ...", filename);
    }

    let file = File::open(filename).unwrap();
    return Ok(BufReader::new(file));
}

/*--------------------------------------------------- build_input_filename - */

fn build_input_filename() -> String {
    
    let test2: bool = is("test2");
    let test: bool = if test2 {false} else {is("test")};

    let prefix = if test {"test_"} else if test2 {"test2_"} else {""};
    let filename = format!("{}input.txt", prefix);

    if !Path::new(&filename).exists() {
        panic!("Input file {} does not exist.", filename);
    }

    return filename;
}

/*--------------------------------------------------------------------- is - */

pub fn is(name: &str) -> bool {
    return APP_GLOBALS.get().unwrap().is_option_on(name);
}

/*---------------------------------------------------------- get_start_time - */

pub fn get_start_time() -> Instant {
    return APP_GLOBALS.get().unwrap().start_time;
}

/*------------------------------------------------------- End of startup.rs - */
use chrono::Local;
use ansi_term::Colour;
use std::sync::Once;
use std::time::Instant;

use crate::startup;

static INIT: Once = Once::new();

/*---------------------------------------------------------------- applog! - */

macro_rules! applog {
    ($fmt_str:literal) => {{
        applog::internal_applog(&format!($fmt_str));
    }};

    ($fmt_str:literal, $($args:expr),*) => {{
        applog::internal_applog(&format!($fmt_str, $($args),*));
    }};
}

/*-------------------------------------------------------- internal_applog - */

pub fn internal_applog(line: &String) {
    let mono = startup::is("mono");
    let notime: bool = startup::is("notime");

    INIT.call_once(|| {
        let _enabled = ansi_term::enable_ansi_support();
    });

    if mono {
        if notime {
            println!("{}", line);
        } else {
            println!("{} {}", get_time_stamp(), line);
        }
    } else {
        if notime {
            println!("{}", Colour::Yellow.paint(line));
        } else {
            println!("{} {}", 
                Colour::Green.paint(get_time_stamp()), 
                Colour::Yellow.paint(line));
        }
    }
}

/*--------------------------------------------------------- get_time_stamp - */

fn get_time_stamp() -> String {
    let now = Local::now();
    return now.format("%Y-%m-%d %H:%M:%S%.3f").to_string();
}

/*---------------------------------------------------------- end_timestamp - */

pub fn end_timestamp(start_time: Instant) {
    let ns = start_time.elapsed().as_nanos();
    let us = ns/1000;
    let ms = us/1000;
    internal_applog(&format!("Time taken: {}.{:03}{}ms", ms, us%1000, (ns%1000)/100));
}

/*------------------------------------------------------- End of applog.rs - */
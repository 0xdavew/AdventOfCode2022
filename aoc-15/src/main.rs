use std::io::{BufRead, BufReader};
use std::fs::File;

#[macro_use]
mod applog;
mod startup;

#[derive(Debug, Clone, Copy)]
struct Range {
    start: i32,
    end: i32
}
impl Default for Range {
    fn default () -> Range {
        Range { start: 0, end: 0}
    }
}
impl Range {
    fn contains(&self, location: i32) -> bool {
        return location >= self.start && location <= self.end;
    }
    fn touches(&self, location: i32) -> bool {
        return location >= self.start-1 && location <= self.end+1;
    }
    fn size(&self) -> u32 {
        return (self.start - self.end).abs() as u32 + 1u32;
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32
}
impl Default for Point {
    fn default () -> Point {
        Point { x: 0, y: 0}
    }
}

#[derive(Debug, Clone, Copy)]
struct Reading {
    sensor: Point,
    beacon: Point
}
impl Default for Reading {
    fn default () -> Reading {
        Reading { sensor: Point::default(), beacon: Point::default()}
    }
}

impl Reading {
    fn distance(&self) -> u32 {
        let x_delta = (self.sensor.x - self.beacon.x).abs() as u32;
        let y_delta = (self.sensor.y - self.beacon.y).abs() as u32;
        return x_delta + y_delta;
    }
    fn sensor_to_row_overlap(&self, row: i32) -> i32 {
        let distance = self.distance() as i32;

        let row_delta = (self.sensor.y - row).abs();
        
        return distance - row_delta;
    }
    fn get_row_visibility(&self, row: i32) -> (bool, Range) {
        let mut range: Range = Range::default();

        let overlap = self.sensor_to_row_overlap(row);
        let visible = if overlap<0 {false} else {true};

        range.start = if visible {self.sensor.x - overlap} else {0};
        range.end = if visible {self.sensor.x + overlap} else {0};

        return (visible, range);
    }
}

/*------------------------------------------------------------------- main - */

fn main() {
    let reader = startup::get_reader().unwrap();

    if startup::is("part1") {  
        part1(reader);
    } else {
        part2(reader);
    }  

    applog::end_timestamp(startup::get_start_time());
}

/*------------------------------------------------------------------ part1 - */

fn part1(reader: BufReader<File>) {

    let mut readings: Vec<Reading> = vec![];
    import_readings(reader, &mut readings);

    let row = if startup::is("test") {10} else {2000000};

    let sensor_ranges = get_sensor_ranges_hitting_row(&readings, row);
    let merged_ranges = merge_sensor_ranges(&sensor_ranges); 

    if startup::is("debug") {
        applog!("{} of {} sensors can see row {}", sensor_ranges.len(), readings.len(), row);
        applog!("Ranges of visibility: {:?}", merged_ranges);
    }

    let beacons = get_beacons_in_ranges(&readings, &merged_ranges, row);
    applog!("Beacons in row {}: {:?}", row, beacons);

    let cells_without_beacon: u32 = get_size_of_ranges(&merged_ranges) - beacons.len() as u32;
    applog!("PART1: On row {}, {} cells are known to not contain a beacon.", row, cells_without_beacon);
}

/*------------------------------------------------------------------ part2 - */

fn part2(reader: BufReader<File>) {

    let mut readings: Vec<Reading> = vec![];
    import_readings(reader, &mut readings);

    let max = if startup::is("test") {20} else {4000000};
    for r in 0..max {
        let (found, hidden) = get_hidden_cells(&readings, r);
        if found {
            let multiplier: i64 = 4000000;
            applog!("Found hidden location: ({},{})", hidden.x, hidden.y);
            let (x, y): (i64, i64) = (hidden.x as i64, hidden.y as i64);
            applog!("Tuning frequency: {}", x * multiplier + y);
        }
    }
}

/*-------------------------------------------------------- import_readings - */

fn import_readings(reader: BufReader<File>, readings: &mut  Vec<Reading>) {

    for line in reader.lines() {
        let line = line.unwrap();

        let reading = parse_reading(&line);
        readings.push(reading);
    }

    if startup::is("debug") {
        applog!("Successfully read {} readings.", readings.len());
    }
}

/*---------------------------------------------------------- parse_reading - */

fn parse_reading(line: &str) -> Reading {

    let tokens: Vec<&str> = line.split(' ').collect();
    if tokens.len() != 10 {
        panic!("Expected 10 tokens, got: {}, line=[{}]", tokens.len(), line);
    }

    let mut reading: Reading = Reading::default();

    reading.sensor.x = extract_int(&tokens[2]);
    reading.sensor.y = extract_int(&tokens[3]);
    reading.beacon.x = extract_int(&tokens[8]);
    reading.beacon.y = extract_int(&tokens[9]);

    return reading;
}

/*------------------------------------------------------------ extract_int - */

// Assumes format: [name]=[value]...

fn extract_int(input_string: &str) -> i32 {
    let tokens: Vec<&str> = input_string.split::<char>('=').collect();
    let value_string: &str = tokens[1];

    let mut start_index: usize=0;
    let mut i: usize=0;
    let mut number_started: bool = false;
    for c in value_string.chars() {

        if c.is_digit(10) && number_started==false {
            start_index = i;
            number_started = true; 
        } else if !c.is_digit(10) && number_started==true {
            break
        }

        i += 1;
    }

    if !number_started {
        panic!("No number found in input: {}", input_string);
    }

    let negative = start_index>0 && value_string.chars().nth(start_index-1).unwrap()=='-';
    let factor = if negative {-1} else {1};
    let digits: &str = if i==value_string.len() {&value_string[start_index..]} else {&value_string[start_index..i]};

    return digits.parse::<i32>().unwrap() * factor; 
}

/*--------------------------------------------------- get_beacons_in_range - */

fn get_beacons_in_ranges(readings: &Vec<Reading>, ranges: &Vec<Range>, row: i32) -> Vec<i32> {
    let mut beacons: Vec<i32> = vec![];

    for reading in readings {
        for range in ranges {
            if reading.beacon.y == row {
                let beacon = reading.beacon.x;
                if range.contains(beacon) && !beacons.contains(&beacon) {
                    beacons.push(beacon);
                }
            }
        }
    }

    return beacons.clone();
}

/*------------------------------------------- get_sensor_ranges_hitting_row - */

fn get_sensor_ranges_hitting_row(readings: &Vec<Reading>, row: i32) -> Vec<Range> {

    let mut ranges: Vec<Range> = vec![];

    for r in readings.iter() {
        let (visible, range) = r.get_row_visibility(row);
        if visible {
            ranges.push(range);
        }
    }

    // Sort the ranges, then merge
    ranges.sort_by(|a, b| a.start.cmp(&b.start));

    return ranges;
}

/*---------------------------------------------------- merge_sensor_ranges - */

fn merge_sensor_ranges(ranges: &Vec<Range>) -> Vec<Range> {

    let mut merged_ranges: Vec<Range> = vec![];
    let mut merged_range: Range = Range::default();
    for i in 0..ranges.len() {
        let range = ranges[i];

        if i == 0 {
            merged_range = range;
            continue;
        }

        // range may be merged into current_range
        if merged_range.touches(range.start) {
            if range.end > merged_range.end {
                merged_range.end = range.end;
            }
            continue;
        }

        // need a new range
        merged_ranges.push(merged_range);
        merged_range = range;
    }
    merged_ranges.push(merged_range);

    return merged_ranges;
}

/*------------------------------------------------------ get_hidden_cells - */

fn get_hidden_cells(readings: &Vec<Reading>, row: i32) -> (bool, Point) {
    
    let mut found: bool = false;
    let mut hidden: Point = Point::default();
    let unmerged_visible = get_sensor_ranges_hitting_row(readings, row);
    let merged = merge_sensor_ranges(&unmerged_visible);

    if merged.len() == 2 {
        found = true;
        hidden.x = merged[0].end+1;
        hidden.y = row;
    }

    return (found, hidden);
}

/*----------------------------------------------------- get_size_of_ranges - */
// assumes ranges pre-merged and non-touching
fn get_size_of_ranges(ranges: &Vec<Range>) -> u32 {
    let mut total_size = 0;

    for r in ranges {
        total_size += r.size();
    }

    return total_size;
}

/*--------------------------------------------------------- End of main.rs - */
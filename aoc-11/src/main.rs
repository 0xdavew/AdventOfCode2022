use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::VecDeque;

#[macro_use]
mod applog;
mod startup;

#[derive(Debug, Copy, Clone)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Operand {
    Value,
    MySelf
}

#[derive(Debug, Clone)]
struct Monkey {
    index: u32,
    worry_levels: VecDeque<u64>,
    operator: Operator,
    operand: Operand,
    value: u64,
    divisor: u64,
    monkey_if_true: usize,
    monkey_if_false: usize,
    inspection_count: u64
}
impl Default for Monkey {
    fn default () -> Monkey {
        Monkey {
            index: 0, 
            worry_levels: VecDeque::new(), 
            operator: Operator::Add, 
            operand: Operand::Value, 
            value: 0,
            divisor: 0,
            monkey_if_true: 0,
            monkey_if_false: 0,
            inspection_count: 0
        }
    }
}
impl Monkey {
    fn clear (&mut self)  {
        self.index = 0; 
        self.worry_levels.clear(); 
        self.operator = Operator::Add;
        self.operand = Operand::Value; 
        self.value = 0;
        self.divisor = 0;
        self.monkey_if_true = 0;
        self.monkey_if_false = 0;
        self.inspection_count = 0;
    }
}

/* ------------------------------------------------------------------ main - */

fn main() {
    let reader = startup::get_reader().unwrap();

    let mut monkeys: Vec<Monkey> = vec![];
    read_monkeys(reader, &mut monkeys);

    let rounds = if startup::is_part1() {20} else {10000};
    let divide_by_3 = if startup::is_part1() {true} else {false};

    // Perform specified number of rounds
    for r in 0..rounds {
        if startup::is_debug() {
            applog!("Starting round: {}", r+1);
            dump_monkey_activity(&monkeys, false);
        }
        perform_monkey_round(&mut monkeys, divide_by_3);
    }

    dump_monkey_activity(&monkeys, true);

    applog::end_timestamp(startup::get_start_time());
}

/* --------------------------------------------------------- read_monkeys - */

fn read_monkeys(reader: BufReader<File>, monkeys: &mut Vec<Monkey>) {

    let mut monkey: Monkey = Monkey::default();
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        match i%7 {
            0 => read_monkey_header(&line, &mut monkey),
            1 => read_worry_levels(&line, &mut monkey),
            2 => read_operation(&line, &mut monkey),
            3 => read_divisor(&line, &mut monkey),
            4 => read_true_monkey(&line, &mut monkey),
            5 => { 
                read_false_monkey(&line, &mut monkey); 
                monkeys.push(monkey.clone());
                monkey.clear();
            },
            _ => (),
        }
    }
}

/* ---------------------------------------------------- read_monkey_header - */

fn read_monkey_header(line: &str, monkey: &mut Monkey) {

    let prefix = "Monkey ";
    if line.starts_with(prefix)==false {
        panic!("Expected [{}], got [{}]", prefix, line);
    }
    monkey.index = line[prefix.len()..line.len()-1].parse::<u32>().unwrap();
}

/* ----------------------------------------------------- read_worry_levels - */

fn read_worry_levels(line: &str, monkey: &mut Monkey) {

    let prefix = "  Starting items: ";
    if line.starts_with(prefix)==false {
        panic!("Expected [{}], got [{}]", prefix, line);
    }
    let levels: Vec<&str> = line[prefix.len()..].split(", ").collect();
    for level in levels {
        monkey.worry_levels.push_back(level.parse::<u64>().unwrap());
    }
}

/* -------------------------------------------------------- read_operation - */

fn read_operation(line: &str, monkey: &mut Monkey) {

    let prefix = "  Operation: new = old ";
    if line.starts_with(prefix)==false {
        panic!("Expected [{}], got [{}]", prefix, line);
    }
    let tokens: Vec<&str> = line[prefix.len()..].split(" ").collect();
    match tokens[0] {
        "+" => monkey.operator = Operator::Add,
        "-" => monkey.operator = Operator::Subtract,
        "*" => monkey.operator = Operator::Multiply,
        "/" => monkey.operator = Operator::Divide,
        _ => panic!("Unrecognised operator: [{}], line: [{}]", tokens[0], line),
    }

    if tokens[1] == "old" {
        monkey.operand = Operand::MySelf;
    } else {
        monkey.operand = Operand::Value;
        monkey.value = tokens[1].parse::<u64>().unwrap();
    }
}

/* ---------------------------------------------------------- read_divisor - */

fn read_divisor(line: &str, monkey: &mut Monkey) {

    let prefix = "  Test: divisible by ";
    if line.starts_with(prefix)==false {
        panic!("Expected [{}], got [{}]", prefix, line);
    }
    monkey.divisor = line[prefix.len()..line.len()].parse::<u64>().unwrap();
}

/* ------------------------------------------------------ read_true_monkey - */

fn read_true_monkey(line: &str, monkey: &mut Monkey) {

    let prefix = "    If true: throw to monkey ";
    if line.starts_with(prefix)==false {
        panic!("Expected [{}], got [{}]", prefix, line);
    }
    monkey.monkey_if_true = line[prefix.len()..line.len()].parse::<usize>().unwrap();
}

/* ----------------------------------------------------- read_false_monkey - */

fn read_false_monkey(line: &str, monkey: &mut Monkey) {

    let prefix = "    If false: throw to monkey ";
    if line.starts_with(prefix)==false {
        panic!("Expected [{}], got [{}]", prefix, line);
    }
    monkey.monkey_if_false = line[prefix.len()..line.len()].parse::<usize>().unwrap();
}

/* -------------------------------------------------- perform_monkey_round - */

fn perform_monkey_round(monkeys: &mut Vec<Monkey>, divide_by_3: bool) {

    let mut monkeys_left: Vec<usize> = vec![];
    for i in 0..monkeys.len() {
        while monkeys[i].worry_levels.len()>0 {

            // Get next worry level
            let w = monkeys[i].worry_levels.pop_front().unwrap();

            // Perform operation
            let argument = 
                if monkeys[i].operand == Operand::Value {
                    monkeys[i].value
                } else {
                    w
                };
            
            let mut new_worry = match monkeys[i].operator {
                Operator::Add => w + argument,
                Operator::Subtract => w - argument,
                Operator::Multiply => w * argument,
                Operator::Divide => w / argument,
            };
            
            if divide_by_3 {
                new_worry /= 3
            }
            
            // Get remainder of product of divisors - to ensure worry remains bounded
            // NB: This is the critical step to make step2 work!
            // If it's divisible, we get 0 => hence next divisor test always true
            // If it's not, remainder will pass/not-pass the test in the same way   
            new_worry %= monkeys.iter().map(|m| m.divisor).reduce(|a, b| a*b).unwrap();

            // Throw item to another monkey
            let target_monkey_index: usize = 
                if new_worry % monkeys[i].divisor == 0 {
                    monkeys[i].monkey_if_true
                } else {
                    monkeys[i].monkey_if_false
                };

            // Don't throw to yourself
            if target_monkey_index == i {
                panic!("Monkey can't throw item to themself.");
            }

            if startup::is_debug() {
                applog!("Monkey: {} -> {}, worry:{} -> {} [divisor={}]", i, target_monkey_index, w, new_worry, monkeys[i].divisor);
            }

            // Perform the throw
            monkeys[target_monkey_index].worry_levels.push_back(new_worry);

            // Increment inspection count
            monkeys[i].inspection_count += 1;

            if monkeys_left.contains(&i)==false { monkeys_left.push(i);}
        }
    }
    if startup::is_debug() {
        applog!("Monkeys left: {:?}", monkeys_left);
    }
}

/* -------------------------------------------------- dump_monkey_activity - */

fn dump_monkey_activity(monkeys: & Vec<Monkey>, get_monkey_business: bool) {

    // Dump monkey activity, tracking inspection numbers
    let mut inspection_counts: Vec<u64> = vec![];
    for m in monkeys {
        applog!("Monkey {}: inspections: {}, worry levels: {:?}", m.index, m.inspection_count, m.worry_levels);
        inspection_counts.push(m.inspection_count);
    }

    if get_monkey_business {
        // Compute monkey business
        applog!("Inspection counts: {:?}", inspection_counts);
        inspection_counts.sort_by(|a, b| b.cmp(a));
        applog!("Monkey business: {}", inspection_counts[0] * inspection_counts[1]);
    }
}

/* -------------------------------------------------------- End of main.rs - */
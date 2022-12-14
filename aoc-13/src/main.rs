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

    let mut left: String = String::default();
    let mut right: String = String::default();

    let mut pair_index: u32 = 0;
    let mut right_order_pairs: Vec<u32> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        if line.len()>0 {
            if left.len()==0 {
                left = line;
            } else {
                right = line;
                
                pair_index += 1;

                if startup::is_debug() {
                    applog!("Pair {}: {} vs {}", pair_index, left, right);
                } 
                let compare_result: i32 = compare_strings(&left, &right);
                applog!("Pair {}: ({}): {} vs {}", pair_index, if compare_result==0 {"="} else if compare_result<0 {"Y"} else {"N"}, left, right);

                if compare_result<=0 {
                    right_order_pairs.push(pair_index);
                }    
                left.clear();
            }
        }
    }
    if startup::is_debug() {
        applog!("Right-ordered pairs: {:?}", right_order_pairs);
    }
    
    applog!("Sum of indexes of right-order pairs: {}", right_order_pairs.iter().sum::<u32>());
}

// <0: Left < Right
// =0: Left = Right
// >0: Left > Right (i.e. wrong order)

fn compare_strings(left: &String, right: &String) -> i32 {

    let mut compare_result: i32 = 1; // assume wrong order

    if startup::is_debug() {
        applog!("Comparing: {} and {}...", left, right);
    }

    let left_data_blank: bool = left.len()==0;
    let right_data_blank: bool = right.len()==0;
    if left_data_blank || right_data_blank {
        if right_data_blank && !left_data_blank {
            compare_result = 1;
        } else if left_data_blank && !right_data_blank {
            compare_result = -1;
        } else {
            compare_result = 0;
        }

        if compare_result>0 && startup::is_debug() {
            applog!("Fail: right data is blank");
        }
        return compare_result;
    }

    let mut end_index: usize = 0;

    let (mut left_unquoting, mut right_unquoting): (u32, u32) = (0, 0);
    let left_data: &str = get_next_sb_field_unquoted(left, 0, &mut end_index, &mut left_unquoting);
    let right_data: &str = get_next_sb_field_unquoted(right, 0, &mut end_index, &mut right_unquoting);

    if startup::is_debug() {
        applog!("Raw data: Left={}, Right={}", left_data, right_data);
    }

    if left_data.len()==0 || right_data.len()==0 {
        if left_data.len() < right_data.len() {
            compare_result = -1;
        } else if left_data.len() > right_data.len() {
            compare_result = 1;
        } else {
            compare_result = 0;
        }

        if compare_result>0 && startup::is_debug() {
            applog!("Fail: right data has fewer fields");
        }

        // Has there been an unquoting discrepancy?
        if compare_result<0 && left_unquoting>right_unquoting {
            if startup::is_debug() {
                applog!("Fail: left data had extra nesting");
            }
            compare_result = 1;
        }
        return compare_result;
    }
    
    let sep: char = ',';
    let open_quote: char = '[';

    let left_data_list: bool = left_data.contains(sep); 
    let right_data_list: bool = right_data.contains(sep);

    if left_data_list || right_data_list {
        let mut left_tokens: Vec<String> = vec![];
        let mut right_tokens: Vec<String> = vec![];
        
        tokenise(left_data, &mut left_tokens);
        tokenise(right_data, &mut right_tokens);

        compare_result = compare_lists(&left_tokens, &right_tokens);

        if compare_result>0 && startup::is_debug() {
            applog!("Fail: right list greater than left list.");
        }
        
        return compare_result;
    }
    
    if left_data.contains(open_quote) || right_data.contains(open_quote) {
        let left_string: String = String::from(left_data);
        let right_string: String = String::from(right_data);
        
        compare_result = compare_strings(&left_string, &right_string);
        if compare_result>0 && startup::is_debug() {
            applog!("Fail: right string greater than left string.");
        }
        return compare_result;
    }

    let left_value: i32 = left_data.parse::<i32>().unwrap();
    let right_value: i32 = right_data.parse::<i32>().unwrap();

    if left_value == right_value {
        compare_result = 0;
    } else if left_value < right_value {
        compare_result = -1;
    } else if right_value > left_value {
        compare_result = 1;
    }

    return compare_result;
}

// <0: Left < Right
// =0: Left = Right
// >0: Left > Right (i.e. wrong order)

fn compare_lists(left: &Vec<String>, right: &Vec<String>) -> i32 {

    let mut compare_result: i32 = 1; // assume wrong order

    let num_left = left.len();
    let num_right = right.len();

    if startup::is_debug() {
        applog!("Comparing lists: left has {} items, right has {} items.", num_left, num_right);
    }

    let mut index: usize = 0;
    while index<num_left && index<num_right {
        compare_result = compare_strings(&left[index], &right[index]);
        if compare_result != 0 {
            // We have a result, terminate the comparison
            return compare_result;
        }
        index += 1;
    }

    // If right side is smaller, then not in the right order
    if index < num_left {
        compare_result = 1;
        if startup::is_debug() {
            applog!("Fail: num_left={}, num_right={}", num_left, num_right);
        }
    }

    return compare_result;
}

/*------------------------------------------------------------------ part2 - */

fn part2(reader: BufReader<File>) {

    // Read the file
    for line in reader.lines() {
        let _line = line.unwrap();
    }

    applog!("Not yet implemented.");
}

/*--------------------------------------------------------------- tokenise - */

fn tokenise(input: &str, tokens: &mut Vec<String>) {

    let mut i: usize = 0;
    while i<input.len() {
        let mut end_index: usize = 0;
        get_next_sb_field(input, i, &mut end_index);
        let field: &str = &input[i..end_index];
        tokens.push(String::from(field));
        i = end_index + 1;
    }
}

/*--------------------------------------------------------- get_next_field - */

fn get_next_field(input: &str, start_index: usize, end_index: &mut usize, sep: char, open_quote: char, close_quote: char) {

    let mut i: usize = start_index;
    let input_len: usize = input.len();
    let mut unclosed_quotes: i32 = 0;
    let mut expect_separator: bool = false;
    while i<input_len {
        let c = input.chars().nth(i).unwrap();
        if expect_separator && c!=sep {
            panic!("Invalid: serial quoted fields rather than nested: i={}, input={}.", i, input);
        } else if c == open_quote {
            unclosed_quotes += 1;
        } else if c == close_quote {
            unclosed_quotes -= 1;
            
            if unclosed_quotes==0 && i!=(input_len-1) {
                expect_separator = true;
            }
        } else if c == sep {
            if unclosed_quotes==0 {
                break;
            }
        }

        i += 1;
    }

    *end_index = i;

    if unclosed_quotes!=0 {
        panic!("Invalid quoting, unclosed_quotes={}", unclosed_quotes);
    }

}

/*------------------------------------------------------ get_next_sb_field - */

fn get_next_sb_field(input: &str, start_index: usize, end_index: &mut usize) {
    get_next_field(input, start_index, end_index, ',', '[', ']');
}

/*---------------------------------------------------- get_next_field_unquoted - */

fn get_next_field_unquoted<'a>(input: &'a str, start_index: usize, end_index: &mut usize, sep: char, open_quote: char, close_quote: char, unquoting_count: &mut u32) -> &'a str {

    let first_char: char = input.chars().nth(start_index).unwrap();
    let quoted: bool = if first_char == open_quote {true} else {false};

    if quoted {
        get_upto_close(input, start_index+1, end_index, open_quote, close_quote);
    } else {
        get_next_field(input, start_index, end_index, sep, open_quote, close_quote);
    }
    
    let input_start = if quoted {start_index + 1} else {start_index};
    let next_field_unquoted: &str = &input[input_start..*end_index];
    
    *unquoting_count = if quoted {1} else {0};
    return get_full_unquoted_field(next_field_unquoted, open_quote, close_quote, unquoting_count);
}

/*------------------------------------------------------ get_next_sb_field - */

fn get_next_sb_field_unquoted<'a>(input: &'a str, start_index: usize, end_index: &mut usize, unquoting_count: &mut u32) -> &'a str {
    return get_next_field_unquoted(input, start_index, end_index, ',', '[', ']', unquoting_count);
}

/*--------------------------------------------------------- get_next_field - */

fn get_upto_close(input: &str, start_index: usize, end_index: &mut usize, open_quote: char, close_quote: char) {

    let mut i: usize = start_index;
    let input_len: usize = input.len();
    while i<input_len {
        let c = input.chars().nth(i).unwrap(); 
        if c == open_quote {
            get_upto_close(input, i+1, end_index, open_quote, close_quote);
            i = *end_index;
        } else if c == close_quote {
            *end_index = i;
            break;
        }

        i += 1;
    }
}

fn is_full_field_quoted(input: &str, open_quote: char, close_quote: char) -> bool {

    // Test for empty field
    if input.len() == 0 {
        return false;
    }

    let mut unclosed_quotes: u32 = 0;
    let input_len: usize = input.len();
    for i in 0..input_len {
        let c = input.chars().nth(i).unwrap(); 
        if c==open_quote {
            unclosed_quotes += 1;
        } else if c==close_quote {
            unclosed_quotes -= 1;
            if unclosed_quotes==0 && i!=(input_len-1) {
                return false; // serial quoted fields rather than nested
            }
        } else if i==0 {
            return false; // should have been trapped by open_quote
        } else if i==input_len-1 {
            return false; // should have been trapped by close_quote
        }
    }

    if unclosed_quotes!=0 {
        panic!("Invalid quoting, unclosed_quotes={}", unclosed_quotes);
    }

    return true;
}

fn get_full_unquoted_field<'a>(next_field_unquoted: &'a str, open_quote: char, close_quote: char, unquoting_count: &mut u32) -> &'a str {

    if is_full_field_quoted(next_field_unquoted, open_quote, close_quote) {
        *unquoting_count = *unquoting_count + 1; 
        let len = next_field_unquoted.len();
        return get_full_unquoted_field(&next_field_unquoted[1..len-1], open_quote, close_quote, unquoting_count);
    }

    return next_field_unquoted;
}

/*--------------------------------------------------------- End of main.rs - */
use std::io::{BufRead, BufReader};
use std::fs::File;

#[macro_use]
mod applog;
mod startup;

#[derive(Debug, Clone)]
struct Directory {
    name: String,
    depth: i32,
    subdirs: Vec<String>,
    local_bytes: i32,
    subdir_bytes: i32,
}
impl Default for Directory {
    fn default () -> Directory {
        Directory{name: String::default(), depth: 0, subdirs: vec![], local_bytes: 0, subdir_bytes: 0}
    }
}

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

    let mut directories: Vec<Directory> = vec![];
    read_shell_history(reader, &mut directories);

    // Finally get total of directories with at most 100000 bytes
    let mut total_100k: i32 = 0;
    applog!("Examining: {} directories...", directories.len());
    for d in directories.iter() {
        applog!("{:?}", d);
        let total_bytes = d.local_bytes+d.subdir_bytes;
        if total_bytes <= 100000 {
            total_100k += total_bytes;
        }
    }

    applog!("Total of directories <=100k: {}", total_100k);
}

/*------------------------------------------------------------------ part2 - */

fn part2(reader: BufReader<File>) {

    let mut directories: Vec<Directory> = vec![];
    read_shell_history(reader, &mut directories);

    let total_space: i32 = 70000000;
    let required_unused: i32 = 30000000;
    let total_used = directories[0].local_bytes + directories[0].subdir_bytes;
    let free_space = total_space - total_used;

    if free_space >= required_unused {
        applog!("We have enough space! (With {} bytes to spare).", free_space - required_unused);
    } else {
        let bytes_required = required_unused - free_space;
        applog!("We don't have enough space! (We need {} more bytes).", bytes_required);

        // Find smallest directory that's big enough
        let mut best_folder_index=0;
        let mut best_bytes_so_far = total_used;
        for (i, d) in directories.iter().enumerate() {
            let dir_bytes = d.local_bytes + d.subdir_bytes;
            if dir_bytes >= bytes_required && dir_bytes < best_bytes_so_far {
                // We have a new leader
                best_folder_index = i;
                best_bytes_so_far = dir_bytes;
            }
        }

        applog!("Folder index {} will do the job: {} bytes.", best_folder_index, best_bytes_so_far);
    }
}

/*----------------------------------------------------- read_shell_history - */

fn read_shell_history(reader: BufReader<File>, directories: &mut Vec<Directory>) {

    //let mut directories: Vec<Directory> = vec![];
    let mut index: usize = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        // New directory
        if line.starts_with("$ cd ") {
            let name = &line[5..];
            change_directory(name, directories, &mut index);
            continue;
        } else if line == "$ ls" {
            continue; // skip
        }

        let mut dir: &mut Directory = &mut directories[index];
        let chunks: Vec<&str> = line.split(" ").collect();
                       
        if line.starts_with("dir ") {
            let subdir: &str = chunks[1];
            dir.subdirs.push(subdir.to_string());
        } else {
            let file_size: i32 = chunks[0].parse::<i32>().unwrap();
            dir.local_bytes += file_size;
        }
    }

    add_subdir_bytes(directories);
}

/*------------------------------------------------------- change_directory - */

fn change_directory(name: &str, directories: &mut Vec<Directory>, index: &mut usize) {

    let mut add_dir: bool = false;

    if name == "/" {
        *index = 0; // back to root
    } else if name == ".." {
        if *index>0 {
            *index = get_parent_index(&directories[*index].name, directories);
        }
    } else {
        add_dir = true;
    }

    if directories.len()==0 {
        add_dir = true;
    }

    if add_dir {
        add_directory(name, directories, index);
    }
}

/*---------------------------------------------------------- add_directory - */

fn add_directory(name: &str, directories: &mut Vec<Directory>, index: &mut usize) {
    let mut dir = Directory::default();
    // Non-root directories must be prefixed with parent
    if directories.len()==0 {
        dir.name = String::from(name);
    } else {
        let parent = &directories[*index];

        if parent.name == "/" {
            dir.name = format!("/{}", name);
        } else {
            dir.name = format!("{}/{}", parent.name, name);
        }
        dir.depth = parent.depth + 1;
    }
    
    directories.push(dir);
    *index = directories.len() - 1;
}

/*--------------------------------------------------------- get_parent_len - */

// NB: parent of "/" is ""!

fn get_parent_len(name: &String, len: usize) -> usize {
    let current_dir: &str = &name[..len];
    let mut parent_len: usize = current_dir.rfind('/').unwrap();
    if parent_len == 0 {
        if current_dir.len()==1 {
            parent_len = 0;
        } else {
            parent_len = 1;
        }
    }
    return parent_len;
}

/*------------------------------------------------------- get_folder_index - */

fn get_folder_index(name: &str, directories: &Vec<Directory>) -> usize {
    let mut index = 0;
    for (i, d) in directories.iter().enumerate() {
        if d.name == name {
            index = i;
            break;
        }
    }
    
    return index;
}

fn get_parent_index(name: &String, directories: &Vec<Directory>) -> usize {
    let parent_len = get_parent_len(name, name.len());
    return get_folder_index(&name[..parent_len], directories);
}
/*------------------------------------------------------- add_subdir_bytes - */

fn add_subdir_bytes(directories: &mut Vec<Directory>) {

    let mut i=0;
    while i<directories.len() {
        let mut parent_len: usize = directories[i].name.len();
        parent_len = get_parent_len(&directories[i].name, parent_len);
        while parent_len>0 {
            let parent_index = get_folder_index(&directories[i].name[..parent_len], directories);
            directories[parent_index].subdir_bytes += directories[i].local_bytes + directories[i].subdir_bytes;
            parent_len = get_parent_len(&directories[i].name, parent_len);
        }
        i += 1;
    }
}

/*--------------------------------------------------------- End of main.rs - */
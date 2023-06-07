use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Peekable;
use std::vec;

const FILESYSTEM_TOTAL_SIZE: i32 = 70000000;
const FILESYSTEM_NECESSARY_SIZE: i32 = 30000000;

#[derive(Debug)]
enum EntryType {
    File(i32),
    Directory(Vec<Entry>),
}

#[derive(Debug)]
struct Entry {
    name: String,
    entry_type: EntryType,
}

fn main() {
    let input = File::open("input.txt").unwrap();
    let lines = io::BufReader::new(input)
        .lines()
        .into_iter()
        .map(|l| l.unwrap());

    let mut filesystem: Entry = Entry {
        name: "/".to_owned(),
        entry_type: EntryType::Directory(vec![]),
    };

    let mut line_iterator = lines.skip(1).peekable();

    command_cd(&mut filesystem, &mut line_iterator);

    let total_size = determine_size_of_entry(&filesystem);
    println!("total filesystem size: '{:?}' bytes", total_size);
    println!(
        "total size of directories smaller than '100000' bytes: '{:?}' bytes",
        determine_smaller_than_100000(&filesystem)
    );

    println!(
        "smallest dir to delete to have '{:?}' bytes: '{:?}' bytes",
        FILESYSTEM_NECESSARY_SIZE,
        determine_smallest_to_delete(total_size, &filesystem)
    );
}

fn determine_size_of_entry(entry: &Entry) -> i32 {
    match &entry.entry_type {
        EntryType::File(size) => *size,
        EntryType::Directory(entries) => entries
            .iter()
            .map(|e| determine_size_of_entry(e))
            .sum::<i32>(),
    }
}

fn determine_smaller_than_100000(entry: &Entry) -> i32 {
    match &entry.entry_type {
        EntryType::File(_) => panic!("Cannot read file in this method"),
        EntryType::Directory(entries) => {
            let mut total_size_of_folders_sm_than_100000 = 0;

            for e in entries {
                match &e.entry_type {
                    EntryType::File(_) => {} // skip files
                    EntryType::Directory(_) => {
                        let size_of_this_directory = determine_size_of_entry(&e);

                        total_size_of_folders_sm_than_100000 +=
                            match size_of_this_directory {
                                0..=100000 => size_of_this_directory,
                                _ => 0,
                            } + determine_smaller_than_100000(&e)
                    }
                }
            }

            total_size_of_folders_sm_than_100000
        }
    }
}

fn determine_smallest_to_delete(current_total_size: i32, entry: &Entry) -> Option<i32> {
    match &entry.entry_type {
        EntryType::File(_) => panic!("Cannot read file in this method"),
        EntryType::Directory(entries) => {
            let current_directory_size = determine_size_of_entry(&entry);
            let mut smallest_to_delete: Option<i32> = None;

            if FILESYSTEM_TOTAL_SIZE - current_total_size + current_directory_size
                >= FILESYSTEM_NECESSARY_SIZE
            {
                smallest_to_delete = Some(current_directory_size);
            }

            for e in entries {
                match &e.entry_type {
                    EntryType::File(_) => {} // skip files
                    EntryType::Directory(_) => {
                        let smallest_to_delete_in_this_directory =
                            determine_smallest_to_delete(current_total_size, &e);

                        if let Some(smallest) = smallest_to_delete_in_this_directory {
                            if smallest_to_delete.is_none()
                                || smallest_to_delete.unwrap() > smallest
                            {
                                smallest_to_delete = smallest_to_delete_in_this_directory;
                            }
                        }
                    }
                }
            }

            smallest_to_delete
        }
    }
}

fn command_cd(entry: &mut Entry, line_iterator: &mut Peekable<impl Iterator<Item = String>>) {
    lazy_static! {
        static ref REG_CD: Regex = Regex::new(r"^\$\s+cd\s+(\.\.|\w+)\s*$").unwrap();
        static ref REG_LS: Regex = Regex::new(r"^\$\s+ls\s*$").unwrap();
    }

    while let Some(line) = line_iterator.next() {
        if let Some(cd_capture) = REG_CD.captures(&line) {
            if &cd_capture[1] == ".." {
                return;
            }

            match entry.entry_type {
                EntryType::File(_) => panic!("Cannot cd from file"),
                EntryType::Directory(ref mut d) => {
                    for e in d {
                        if e.name == cd_capture[1] {
                            command_cd(e, line_iterator);
                            break;
                        }
                    }
                }
            }
        }

        if let Some(_) = REG_LS.captures(&line) {
            match entry.entry_type {
                EntryType::File(_) => panic!("Cannot ls a file"),
                EntryType::Directory(ref mut d) => command_ls(d, line_iterator),
            };
        }
    }
}

fn command_ls(entries: &mut Vec<Entry>, lines: &mut Peekable<impl Iterator<Item = String>>) {
    lazy_static! {
        static ref REG_ENTRY: Regex = Regex::new(r"^(dir|\d+)\s+(\S+)\s*$").unwrap();
    }

    while let Some(line) = lines.next() {
        let capture = REG_ENTRY.captures(&line).unwrap();

        entries.push(Entry {
            name: capture[2].to_owned(),
            entry_type: match &capture[1] {
                "dir" => EntryType::Directory(vec![]),
                a => EntryType::File(a.parse::<i32>().expect("Cannot parse expected number")),
            },
        });

        let peek = lines.peek();
        if peek != None && &peek.unwrap()[..1] == "$" {
            return;
        }
    }
}

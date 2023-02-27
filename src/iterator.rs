use crate::dictionary::*;
use std::io::{self, prelude::*, BufReader};
use std::fs::File;

struct EntryIterator {
    reader: BufReader<File>,
}

impl Iterator for EntryIterator {
    type Item = Entry;
    fn next(&mut self) -> Option<Self::Item> {
        let mut lines = Vec::new();
        loop {
            let mut buffer = String::new();
            match self.reader.read_line(&mut buffer) {
                Ok(0) => break, // end of line
                Ok(_) => {
                    let line = buffer.trim().to_string();
                    lines.push(line.to_string());   // yes, this includes pushing the empty line in order to
                                                    // append a new line character to terminate the sentence
                    if line.is_empty() {
                        break;
                    }
                },
                Err(_) => break,
            }
        }
        let raw = lines.join("\n");
        let entry = parse_entry(raw.as_str());
        Some(entry)
    }
}

impl EntryIterator {
    fn new(file: File) -> EntryIterator {
        EntryIterator {
            reader: BufReader::new(file),
        }
    }
}

#[test]
fn test_parse_entry_by_entry_from_file() {
    use std::fs::File;

    let file = File::open("src/viet-eng.txt")
        .expect("Could not open file");
    let mut entry_iterator = EntryIterator::new(file);
    
    let first_entry: Entry = entry_iterator.next().unwrap();
    let first_expected = Entry {
        word: "00-database-info".to_string(),
        defs: vec![
            Definition {
                part_of_speech: "".to_string(),
                definition: "This is the Vietnamese-English dictionary \
                    database of the Free Vietnamese Dictionary Project. \
                    It contains more than 23.400 entries with definitions \
                    and illustrative examples.".to_string(),
                sentences: vec![],
            },
            Definition {
                part_of_speech: "".to_string(),
                definition: "This database was compiled by Ho Ngoc Duc and \
                    other members of the Free Vietnamese Dictionary Project \
                    (http://www.informatik.uni-leipzig.de/~duc/Dict/)".to_string(),
                sentences: vec![],
            },
            Definition {
                part_of_speech: "".to_string(),
                definition: "Copyright (C) 1997-2003 The Free Vietnamese Dictionary Project".to_string(),
                sentences: vec![],
            },
            Definition {
                part_of_speech: "".to_string(),
                definition: "This program is free software; you can redistribute it and/or \
                    modify it under the terms of the GNU General Public License as published by \
                    the Free Software Foundation; either version 2 of the License, or (at your \
                    option) any later version. This program is distributed in the hope that it \
                    will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty \
                    of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General \
                    Public License for more details.".to_string(),
                sentences: vec![],
            },
        ],
    };

    let second_entry: Entry = entry_iterator.next().unwrap();
    let second_expected = Entry {
        word: "00-database-short".to_string(),
        defs: vec![
            Definition {
                part_of_speech: "".to_string(),
                definition: "FVDP Vietnamese-English dictionary".to_string(),
                sentences: vec![],
            },
        ],
    };

    let third_entry: Entry = entry_iterator.next().unwrap();
    let third_expected = Entry {
        word: "00-database-url".to_string(),
        defs: vec![
            Definition {
                part_of_speech: "".to_string(),
                definition: "http://www.informatik.uni-leipzig.de/~duc/Dict/".to_string(),
                sentences: vec![],
            },
        ],
    };

    assert_eq!(first_entry, first_expected);
    assert_eq!(second_entry, second_expected);
    assert_eq!(third_entry, third_expected);
}

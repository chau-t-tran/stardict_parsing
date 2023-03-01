use crate::dictionary::*;
use std::fs::File;
use std::io::{self, prelude::*, BufReader, SeekFrom};
use pretty_assertions::{assert_eq, assert_ne};

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
                Ok(0) => return None, // end of line
                Ok(_) => {
                    let line = buffer.trim().to_string();
                    if line.contains("@") && lines.len() > 0 {
                        self.reader.seek(SeekFrom::Current(-((line.len()+2) as i64)));
                        break;
                    }
                    lines.push(line.to_string()); // yes, this includes pushing the empty line in order to
                }
                Err(_) => return None,
            }
        }
        let raw = lines.join("\n");
        println!("INPUT: {}", raw);
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

    let file = File::open("src/viet-eng.txt").expect("Could not open file");
    let mut entry_iterator = EntryIterator::new(file);

    let first_entry: Entry = entry_iterator.next().unwrap();
    let first_expected = Entry {
        word: TAPair::new("00-database-info".to_string()),
        defs: vec![
            Definition {
                part_of_speech: TAPair::new("".to_string()),
                definition: TAPair::new(
                    "This is the Vietnamese-English dictionary \
                    database of the Free Vietnamese Dictionary Project. \
                    It contains more than 23.400 entries with definitions \
                    and illustrative examples."
                        .to_string(),
                ),
                sentences: vec![],
            },
            Definition {
                part_of_speech: TAPair::new("".to_string()),
                definition: TAPair::new(
                    "This database was compiled by Ho Ngoc Duc and \
                    other members of the Free Vietnamese Dictionary Project \
                    (http://www.informatik.uni-leipzig.de/~duc/Dict/)"
                        .to_string(),
                ),
                sentences: vec![],
            },
            Definition {
                part_of_speech: TAPair::new("".to_string()),
                definition: TAPair::new(
                    "Copyright (C) 1997-2003 The Free Vietnamese Dictionary Project".to_string(),
                ),
                sentences: vec![],
            },
            Definition {
                part_of_speech: TAPair::new("".to_string()),
                definition: TAPair::new(
                    "This program is free software; you can redistribute it and/or \
                    modify it under the terms of the GNU General Public License as published by \
                    the Free Software Foundation; either version 2 of the License, or (at your \
                    option) any later version. This program is distributed in the hope that it \
                    will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty \
                    of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General \
                    Public License for more details."
                        .to_string(),
                ),
                sentences: vec![],
            },
        ],
    };

    let second_entry: Entry = entry_iterator.next().unwrap();
    let second_expected = Entry {
        word: TAPair::new("00-database-short".to_string()),
        defs: vec![Definition {
            part_of_speech: TAPair::new("".to_string()),
            definition: TAPair::new("FVDP Vietnamese-English dictionary".to_string()),
            sentences: vec![],
        }],
    };

    let third_entry: Entry = entry_iterator.next().unwrap();
    let third_expected = Entry {
        word: TAPair::new("00-database-url".to_string()),
        defs: vec![Definition {
            part_of_speech: TAPair::new("".to_string()),
            definition: TAPair::new("http://www.informatik.uni-leipzig.de/~duc/Dict/".to_string()),
            sentences: vec![],
        }],
    };

    assert_eq!(first_entry, first_expected);
    assert_eq!(second_entry, second_expected);
    assert_eq!(third_entry, third_expected);
}

#[test]
fn test_parse_until_end() {
    use std::fs::File;

    let file = File::open("src/viet-eng.txt").expect("Could not open file");
    let mut entry_iterator = EntryIterator::new(file);

    for value in entry_iterator {
        println!("{:?}", value);
    }
}

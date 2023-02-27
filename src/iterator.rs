use crate::dictionary::*;

#[test]
fn test_parse_entry_by_entry_from_file() {
    use std::fs::File;

    let file = File::open("viet-eng.txt");
    let entry_iterator = EntryIterator::new(file);
    let entry: Entry = entry_iterator.next();
    let expected = Entry {
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
        ]
    };
    assert_eq!(entry, expected);
}

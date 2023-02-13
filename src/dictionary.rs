#[derive(PartialEq, Debug)]
pub struct Word {
    pub word: String,
    pub pronunciation: String,
    pub defs: Vec<Definition>,
}

#[derive(PartialEq, Debug)]
pub struct Definition {
    pub part_of_speech: String,
    pub definition: String,
    pub sentences: Vec<Sentence>,
}

#[derive(PartialEq, Debug)]
pub struct Sentence {
    pub viet: String,
    pub eng: String,
}

fn parse_stardict(raw: String) -> Word {
    Word { 
        word: "".to_owned(), 
        pronunciation: "".to_owned(), 
        defs: vec![] 
    }
}

#[test]
fn test_parse_stardict() {
    let raw = "an phận	@an phận\n* verb\n- To feel smug\n=tư tưởng \
               an phận+Smugness, smug feeling\n=an phận thủ thường+to \
               feel smug about one's present circumstances".to_owned();

    let word = Word {
        word: "an phận".to_owned(),
        pronunciation: "".to_owned(),
        defs: vec![
            Definition {
                part_of_speech: "verb".to_owned(),
                definition: "To feel smug".to_owned(),
                sentences: vec![
                    Sentence { 
                        viet: "tư tưởng an phận".to_owned(), 
                        eng: "Smugness, smug feeling".to_owned(),
                    },
                    Sentence { 
                        viet: "an phận thủ thường".to_owned(), 
                        eng: "to feel smug about one's present circumstances".to_owned(),
                    },
                ],
            },
        ],
    };

    assert_eq!(parse_stardict(raw), word);
}

use pest::iterators::Pair;
use pest::Parser;
use std::fs::read_to_string;
use pretty_assertions::{assert_eq, assert_ne};

#[derive(Parser)]
#[grammar = "stardict_grammar.pest"]
pub struct StardictParser;

#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct TAPair {
    pub text: String,
    pub audio_url: String,
}

impl TAPair {
    pub fn new(text: String) -> Self {
        TAPair {
            text,
            audio_url: "".to_string(),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct Sentence {
    pub viet: TAPair,
    pub eng: TAPair,
}

#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct Sense {
    pub part_of_speech: TAPair,
    pub definition: TAPair,
    pub sentences: Vec<Sentence>,
}

#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct Entry {
    pub word: TAPair,
    pub senses: Vec<Sense>,
}

#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct Dictionary {
    pub entries: Vec<Entry>,
}

fn parse_sentence(p: Pair<Rule>) -> Sentence {
    let mut sentence: Sentence = Default::default();
    for field in p.into_inner() {
        match field.as_rule() {
            Rule::viet => sentence.viet = TAPair::new(field.as_str().trim().to_string()),
            Rule::eng => sentence.eng = TAPair::new(field.as_str().trim().to_string()),
            _ => println!("Error, fields not found"),
        }
    }
    sentence
}

fn parse_sense(p: Pair<Rule>) -> Sense {
    let mut sense: Sense = Default::default();
    for field in p.into_inner() {
        match field.as_rule() {
            Rule::part_of_speech => {
                sense.part_of_speech = TAPair::new(field.as_str().trim().to_string())
            }
            Rule::definition => {
                sense.definition = TAPair::new(field.as_str().trim().to_string())
            }
            Rule::sentence => sense.sentences.push(parse_sentence(field)),
            _ => println!("Error, fields not found"),
        }
    }
    sense
}

pub fn parse_entry(raw: &str) -> Entry {
    let data = StardictParser::parse(Rule::entry, raw)
        .expect("cannot parse")
        .next()
        .unwrap();
    let mut entry: Entry = Default::default();
    for field in data.into_inner() {
        match field.as_rule() {
            Rule::word => entry.word = TAPair::new(field.as_str().trim().to_string()),
            Rule::sense => entry.senses.push(parse_sense(field)),
            _ => println!("Error, fields not found"),
        }
    }
    entry
}

#[test]
fn test_parse_entry_basic() {
    let raw = "@an phận\n\
        * verb\n\
        - To feel smug\n\
        =tư tưởng an phận+Smugness, smug feeling\n\
        =an phận thủ thường+to feel smug about one's present circumstances \n\n";

    let entry = Entry {
        word: TAPair::new("an phận".to_string()),
        senses: vec![Sense {
            part_of_speech: TAPair::new("verb".to_string()),
            definition: TAPair::new("To feel smug".to_string()),
            sentences: vec![
                Sentence {
                    viet: TAPair::new("tư tưởng an phận".to_string()),
                    eng: TAPair::new("Smugness, smug feeling".to_string()),
                },
                Sentence {
                    viet: TAPair::new("an phận thủ thường".to_string()),
                    eng: TAPair::new("to feel smug about one's present circumstances".to_string()),
                },
            ],
        }],
    };

    assert_eq!(parse_entry(raw), entry);
}

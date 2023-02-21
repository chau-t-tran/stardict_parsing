use pest::Parser;
use pest::iterators::Pair;
use std::fs::read_to_string;

#[derive(Parser)]
#[grammar = "stardict_grammar.pest"]
pub struct StardictParser;

#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct Sentence {
    viet: String,
    eng: String,
}

#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct Definition {
    part_of_speech: String,
    definition: String,
    sentences: Vec<Sentence>,
}

#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct Word {
    spelling: String,
    pronunciation: String,
}

#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct Entry {
    word: Word,
    defs: Vec<Definition>,
}

#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct Dictionary {
    entries: Vec<Entry>
}

fn parse_sentence(p: Pair<Rule>) -> Sentence {
    let mut sentence: Sentence = Default::default();
    for field in p.into_inner() {
        match field.as_rule() {
            Rule::viet => sentence.viet = field.as_str().to_string(),
            Rule::eng => sentence.eng = field.as_str().to_string(),
            _ => println!("Error, fields not found"),
        }
    }
    sentence
}

fn parse_def(p: Pair<Rule>) -> Definition {
    let mut definition: Definition = Default::default();
    for field in p.into_inner() {
        match field.as_rule() {
            Rule::part_of_speech => definition.part_of_speech = field.as_str().to_string(),
            Rule::description => definition.definition = field.as_str().to_string(),
            Rule::sentence => definition.sentences.push(parse_sentence(field)),
            _ => println!("Error, fields not found"),
        }
    }
    definition
}

fn parse_word(p: Pair<Rule>) -> Word {
    let mut word: Word = Default::default();
    for field in p.into_inner() {
        match field.as_rule() {
            Rule::spelling => word.spelling = field.as_str().to_string(),
            Rule::pronunciation => word.pronunciation = field.as_str().to_string(),
            _ => println!("Error, fields not found"),
        }
    }
    word
}

fn parse_entry(p: Pair<Rule>) -> Entry {
    let mut entry: Entry = Default::default();
    for field in p.into_inner() {
        match field.as_rule() {
            Rule::word => entry.word = parse_word(field),
            Rule::definition => entry.defs.push(parse_def(field)),
            _ => println!("Error, fields not found"),
        }
    }
    entry
}

pub fn parse_stardict(raw: &str) -> Dictionary {
    let data = StardictParser::parse(Rule::dict, raw)
        .expect("cannot parse")
        .next()
        .unwrap();

    let mut dict: Dictionary = Default::default();
    for entry in data.into_inner() {
        match entry.as_rule() {
            Rule::entry => dict.entries.push(parse_entry(entry)),
            _ => println!("Error: not entry"),
        }
    }
    dict
}

#[test]
fn test_parse_stardict() {
    let raw = "an phận	@an phận\\n* verb\\n- To feel smug\\n=tư tưởng \
   an phận+Smugness, smug feeling\\n=an phận thủ thường+to \
   feel smug about one's present circumstances\n";

    let dict = Dictionary {
        entries: vec![
            Entry {
                word: Word {
                    spelling: "an phận".to_string(),
                    pronunciation: "".to_string(),
                },
                defs: vec![
                    Definition {
                        part_of_speech: "verb".to_string(),
                        definition: "To feel smug".to_string(),
                        sentences: vec![
                            Sentence { 
                                viet: "tư tưởng an phận".to_string(), 
                                eng: "Smugness, smug feeling".to_string(),
                            },
                            Sentence { 
                                viet: "an phận thủ thường".to_string(), 
                                eng: "to feel smug about one's present circumstances".to_string(),
                            },
                        ],
                    },
                ],
            },
        ]
    };

    assert_eq!(parse_stardict(raw), dict);
}

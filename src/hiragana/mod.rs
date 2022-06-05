use std::{collections::BTreeMap, fs, io::Error};

use serde::Deserialize;

pub mod tui;
mod ui;

fn read_file() -> Result<String, Error> {
    Ok(fs::read_to_string("src/hiragana/hiragana.txt")?)
}

#[derive(Debug, Deserialize)]
pub struct HiraganaCategories {
    pub v: BTreeMap<String, String>,
    pub k: BTreeMap<String, String>,
    pub s: BTreeMap<String, String>,
    pub t: BTreeMap<String, String>,
    pub n: BTreeMap<String, String>,
    pub h: BTreeMap<String, String>,
    pub m: BTreeMap<String, String>,
    pub r: BTreeMap<String, String>,
    pub y: BTreeMap<String, String>,
    pub w: BTreeMap<String, String>,
}

#[derive(Deserialize, Debug)]
pub struct Hiragana;

impl Hiragana {
    fn new<'a>() -> Result<Vec<(String, String)>, Error> {
        let file = read_file()?;

        // let json = include_str!("hj.json");
        // let hj: HiraganaCategories = serde_json::from_str(&json).unwrap();
        // println!("{:?}", hj);
        // panic!("");

        let mut hiragana: Vec<(String, String)> = Vec::new();
        file.lines().for_each(|l| {
            let mut split = l.split_whitespace();
            let (first, second) = (
                split.next().unwrap().to_string(),
                split.next().unwrap().to_string(),
            );
            hiragana.push((first, second));
        });

        Ok(hiragana)
    }
}

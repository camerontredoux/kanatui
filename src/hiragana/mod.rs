use std::{collections::HashMap, fs, io::Error};

use serde::Deserialize;

pub mod tui;
mod ui;

fn read_file() -> Result<String, Error> {
    Ok(fs::read_to_string("src/hiragana/hiragana.txt")?)
}

#[derive(Deserialize, Debug)]
pub struct Hiragana;

impl Hiragana {
    fn new<'a>() -> Result<Vec<(String, String)>, Error> {
        let file = read_file()?;
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

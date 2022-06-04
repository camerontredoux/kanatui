use std::error::Error;

use clap::{Parser, Subcommand};
mod hiragana;
mod katakana;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    kana: Kana,
}

#[derive(Debug, Subcommand)]
enum Kana {
    /// Starts hiragana practice
    Hiragana,
    /// Starts katakana practice
    Katakana,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    match args.kana {
        Kana::Hiragana => hiragana::tui::terminal()?,
        Kana::Katakana => katakana::tui::terminal()?,
    }
    Ok(())
}


use clap::{App, Arg, ArgMatches};
use regex::Regex;
use std::io::{self, Write};
use anyhow::{Context, Result};
use colored::*;

pub fn main() -> Result<()>{
    let matches = parse_commandline_args();

    let pattern = matches.value_of("pattern").unwrap();
    let path = matches.value_of("file").unwrap();
    let is_color = matches.is_present("color");

    let pattern = Regex::new(pattern)
        .with_context(|| "invalid pattern")?;

    let contents = std::fs::read_to_string(&path)
        .with_context(|| format!("could not read file '{}'", path))?;

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for line in contents.lines() {
        if let Some(matched) = pattern.find(line) {
            if is_color {
                writeln!(handle, "{}{}{}",
                         &line[..matched.start()],
                         &line[matched.start()..matched.end()].red(),
                         &line[matched.end()..])?;
            } else {
                writeln!(handle, "{}", line)?; 
            }
        }
    }

    Ok(())
}

fn parse_commandline_args() -> ArgMatches {
    App::new("mygrep")
        .version("1.0.0")
        .author("Aman Shaw  <amanshaw4511@protonmail.com>")
        .about("An alternative to grep")
        .arg(Arg::new("pattern")
             .short('p')
             .long("pattern")
             .value_name("PATTERN")
             .about("regex pattern to find")
             .takes_value(true)
             .required(true)
            )
        .arg(Arg::new("file")
             .short('f')
             .long("file")
             .value_name("FILE")
             .about("file to find for matches")
             .takes_value(true)
             .required(true)
            )
        .arg(Arg::new("color")
             .short('c')
             .long("color")
             .takes_value(false)
             .about("prints matching string in color")
             .required(false)
            )
        .get_matches()
}

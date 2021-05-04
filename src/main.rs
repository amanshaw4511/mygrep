
use clap::{App, Arg, ArgMatches};
use regex::{Match, Regex};
use std::io::{self, StdoutLock, Write};
use anyhow::{Context, Result};
use colored::*;

pub fn main() -> Result<()>{
    let args = parse_commandline_args();

    let pattern = args.value_of("pattern").unwrap();
    let path = args.value_of("file").unwrap();
    let has_color = args.is_present("color");
    let has_ignore_case = args.is_present("ignore_case");
    let has_invert_match = args.is_present("invert_match");


    let ignore_case_pattern = format!("(?i){}", pattern);

    let pattern = Regex::new(match has_ignore_case 
        {
            true => &ignore_case_pattern,
            false => pattern
        }).with_context(|| "invalid pattern")?;

    let contents = std::fs::read_to_string(&path)
        .with_context(|| format!("could not read file '{}'", path))?;

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for line in contents.lines() {
        match pattern.find(line) {
            Some(matched)  => {
                if !has_invert_match { 
                    print_matched_line(&mut handle, line, matched, has_color)?;
                };
            },

            None => {
                if has_invert_match {
                    writeln!(handle, "{}", line)?; 
                };
            }
        }
    }

    Ok(())
}

#[inline]
fn print_matched_line(handle: &mut StdoutLock,line: &str, matched: Match, has_color: bool) -> Result<()> {
    if has_color {
        writeln!(handle, "{}{}{}",
                 &line[..matched.start()],
                 &line[matched.start()..matched.end()].red(),
                 &line[matched.end()..])?;
    } else {
        writeln!(handle, "{}", line)?; 
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
             .takes_value(true)
             .required(true)
             .about("regex pattern to find")
            )
        .arg(Arg::new("file")
             .short('f')
             .long("file")
             .value_name("FILE")
             .takes_value(true)
             .required(true)
             .about("file to find for matches")
            )
        .arg(Arg::new("color")
             .short('c')
             .long("color")
             .takes_value(false)
             .required(false)
             .about("prints matching string in color")
            )
        .arg(Arg::new("ignore_case")
             .short('i')
             .long("ignore-case")
             .takes_value(false)
             .required(false)
             .about("ignore matching case")
            )
        .arg(Arg::new("invert_match")
             .short('v')
             .long("invert-match")
             .takes_value(false)
             .required(false)
             .about("prints not matching line")
             )
        .get_matches()
}


use clap::{App, Arg, ArgMatches};

pub fn main() {
    let matches = fun_name();

    let pattern = matches.value_of("pattern").unwrap();
    let path = matches.value_of("file").unwrap();

    let contents = std::fs::read_to_string(&path).expect("file not found");

    for line in contents.lines() {
        if line.contains(&pattern) {
            println!("{}", line);
            }
    }
}

fn fun_name() -> ArgMatches {
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
        .get_matches()
}

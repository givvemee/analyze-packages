// excuting command line args
use clap::{Arg, Command};

pub fn get_matches() -> clap::ArgMatches {
    Command::new("analyze-docs")
        .version("1.0")
        .author("givvemee <givvemeee@gmail.com>")
        .about("Analyzes package.json and fetches descriptions")
        .arg(Arg::new("file")
            .short('f')
            .long("file")
            .value_name("FILE")
            .help("Sets a custom package.json file"))
        .get_matches()
}

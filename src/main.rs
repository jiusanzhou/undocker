extern crate atty;
#[macro_use]
extern crate clap;
extern crate chrono;
#[macro_use]
extern crate lazy_static;
extern crate termcolor;

mod cli;
mod cmd;
mod console;

fn main() {
    let matches = cli::build_cli().get_matches();

    match matches.subcommand() {
        ("undocker", Some(_matches)) => {
            let f = matches.value_of("file").unwrap_or("./data/undocker.csv");
            cmd::undocker(f);
        }
        _ => unreachable!(),
    }
}
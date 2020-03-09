#![allow(dead_code)]
use std::env;
use std::io::Write;

use atty;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

lazy_static! {
    /// Termcolor color choice.
    /// We do not rely on ColorChoice::Auto behavior
    /// as the check is already performed by has_color.
    static ref COLOR_CHOICE: ColorChoice =
        if has_color() {
            ColorChoice::Always
        } else {
            ColorChoice::Never
        };
}

pub fn info(message: &str) {
    colorize(message, ColorSpec::new().set_bold(true));
}

pub fn warn(message: &str) {
    colorize(message, ColorSpec::new().set_bold(true).set_fg(Some(Color::Yellow)));
}

pub fn success(message: &str) {
    colorize(message, ColorSpec::new().set_bold(true).set_fg(Some(Color::Green)));
}

pub fn error(message: &str) {
    colorize(message, ColorSpec::new().set_bold(true).set_fg(Some(Color::Red)));
}

/// Print a colorized message to stdout
fn colorize(message: &str, color: &ColorSpec) {
    let mut stdout = StandardStream::stdout(*COLOR_CHOICE);
    stdout.set_color(color).unwrap();
    writeln!(&mut stdout, "{}", message).unwrap();
    stdout.set_color(&ColorSpec::new()).unwrap();
}

/// Check whether to output colors
fn has_color() -> bool {
    let use_colors = env::var("CLICOLOR").unwrap_or_else(|_| "1".to_string()) != "0"
        && env::var("NO_COLOR").is_err();
    let force_colors = env::var("CLICOLOR_FORCE").unwrap_or_else(|_| "0".to_string()) != "0";

    force_colors || use_colors && atty::is(atty::Stream::Stdout)
}
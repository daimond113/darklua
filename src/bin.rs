mod cli;

use std::{process,thread};

use clap::Parser;
use cli::Darklua;
use env_logger::{
    fmt::{Color, Style, StyledValue},
    Builder,
};
use log::Level;

const STACK_SIZE: usize = 10 * 1024 * 1024 * 1024;

fn run() {
    let darklua = Darklua::parse();

    let filter = darklua.get_log_level_filter();

    formatted_logger().filter_module("darklua", filter).init();

    match darklua.run() {
        Ok(()) => {}
        Err(err) => {
            process::exit(err.exit_code());
        }
    }
}

fn main() {
    // Spawn thread with explicit stack size
    let child = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(run)
        .unwrap();

    // Wait for thread to join
    child.join().unwrap();
}

fn formatted_logger() -> Builder {
    let mut builder = Builder::new();
    builder.format(|f, record| {
        use std::io::Write;

        let mut style = f.style();
        let level = colored_level(&mut style, record.level());

        writeln!(f, " {} > {}", level, record.args(),)
    });
    builder
}

fn colored_level(style: &mut Style, level: Level) -> StyledValue<&'static str> {
    match level {
        Level::Trace => style.set_color(Color::Magenta).value("TRACE"),
        Level::Debug => style.set_color(Color::Blue).value("DEBUG"),
        Level::Info => style.set_color(Color::Green).value("INFO"),
        Level::Warn => style.set_color(Color::Yellow).value("WARN"),
        Level::Error => style.set_color(Color::Red).value("ERROR"),
    }
}

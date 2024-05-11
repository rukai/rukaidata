use anstyle::{AnsiColor, Style};
use env_logger::fmt::Formatter;
use env_logger::Builder;
use log::{Level, Record};
use std::env;
use std::io;
use std::io::Write;

pub fn init() {
    if let Ok(env_var) = env::var("BW_LOG") {
        Builder::new().format(format).parse_filters(&env_var).init()
    }
}

fn format(buf: &mut Formatter, record: &Record) -> io::Result<()> {
    let level = record.level();
    let level_style = Style::new().fg_color(Some(
        match level {
            Level::Trace => AnsiColor::White,
            Level::Debug => AnsiColor::Blue,
            Level::Info => AnsiColor::Green,
            Level::Warn => AnsiColor::Yellow,
            Level::Error => AnsiColor::Red,
        }
        .into(),
    ));

    let write_level = write!(buf, "{level_style}{level:>5}{level_style:#}");
    let write_args = if let Some(module_path) = record.module_path() {
        writeln!(buf, " {} {}", module_path, record.args())
    } else {
        writeln!(buf, " {}", record.args())
    };

    write_level.and(write_args)
}

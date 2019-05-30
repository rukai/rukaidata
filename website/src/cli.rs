use std::env;

use getopts::Options;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] [list of fighters to export]", program);
    print!("{}", opts.usage(&brief));
}

pub(crate) fn parse_cli() -> Option<CLIResults> {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    let mut opts = Options::new();
    opts.optflag("g", "gif",      "Enable subaction gif generation");
    opts.optflag("w", "web",      "Enable website generation");
    opts.optopt( "m", "mods",     "List of mod folders in data/ to use", "NAME1,NAME2,NAME3...");
    opts.optopt( "f", "fighters", "List of fighters to use",             "NAME1,NAME2,NAME3...");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(_) => {
            print_usage(program, opts);
            return None;
        }
    };

    let mut fighter_names: Vec<String> = vec!();
    if let Some(f_match) = matches.opt_str("f") {
        for fighter_name in f_match.split(",") {
            fighter_names.push(fighter_name.to_lowercase());
        }
    }

    let mut mod_names = vec!();
    if let Some(m_match) = matches.opt_str("m") {
        for mod_name in m_match.split(",") {
            mod_names.push(mod_name.to_lowercase());
        }
    }

    let generate_gifs = matches.opt_present("g");
    let generate_web = matches.opt_present("w");

    Some(CLIResults { mod_names, fighter_names, generate_gifs, generate_web })
}

pub struct CLIResults {
    pub mod_names:     Vec<String>,
    pub fighter_names: Vec<String>,
    pub generate_gifs: bool,
    pub generate_web:  bool,
}

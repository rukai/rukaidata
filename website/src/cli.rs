use clap::Parser;

#[derive(Parser, Clone)]
pub struct Args {
    #[clap(long, short, value_delimiter = ',')]
    /// List of mod folders in data/ to use
    pub mod_names: Vec<String>,

    /// List of fighters to use
    #[clap(long, short, value_delimiter = ',')]
    pub fighter_names: Vec<String>,

    /// Enable subaction gif generation
    #[clap(long, short, action)]
    pub generate_gifs: bool,

    /// Enable website generation
    #[clap(long, short = 'w', action)]
    pub generate_web: bool,

    /// Use wasm/wgpu backend
    #[clap(long, short = 'a', action)]
    pub wasm_mode: bool,

    /// Serve the website at localhost:8000 after generating it
    #[clap(long, short)]
    #[clap(long, short, action)]
    pub serve: bool,
}

pub fn args() -> Args {
    let mut args = Args::parse();
    for m in &mut args.mod_names {
        m.make_ascii_lowercase();
    }
    for f in &mut args.fighter_names {
        f.make_ascii_lowercase();
    }
    args
}

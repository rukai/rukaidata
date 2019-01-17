use std::env;
use std::path::Path;
use simple_command::simple_command;

fn main() {
    let npm_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../npm-webpack");
    env::set_current_dir(&npm_dir).expect("Failed: cd ../npm-webpack");

    if !npm_dir.join("node_modules").exists() {
        simple_command("npm install");
    }
    simple_command("npm run debug");
}

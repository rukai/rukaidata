#![feature(rust_2018_preview, plugin, decl_macro, custom_derive, nll)]
#![warn(rust_2018_idioms)]
#![plugin(rocket_codegen)]

             extern crate brawllib_rs;
             extern crate getopts;
             extern crate rocket;
             extern crate rocket_contrib;
#[macro_use] extern crate log;

use rocket_contrib::Template;
use rocket::response::NamedFile;

use brawllib_rs::fighter::Fighter;
use brawllib_rs::high_level_fighter::HighLevelFighter;

use std::path::{Path, PathBuf};
use std::fs;

pub mod cli;

fn main() {
    let cli = if let Some(cli) = cli::parse_cli() {
        cli
    } else {
        return;
    };

    let data_dir = match fs::read_dir("data") {
        Ok(dir) => dir,
        Err(_) => {
            error!("Can't read 'data' directory.");
            return;
        }
    };

    let mut brawl_mods = vec!();
    for data in data_dir {
        let data = data.unwrap();
        let mod_name = data.file_name().into_string().unwrap();
        let lower_mod_name = mod_name.to_lowercase();
        if cli.mod_names.len() == 0 || cli.mod_names.iter().any(|x| x == &lower_mod_name) {
            let mut path = data.path();
            path.push("fighter");

            let mut high_level_fighters = vec!();
            match fs::read_dir(path) {
                Ok(fighter_dir) => {
                    let mod_fighter_dir = if lower_mod_name == "brawl" {
                        None
                    } else {
                        Some(fighter_dir)
                    };

                    let fighters = match fs::read_dir("data/brawl/fighter") {
                        Ok(brawl_dir) => Fighter::load(brawl_dir, mod_fighter_dir, true),
                        Err(_) => {
                            error!("Can't read 'data/brawl/fighter' directory.");
                            return;
                        }
                    };
                    for fighter in fighters {
                        let lower_fighter_name = fighter.cased_name.to_lowercase();
                        if cli.fighter_names.len() == 0 || cli.fighter_names.iter().any(|x| x == &lower_fighter_name) {
                            high_level_fighters.push(HighLevelFighter::new(&fighter));
                        }
                    }
                }
                Err(_) => {
                    error!("Failed to open fighter directory");
                    return;
                }
            };

            brawl_mods.push(BrawlMod {
                name:     mod_name,
                fighters: high_level_fighters
            });
        }
    }

    rocket::ignite()
        .manage(brawl_mods)
        .mount("/", routes![index, files])
        .attach(Template::fairing())
        .launch();
}

#[get("/static/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[get("/")]
fn index() -> Template {
    Template::render("index", 0)
}

struct BrawlMod {
    name:     String,
    fighters: Vec<HighLevelFighter>,
}

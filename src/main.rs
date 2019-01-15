#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

use rocket_contrib::templates::Template;
use rocket::response::NamedFile;

use brawllib_rs::fighter::Fighter;
use brawllib_rs::high_level_fighter::HighLevelFighter;

use std::path::{Path, PathBuf};
use std::fs;

pub mod cli;
pub mod logger;
pub mod page;
pub mod brawl_data;

use crate::brawl_data::{BrawlMods, BrawlMod};

fn main() {
    logger::init();
    let cli = if let Some(cli) = cli::parse_cli() {
        cli
    } else {
        return;
    };

    let data_dir = match fs::read_dir("data") {
        Ok(dir) => dir,
        Err(_) => {
            println!("Can't read 'data' directory.");
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

                    let fighters = match fs::read_dir("data/Brawl/fighter") {
                        Ok(brawl_dir) => Fighter::load(brawl_dir, mod_fighter_dir, true),
                        Err(_) => {
                            println!("Can't read 'data/Brawl/fighter' directory.");
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
                    println!("Failed to open fighter directory");
                    return;
                }
            };

            brawl_mods.push(BrawlMod {
                name:     mod_name,
                fighters: high_level_fighters
            });
        }
    }

    let brawl_mods = BrawlMods {
        mods: brawl_mods
    };

    rocket::ignite()
        .manage(brawl_mods)
        .mount("/", routes![files, page::index::serve, page::brawl_mod::serve, page::fighter::serve, page::action::serve])
        .attach(Template::fairing())
        .launch();
}

#[get("/dist/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("npm-webpack/dist").join(file)).ok()
}

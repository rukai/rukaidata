#[macro_use] extern crate serde_derive;

use handlebars::Handlebars;

use brawllib_rs::fighter::Fighter;
use brawllib_rs::high_level_fighter::HighLevelFighter;

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
    println!("brawl files loaded");

    let brawl_mods = BrawlMods {
        mods: brawl_mods
    };

    let mut handlebars = Handlebars::new();
    handlebars.register_templates_directory(".html.hbs", "templates").unwrap();
    println!("handlebars templates loaded");

    page::index::generate(&handlebars, &brawl_mods);
    page::brawl_mod::generate(&handlebars, &brawl_mods);
    page::fighter::generate(&handlebars, &brawl_mods);
    page::action::generate(&handlebars, &brawl_mods);
}

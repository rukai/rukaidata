#![feature(rust_2018_preview, plugin, decl_macro, custom_derive, nll)]
#![warn(rust_2018_idioms)]
#![plugin(rocket_codegen)]

             extern crate brawllib_rs;
             extern crate getopts;
             extern crate rocket;
             extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
             extern crate serde_json;

use rocket_contrib::Template;
use rocket::response::NamedFile;
use rocket::State;

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
        .mount("/", routes![files, index, page])
        .attach(Template::fairing())
        .launch();
}

#[get("/static/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

// TODO: Allow configuration of the default values or at least choose them smartly
#[get("/")]
fn index(brawl_mods: State<BrawlMods>) -> Template {
    let (mod_name, fighter_name) = if let Some(default_mod) = brawl_mods.mods.iter().find(|x| x.fighters.len() > 0) {
        if let Some(default_fighter) = default_mod.fighters.get(0) {
            (default_mod.name.clone(), default_fighter.name.clone())
        } else {
            let mod_links = brawl_mods.gen_mod_links(default_mod.name.clone());
            let error = format!("The default mod {} contains no fighters.", default_mod.name);
            return Template::render("error", ErrorPage { mod_links, error });
        }
    } else {
        let mod_links = brawl_mods.gen_mod_links(String::new());
        let error = String::from("No mods were loaded.");
        return Template::render("error", ErrorPage { mod_links, error });
    };

    page(brawl_mods, mod_name, fighter_name, String::from("Wait1"), 0)
}

#[get("/framedata/<mod_name>/<fighter_name>/<action_name>/<frame>")]
fn page(brawl_mods: State<BrawlMods>, mod_name: String, fighter_name: String, action_name: String, frame: usize) -> Template {
    let mod_links = brawl_mods.gen_mod_links(mod_name.clone());
    if let Some(brawl_mod) = brawl_mods.mods.iter().find(|x| x.name == mod_name) {
        if let Some(fighter) = brawl_mod.fighters.iter().find(|x| x.name == fighter_name) {
            if let Some(action) = fighter.actions.iter().find(|x| x.name == action_name) {
                if let Some(frame) = action.frames.get(frame) {
                    let page = Page {
                        mod_links,
                        title:         format!("{} - {} - {}", action_name, fighter_name, mod_name),
                        fighter_links: brawl_mod.gen_fighter_links(fighter_name),
                        action_links:  brawl_mod.gen_action_links(fighter, action_name),
                        scripts:       serde_json::to_string_pretty(&action.scripts).unwrap(),
                        frame:         serde_json::to_string_pretty(frame).unwrap(),
                    };
                    Template::render("page", page)
                } else {
                    let error = format!("The frame {} does not exist in action {} in fighter {} in mod {}.", frame, action_name, fighter_name, mod_name);
                    Template::render("error", ErrorPage { mod_links, error })
                }
            } else {
                let error = format!("The action {} does not exist in fighter {} in mod {}.", action_name, fighter_name, mod_name);
                Template::render("error", ErrorPage { mod_links, error })
            }
        } else {
            let error = format!("The Fighter {} does not exist in mod {}.", fighter_name, mod_name);
            Template::render("error", ErrorPage { mod_links, error })
        }
    } else {
        let error = format!("The mod {} does not exist.", mod_name);
        Template::render("error", ErrorPage { mod_links, error })
    }
}

struct BrawlMods {
    mods: Vec<BrawlMod>,
}

impl BrawlMods {
    fn gen_mod_links(&self, current_mod: String) -> Vec<NavLink> {
        let mut links = vec!();
        for brawl_mod in &self.mods { // TODO: Allow specify ordering either via config file or the order used in --mods NAME1,NAME2
            if let Some(fighter) = brawl_mod.fighters.get(0) {
                links.push(NavLink {
                    name:    brawl_mod.name.clone(),
                    link:    format!("/framedata/{}/{}/Wait1/0", brawl_mod.name, fighter.name),
                    current: brawl_mod.name == current_mod,
                });
            }
        }
        links
    }
}

struct BrawlMod {
    name:     String,
    fighters: Vec<HighLevelFighter>,
}

impl BrawlMod {
    fn gen_fighter_links(&self, current_fighter: String) -> Vec<NavLink> {
        let mut links = vec!();
        for fighter in &self.fighters {
            links.push(NavLink {
                name:    fighter.name.clone(),
                link:    format!("/framedata/{}/{}/Wait1/0", self.name, fighter.name),
                current: current_fighter == fighter.name,
            });
        }
        links
    }

    fn gen_action_links(&self, fighter: &HighLevelFighter, current_action: String) -> Vec<NavLink> {
        let mut links = vec!();
        for action in &fighter.actions {
            links.push(NavLink {
                name:    action.name.clone(),
                link:    format!("/framedata/{}/{}/{}/0", self.name, fighter.name, action.name),
                current: current_action == action.name,
            });
        }
        links
    }
}

#[derive(Serialize)]
struct Page {
    mod_links:     Vec<NavLink>,
    fighter_links: Vec<NavLink>,
    action_links:  Vec<NavLink>,
    title:         String,
    scripts:       String,
    frame:         String,
}

#[derive(Serialize)]
struct ErrorPage {
    mod_links: Vec<NavLink>,
    error:     String,
}

#[derive(Serialize)]
struct NavLink {
    name:    String,
    link:    String,
    current: bool,
}

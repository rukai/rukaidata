use std::fs;
use std::fs::DirEntry;
use std::collections::HashMap;

use brawllib_rs::fighter::Fighter;
use brawllib_rs::high_level_fighter::HighLevelFighter;

use crate::page::NavLink;
use crate::cli::CLIResults;

pub struct BrawlMods {
    pub mods: Vec<BrawlMod>,
}

pub struct BrawlMod {
    pub name:        String,
    pub fighters:    Vec<BrawlFighter>,
}

pub struct BrawlFighter {
    pub fighter: HighLevelFighter,
    pub script_lookup: HashMap<u32, ScriptInfo>,
}

pub struct ScriptInfo {
    pub name:    String,
    pub address: String,
}

impl BrawlMods {
    pub fn new(cli: &CLIResults) -> Option<BrawlMods> {
        match fs::read_dir("../data") {
            Ok(dir) => {
                Some(BrawlMods {
                    mods: dir.filter_map(|x| BrawlMod::new(x.unwrap(), &cli)).collect(),
                })
            }
            Err(_) => {
                println!("Can't read 'data' directory.");
                None
            }
        }
    }

    pub fn gen_mod_links(&self, current_mod: String) -> Vec<NavLink> {
        let mut links = vec!();
        for brawl_mod in &self.mods { // TODO: Allow specify ordering either via config file or the order used in --mods NAME1,NAME2
            links.push(NavLink {
                name:    brawl_mod.name.clone(),
                link:    format!("/{}", brawl_mod.name),
                current: brawl_mod.name == current_mod,
            });
        }
        links
    }
}

impl BrawlMod {
    pub fn new(data: DirEntry, cli: &CLIResults) -> Option<BrawlMod> {
        let mod_name = data.file_name().into_string().unwrap();
        let lower_mod_name = mod_name.to_lowercase();
        if cli.mod_names.len() == 0 || cli.mod_names.iter().any(|x| x == &lower_mod_name) {
            let mut path = data.path();
            path.push("fighter");

            let mut brawl_fighters = vec!();
            match fs::read_dir(path) {
                Ok(fighter_dir) => {
                    let mod_fighter_dir = if lower_mod_name == "brawl" {
                        None
                    } else {
                        Some(fighter_dir)
                    };

                    let fighters = match fs::read_dir("../data/Brawl/fighter") {
                        Ok(brawl_dir) => Fighter::load(brawl_dir, mod_fighter_dir, true),
                        Err(_) => {
                            println!("Can't read 'data/Brawl/fighter' directory.");
                            return None;
                        }
                    };
                    for fighter in fighters {
                        let lower_fighter_name = fighter.cased_name.to_lowercase();
                        if cli.fighter_names.len() == 0 || cli.fighter_names.iter().any(|x| x == &lower_fighter_name) {
                            let fighter = HighLevelFighter::new(&fighter);

                            let mut script_lookup = HashMap::new();

                            for action in &fighter.actions {
                                let name = action.script_entry.offset.to_string();
                                let address = format!("/{}/{}/actions/{}.html#script-entry", mod_name, fighter.name, name);
                                script_lookup.insert(action.script_entry.offset, ScriptInfo { name, address });

                                let name = action.script_exit.offset.to_string();
                                let address = format!("/{}/{}/actions/{}.html#script-exit", mod_name, fighter.name, name);
                                script_lookup.insert(action.script_exit.offset, ScriptInfo { name, address });
                            }

                            for subaction in &fighter.subactions {
                                let scripts = &subaction.scripts;

                                let name = format!("{} Main", subaction.name);
                                let address = format!("/{}/{}/subactions/{}.html#script-main", mod_name, fighter.name, subaction.name);
                                script_lookup.insert(scripts.script_main.offset, ScriptInfo { name, address });

                                let name = format!("{} GFX", subaction.name);
                                let address = format!("/{}/{}/subactions/{}.html#script-gfx", mod_name, fighter.name, subaction.name);
                                script_lookup.insert(scripts.script_gfx.offset, ScriptInfo { name, address });

                                let name = format!("{} SFX", subaction.name);
                                let address = format!("/{}/{}/subactions/{}.html#script-sfx", mod_name, fighter.name, subaction.name);
                                script_lookup.insert(scripts.script_sfx.offset, ScriptInfo { name, address });

                                let name = format!("{} Other", subaction.name);
                                let address = format!("/{}/{}/subactions/{}.html#script-other", mod_name, fighter.name, subaction.name);
                                script_lookup.insert(scripts.script_other.offset, ScriptInfo { name, address });
                            }

                            for script in &fighter.scripts_fragment_fighter {
                                let name = script.offset.to_string();
                                let address = format!("/{}/{}/scripts/{}.html", mod_name, fighter.name, name);
                                script_lookup.insert(script.offset, ScriptInfo { name, address });
                            }

                            for script in &fighter.scripts_fragment_common {
                                let name = script.offset.to_string();
                                let address = format!("/{}/{}/scripts/{}.html", mod_name, fighter.name, name);
                                script_lookup.insert(script.offset, ScriptInfo { name, address });
                                //assert!(script_lookup.insert(script.offset, ScriptInfo { name, address }).is_none());
                            }

                            brawl_fighters.push(BrawlFighter { fighter, script_lookup });
                        }

                    }
                }
                Err(_) => {
                    println!("Failed to open fighter directory");
                    return None;
                }
            };

            Some(BrawlMod {
                name:     mod_name,
                fighters: brawl_fighters,
            })
        } else {
            None
        }
    }

    pub fn gen_fighter_links(&self, current_fighter: &str) -> Vec<NavLink> {
        let mut links = vec!();
        for fighter in &self.fighters {
            links.push(NavLink {
                name:    fighter.fighter.name.clone(),
                link:    format!("/{}/{}", self.name, fighter.fighter.name),
                current: current_fighter == &fighter.fighter.name,
            });
        }
        links
    }

    pub fn gen_subaction_links(&self, fighter: &HighLevelFighter, current_subaction: String) -> Vec<NavLink> {
        let mut links = vec!();
        for subaction in &fighter.subactions {
            links.push(NavLink {
                name:    subaction.name.clone(),
                link:    format!("/{}/{}/subactions/{}.html", self.name, fighter.name, subaction.name),
                current: current_subaction == subaction.name,
            });
        }
        links
    }

    pub fn gen_script_fragment_common_links(&self, fighter: &HighLevelFighter, current_script: u32) -> Vec<NavLink> {
        let mut links = vec!();
        for script in &fighter.scripts_fragment_common {
            links.push(NavLink {
                name:    script.offset.to_string(),
                link:    format!("/{}/{}/scripts/{}.html", self.name, fighter.name, script.offset),
                current: current_script == script.offset,
            });
        }
        links
    }

    pub fn gen_script_fragment_fighter_links(&self, fighter: &HighLevelFighter, current_script: u32) -> Vec<NavLink> {
        let mut links = vec!();
        for script in &fighter.scripts_fragment_fighter {
            links.push(NavLink {
                name:    script.offset.to_string(),
                link:    format!("/{}/{}/scripts/{}.html", self.name, fighter.name, script.offset),
                current: current_script == script.offset,
            });
        }
        links
    }

    pub fn gen_action_links(&self, fighter: &HighLevelFighter, current_action: &str) -> Vec<NavLink> {
        let mut links = vec!();
        for action in &fighter.actions {
            links.push(NavLink {
                name:    action.name.clone(),
                link:    format!("/{}/{}/actions/{}.html", self.name, fighter.name, action.name),
                current: current_action == action.name,
            });
        }
        links
    }
}

use std::fs;
use std::fs::DirEntry;
use std::collections::HashMap;

use brawllib_rs::fighter::{Fighter, ModType};
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
    pub fighter:              HighLevelFighter,
    pub script_lookup:        HashMap<i32, ScriptInfo>,
    pub script_lookup_common: HashMap<i32, ScriptInfo>,
}

pub struct ScriptInfo {
    pub name:    String,
    pub address: String,
}

impl BrawlMods {
    pub fn new(cli: &CLIResults) -> Option<BrawlMods> {
        match fs::read_dir("../data") {
            Ok(dir) => {
                let mut mods: Vec<_> = dir.filter_map(|x| BrawlMod::new(x.unwrap(), &cli)).collect();
                mods.sort_by_key(|x| x.name.clone());
                Some(BrawlMods { mods })
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
                    let is_mod = lower_mod_name != "brawl";
                    let mod_fighter_dir = if is_mod {
                        Some(fighter_dir)
                    } else {
                        None
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

                        // Filter unmodified fighters from mods, so that deleted fighters from mods don't show up as brawl fighters
                        let unmodified_fighter_in_mod = match fighter.mod_type {
                            ModType::NotMod         => true,
                            ModType::ModFromBase    => false,
                            ModType::ModFromScratch => false,
                        } && is_mod;

                        if (cli.fighter_names.len() == 0 || cli.fighter_names.iter().any(|x| x == &lower_fighter_name)) && lower_fighter_name != "poketrainer" && !unmodified_fighter_in_mod {
                            let fighter = HighLevelFighter::new(&fighter);

                            let mut script_lookup = HashMap::new();
                            let mut script_lookup_common = HashMap::new();

                            for action in &fighter.actions {
                                if action.common {
                                    if action.script_entry.offset != 0 {
                                        let name = action.script_entry.offset.to_string();
                                        let address = format!("/{}/{}/actions/{}.html#script-entry", mod_name, fighter.name, name);
                                        // These sorts of scripts may be from the same offset, as multiple actions refer to the same script.
                                        // It shouldnt matter too much as the scripts are going to be identical anyway.
                                        script_lookup_common.insert(action.script_entry.offset, ScriptInfo { name, address });
                                    }

                                    if action.script_exit.offset != 0 {
                                        let name = action.script_exit.offset.to_string();
                                        let address = format!("/{}/{}/actions/{}.html#script-exit", mod_name, fighter.name, name);
                                        script_lookup_common.insert(action.script_exit.offset, ScriptInfo { name, address });
                                    }
                                }
                                else {
                                    if action.script_entry.offset != 0 {
                                        let name = action.script_entry.offset.to_string();
                                        let address = format!("/{}/{}/actions/{}.html#script-entry", mod_name, fighter.name, name);
                                        script_lookup.insert(action.script_entry.offset, ScriptInfo { name, address });
                                    }

                                    if action.script_exit.offset != 0 {
                                        let name = action.script_exit.offset.to_string();
                                        let address = format!("/{}/{}/actions/{}.html#script-exit", mod_name, fighter.name, name);
                                        script_lookup.insert(action.script_exit.offset, ScriptInfo { name, address });
                                    }
                                }
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
                                let name = format!("0x{:x}", script.offset);
                                let address = format!("/{}/{}/scripts/{}.html", mod_name, fighter.name, name);
                                // fragment scripts should not have duplicate offsets, they are
                                // guaranteed unique by the way they are generated.
                                assert!(script_lookup.insert(script.offset, ScriptInfo { name, address }).is_none());
                            }

                            for script in &fighter.scripts_fragment_common {
                                let name = format!("0x{:x}", script.offset);
                                let address = format!("/{}/{}/scripts_common/{}.html", mod_name, fighter.name, name);
                                assert!(script_lookup_common.insert(script.offset, ScriptInfo { name, address }).is_none());
                            }

                            brawl_fighters.push(BrawlFighter { fighter, script_lookup, script_lookup_common });
                        }

                    }
                }
                Err(_) => {
                    println!("Failed to open fighter directory");
                    return None;
                }
            };

            brawl_fighters.sort_by_key(|x| x.fighter.name.clone());

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

    pub fn gen_subaction_links(&self, fighter: &HighLevelFighter, current_subaction: String) -> SubactionLinks {
        let mut attacks_jab = vec!();
        let mut attacks_tilt = vec!();
        let mut attacks_smash = vec!();
        let mut attacks_dash = vec!();
        let mut attacks_aerial = vec!();
        let mut specials = vec!();
        let mut grabs = vec!();
        let mut ledge_options = vec!();
        let mut knockdowns = vec!();
        let mut trips = vec!();
        let mut dodges = vec!();
        let mut finals = vec!();
        let mut taunts = vec!();
        let mut stun = vec!();
        let mut sleep = vec!();
        let mut swim = vec!();
        let mut item = vec!();
        let mut item_throw = vec!();
        let mut movements = vec!();
        let mut crawl = vec!();
        let mut glide = vec!();
        let mut wall_ceiling_tech = vec!();
        let mut footstool = vec!();
        let mut misc = vec!();
        let mut none = vec!();

        for subaction in &fighter.subactions {
            let link = NavLink {
                name:    subaction.name.clone(),
                link:    format!("/{}/{}/subactions/{}.html", self.name, fighter.name, subaction.name),
                current: current_subaction == subaction.name,
            };

            // This heuristic is kind of dumb.
            // alernatively I could try and read which actions/subactions call each other to
            // determine where to add each subaction
            //
            // NOTE: Be careful that sometimes the check uses link.name.contains(..) and other times it uses link.name.starts_with(..)
            if link.name.contains("Cliff") {
                ledge_options.push(link);
            }
            else if link.name.contains("Item") || link.name.contains("Gekikara") {
                item.push(link);
            }
            else if link.name.contains("Ganon") || link.name.contains("Snake") || link.name.contains("Bitten") || link.name.contains("Stick") || link.name.contains("Rope") || link.name.contains("Ladder") || link.name.contains("Egg") || link.name.contains("Capture") || link.name.contains("Zitabata") || link.name.contains("Swing"){
                misc.push(link);
            }
            else if link.name.contains("FuraFura") {
                stun.push(link);
            }
            else if link.name.contains("FuraSleep") {
                sleep.push(link);
            }
            else if link.name.contains("Final") {
                finals.push(link);
            }
            else if link.name.contains("Swim") {
                swim.push(link);
            }
            else if link.name.contains("Slip") {
                trips.push(link);
            }
            else if link.name.contains("Glide") {
                glide.push(link);
            }
            else if link.name.contains("Shank") || link.name.contains("AttackSquat") || link.name.contains("SquatF") || link.name.contains("SquatB") {
                crawl.push(link);
            }
            else if link.name.contains("Down") {
                knockdowns.push(link);
            }
            else if link.name.contains("AirCatch") {
                misc.push(link);
            }
            else if link.name.contains("Catch") || link.name.starts_with("Throw") && !link.name.contains("Thrown") {
                grabs.push(link);
            }
            else if !link.name.starts_with("Throw") && link.name.contains("Throw") && !link.name.contains("Thrown") {
                item_throw.push(link);
            }
            else if link.name.contains("Step") {
                footstool.push(link);
            }
            else if link.name.contains("Fall") || link.name.contains("Landing") {
                movements.push(link);
            }
            else if link.name.contains("Special") {
                specials.push(link);
            }
            else if link.name.contains("AttackEnd") {
                misc.push(link);
            }
            else if link.name.contains("Attack") {
                let number: String = link.name.chars().filter(|x| x.is_digit(10)).collect();
                if link.name.contains("Air") {
                    attacks_aerial.push(link);
                }
                else if link.name.contains("Attack") && number.starts_with("1") {
                    attacks_jab.push(link);
                }
                else if link.name.contains("Attack") && number.starts_with("3") {
                    attacks_tilt.push(link);
                }
                else if link.name.contains("Attack") && number.starts_with("4") {
                    attacks_smash.push(link);
                }
                else if link.name.contains("Attack") {
                    attacks_dash.push(link);
                }
                else {
                    error!("Missed an attack for {} in the subaction navigation", fighter.name);
                }
            }
            else if link.name.contains("Appeal") || link.name.contains("Win") || link.name == "Lose" {
                taunts.push(link);
            }
            else if link.name.contains("Wait") || link.name.contains("Dash") || link.name.contains("Run") || link.name.contains("Turn") || link.name.contains("Walk") || link.name.contains("Jump") || link.name.contains("MissFoot") || link.name.contains("Ottotto") || link.name.contains("Squat") {
                movements.push(link);
            }
            else if link.name.contains("Stop") || link.name.contains("PassiveWallJump") {
                wall_ceiling_tech.push(link);
            }
            else if link.name.contains("Escape") || link.name.contains("Guard") {
                dodges.push(link);
            }
            else if link.name.contains("NONE") || link.name.starts_with("_") {
                none.push(link);
            }
            else {
                misc.push(link);
            }
        }
        attacks_jab.sort_by_key(|x| x.name.clone());
        attacks_tilt.sort_by_key(|x| x.name.clone());
        attacks_smash.sort_by_key(|x| x.name.clone());
        attacks_dash.sort_by_key(|x| x.name.clone());
        attacks_aerial.sort_by_key(|x| x.name.clone());
        specials.sort_by_key(|x| x.name.clone());
        grabs.sort_by_key(|x| x.name.clone());
        ledge_options.sort_by_key(|x| x.name.clone());
        knockdowns.sort_by_key(|x| x.name.clone());
        trips.sort_by_key(|x| x.name.clone());
        dodges.sort_by_key(|x| x.name.clone());
        taunts.sort_by_key(|x| x.name.clone());
        finals.sort_by_key(|x| x.name.clone());
        stun.sort_by_key(|x| x.name.clone());
        sleep.sort_by_key(|x| x.name.clone());
        swim.sort_by_key(|x| x.name.clone());
        wall_ceiling_tech.sort_by_key(|x| x.name.clone());
        footstool.sort_by_key(|x| x.name.clone());
        glide.sort_by_key(|x| x.name.clone());
        crawl.sort_by_key(|x| x.name.clone());
        movements.sort_by_key(|x| x.name.clone());
        item.sort_by_key(|x| x.name.clone());
        item_throw.sort_by_key(|x| x.name.clone());
        misc.sort_by_key(|x| x.name.clone());
        none.sort_by_key(|x| x.name.clone());

        let has_glide = glide.len() > 0;
        let has_crawl = crawl.len() > 0;

        SubactionLinks { attacks_aerial, attacks_jab, attacks_tilt, attacks_smash, attacks_dash, grabs, specials, knockdowns, trips, ledge_options, dodges, wall_ceiling_tech, glide, crawl, footstool, movements, finals, taunts, stun, sleep, swim, item, item_throw, none, misc, has_glide, has_crawl }
    }

    pub fn gen_script_fragment_common_links(&self, fighter: &HighLevelFighter, current_script: i32) -> Vec<NavLink> {
        let mut links = vec!();
        for script in &fighter.scripts_fragment_common {
            links.push(NavLink {
                name:    format!("0x{:x}", script.offset),
                link:    format!("/{}/{}/scripts_common/0x{:x}.html", self.name, fighter.name, script.offset),
                current: current_script == script.offset,
            });
        }
        links
    }

    pub fn gen_script_fragment_fighter_links(&self, fighter: &HighLevelFighter, current_script: i32) -> Vec<NavLink> {
        let mut links = vec!();
        for script in &fighter.scripts_fragment_fighter {
            links.push(NavLink {
                name:    format!("0x{:x}", script.offset),
                link:    format!("/{}/{}/scripts/0x{:x}.html", self.name, fighter.name, script.offset),
                current: current_script == script.offset,
            });
        }
        links
    }

    pub fn gen_script_section_links(&self, fighter: &HighLevelFighter, current_script: &str) -> Vec<NavLink> {
        let mut links = vec!();
        for script in &fighter.scripts_section {
            links.push(NavLink {
                name:    script.name.clone(),
                link:    format!("/{}/{}/scripts_common/{}.html", self.name, fighter.name, script.name),
                current: current_script == script.name,
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

#[derive(Serialize)]
pub struct SubactionLinks {
    pub attacks_jab:       Vec<NavLink>,
    pub attacks_tilt:      Vec<NavLink>,
    pub attacks_smash:     Vec<NavLink>,
    pub attacks_dash:      Vec<NavLink>,
    pub attacks_aerial:    Vec<NavLink>,
    pub grabs:             Vec<NavLink>,
    pub specials:          Vec<NavLink>,
    pub knockdowns:        Vec<NavLink>,
    pub trips:             Vec<NavLink>,
    pub ledge_options:     Vec<NavLink>,
    pub dodges:            Vec<NavLink>,
    pub wall_ceiling_tech: Vec<NavLink>,
    pub footstool:         Vec<NavLink>,
    pub glide:             Vec<NavLink>,
    pub crawl:             Vec<NavLink>,
    pub movements:         Vec<NavLink>,
    pub finals:            Vec<NavLink>,
    pub taunts:            Vec<NavLink>,
    pub stun:              Vec<NavLink>,
    pub sleep:             Vec<NavLink>,
    pub swim:              Vec<NavLink>,
    pub item:              Vec<NavLink>,
    pub item_throw:        Vec<NavLink>,
    pub misc:              Vec<NavLink>,
    pub none:              Vec<NavLink>,
    pub has_glide:         bool,
    pub has_crawl:         bool,
}

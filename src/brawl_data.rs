use brawllib_rs::high_level_fighter::HighLevelFighter;

use crate::page::NavLink;

pub struct BrawlMods {
    pub mods: Vec<BrawlMod>,
}

impl BrawlMods {
    pub fn gen_mod_links(&self, current_mod: String) -> Vec<NavLink> {
        let mut links = vec!();
        for brawl_mod in &self.mods { // TODO: Allow specify ordering either via config file or the order used in --mods NAME1,NAME2
            links.push(NavLink {
                name:    brawl_mod.name.clone(),
                link:    format!("/framedata/{}", brawl_mod.name),
                current: brawl_mod.name == current_mod,
            });
        }
        links
    }
}

pub struct BrawlMod {
    pub name:     String,
    pub fighters: Vec<HighLevelFighter>,
}

impl BrawlMod {
    pub fn gen_fighter_links(&self) -> Vec<NavLink> {
        let mut links = vec!();
        for fighter in &self.fighters {
            links.push(NavLink {
                name:    fighter.name.clone(),
                link:    format!("/framedata/{}/{}", self.name, fighter.name),
                current: false,
            });
        }
        links
    }

    pub fn gen_fighter_links_action(&self, current_fighter: String, current_action: String) -> Vec<NavLink> {
        let mut links = vec!();
        for fighter in &self.fighters {
            let action_name = if fighter.actions.iter().find(|x| x.name == current_action).is_some() {
                current_action.clone()
            } else {
                String::from("Wait1")
            };
            links.push(NavLink {
                name:    fighter.name.clone(),
                link:    format!("/framedata/{}/{}/{}.html", self.name, fighter.name, action_name),
                current: current_fighter == fighter.name,
            });
        }
        links
    }

    pub fn gen_action_links(&self, fighter: &HighLevelFighter, current_action: String) -> Vec<NavLink> {
        let mut links = vec!();
        for action in &fighter.actions {
            links.push(NavLink {
                name:    action.name.clone(),
                link:    format!("/framedata/{}/{}/{}.html", self.name, fighter.name, action.name),
                current: current_action == action.name,
            });
        }
        links
    }
}


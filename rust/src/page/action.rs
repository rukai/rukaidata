use std::fs::File;
use std::fs;

use handlebars::Handlebars;
use rayon::prelude::*;

use crate::brawl_data::BrawlMods;
use crate::page::NavLink;
use crate::process_scripts;

pub fn generate(handlebars: &Handlebars, brawl_mods: &BrawlMods) {
    for brawl_mod in &brawl_mods.mods {
        let mod_links = brawl_mods.gen_mod_links(brawl_mod.name.clone());

        for fighter in &brawl_mod.fighters {
            let mut fighter_links = vec!();
            for other_fighter in &brawl_mod.fighters {
                let other_name = &other_fighter.fighter.name;
                fighter_links.push(NavLink {
                    name:    other_name.clone(),
                    link:    format!("/{}/{}/actions", brawl_mod.name, fighter.fighter.name),
                    current: other_name == &fighter.fighter.name,
                });
            }
            fighter.fighter.actions.par_iter().for_each(|action| {
                let page = ActionPage {
                    mod_links:     &mod_links,
                    title:         format!("{} - {} - Action - {}", brawl_mod.name, fighter.fighter.name, action.name),
                    action_links:  brawl_mod.gen_action_links(&fighter.fighter, &action.name),
                    script_entry:  process_scripts::process_events(&action.script_entry.block.events, brawl_mod, &fighter),
                    script_exit:   process_scripts::process_events(&action.script_exit.block.events, brawl_mod, &fighter),
                    fighter_links: &fighter_links,
                };

                fs::create_dir_all(format!("../root/{}/{}/actions/", brawl_mod.name, fighter.fighter.name)).unwrap();
                let path = format!("../root/{}/{}/actions/{}.html",
                    brawl_mod.name, fighter.fighter.name, action.name);
                let file = File::create(path).unwrap();
                handlebars.render_to_write("action", &page, file).unwrap();
                info!("{} {} action {}", brawl_mod.name, fighter.fighter.name, action.name);
            });
        }
    }
}

#[derive(Serialize)]
pub struct ActionPage<'a> {
    mod_links:     &'a [NavLink],
    fighter_links: &'a [NavLink],
    action_links:  Vec<NavLink>,
    title:         String,
    script_entry:  String,
    script_exit:   String,
}

use std::fs::File;
use std::fs;

use handlebars::Handlebars;
use rayon::prelude::*;

use crate::brawl_data::BrawlMods;
use crate::page::NavLink;
use crate::process_scripts;
use crate::assets::AssetPaths;

pub fn generate(handlebars: &Handlebars, brawl_mods: &BrawlMods, assets: &AssetPaths) {
    for brawl_mod in &brawl_mods.mods {
        let mod_links = brawl_mods.gen_mod_links(brawl_mod.name.clone());

        for fighter in &brawl_mod.fighters {
            let mut fighter_links = vec!();
            for other_fighter in &brawl_mod.fighters {
                let other_name = &other_fighter.fighter.name;
                fighter_links.push(NavLink {
                    name:    other_name.clone(),
                    link:    format!("/{}/{}/actions", brawl_mod.name, other_fighter.fighter.name),
                    current: other_name == &fighter.fighter.name,
                });
            }
            fighter_links.push(NavLink {
                name:    "Fighter Common".into(),
                link:    format!("/{}/common/actions", brawl_mod.name),
                current: false,
            });
            fighter.fighter.actions.par_iter().for_each(|action| {
                let page = ActionPage {
                    assets,
                    mod_links:     &mod_links,
                    title:         format!("{} - {} - Action - {}", brawl_mod.name, fighter.fighter.name, action.name),
                    action_links:  brawl_mod.gen_action_links(fighter.fighter.name.clone(), &fighter.fighter.actions, &action.name),
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

        let mut fighter_links = vec!();
        for other_fighter in &brawl_mod.fighters {
            let other_name = &other_fighter.fighter.name;
            fighter_links.push(NavLink {
                name:    other_name.clone(),
                link:    format!("/{}/{}/actions", brawl_mod.name, other_fighter.fighter.name),
                current: other_name == "common",
            });
        }
        fighter_links.push(NavLink {
            name:    "Fighter Common".into(),
            link:    format!("/{}/common/actions", brawl_mod.name),
            current: true,
        });
        brawl_mod.fighter_common.actions.par_iter().for_each(|action| {
            let page = ActionPage {
                assets,
                mod_links:     &mod_links,
                title:         format!("{} - Common Fighter - Action - {}", brawl_mod.name, action.name),
                action_links:  brawl_mod.gen_action_links("common".into(), &brawl_mod.fighter_common.actions, &action.name),
                script_entry:  process_scripts::process_events_common(&action.script_entry.block.events, brawl_mod, &brawl_mod.script_lookup_common),
                script_exit:   process_scripts::process_events_common(&action.script_exit.block.events, brawl_mod, &brawl_mod.script_lookup_common),
                fighter_links: &fighter_links,
            };

            fs::create_dir_all(format!("../root/{}/common/actions/", brawl_mod.name)).unwrap();
            let path = format!("../root/{}/common/actions/{}.html", brawl_mod.name, action.name);
            let file = File::create(path).unwrap();
            handlebars.render_to_write("action", &page, file).unwrap();
            info!("{} Common Fighter action {}", brawl_mod.name, action.name);
        });
    }
}

#[derive(Serialize)]
pub struct ActionPage<'a> {
    assets:        &'a AssetPaths,
    mod_links:     &'a [NavLink],
    fighter_links: &'a [NavLink],
    action_links:  Vec<NavLink>,
    title:         String,
    script_entry:  String,
    script_exit:   String,
}

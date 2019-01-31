use std::fs::File;
use std::fs;

use handlebars::Handlebars;
use rayon::prelude::*;

use crate::brawl_data::BrawlMods;
use crate::page::NavLink;
use crate::assets::AssetPaths;

pub fn generate(handlebars: &Handlebars, brawl_mods: &BrawlMods, assets: &AssetPaths) {
    for brawl_mod in &brawl_mods.mods {
        let mod_links = brawl_mods.gen_mod_links(brawl_mod.name.clone());
        brawl_mod.fighters.par_iter().for_each(|fighter| {
            let fighter = &fighter.fighter;

            let mut fighter_links = vec!();
            for other_fighter in &brawl_mod.fighters {
                let other_name = &other_fighter.fighter.name;
                fighter_links.push(NavLink {
                    name:    other_name.clone(),
                    link:    format!("/{}/{}/actions", brawl_mod.name, other_fighter.fighter.name),
                    current: other_name == &fighter.name,
                });
            }

            let page = ActionsPage {
                mod_links:     &mod_links,
                title:         format!("{} - {} - Actions", brawl_mod.name, fighter.name),
                action_links:  brawl_mod.gen_action_links(fighter, ""),
                fighter_links,
                assets,
            };

            fs::create_dir_all(format!("../root/{}/{}/actions", brawl_mod.name, fighter.name)).unwrap();
            let path = format!("../root/{}/{}/actions/index.html", brawl_mod.name, fighter.name);
            let file = File::create(path).unwrap();
            handlebars.render_to_write("actions", &page, file).unwrap();
        });
    }
}

#[derive(Serialize)]
struct ActionsPage<'a> {
    assets:        &'a AssetPaths,
    mod_links:     &'a [NavLink],
    title:         String,
    fighter_links: Vec<NavLink>,
    action_links:  Vec<NavLink>,
}

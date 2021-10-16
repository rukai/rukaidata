use std::fs;
use std::fs::File;

use handlebars::Handlebars;
use rayon::prelude::*;

use crate::assets::AssetPaths;
use crate::brawl_data::{BrawlMods, SubactionLinks};
use crate::page::NavLink;

pub fn generate(handlebars: &Handlebars, brawl_mods: &BrawlMods, assets: &AssetPaths) {
    for brawl_mod in &brawl_mods.mods {
        let mod_links = brawl_mods.gen_mod_links(brawl_mod.name.clone());
        brawl_mod.fighters.par_iter().for_each(|fighter| {
            let fighter = &fighter.fighter;
            let page = SubactionsPage {
                mod_links: &mod_links,
                title: format!("{} - {} - Subactions", brawl_mod.name, fighter.name),
                fighter_links: brawl_mod.gen_fighter_links(&fighter.name),
                subaction_links: brawl_mod.gen_subaction_links(fighter, String::from("")),
                assets,
            };

            fs::create_dir_all(format!(
                "../root/{}/{}/subactions",
                brawl_mod.name, fighter.name
            ))
            .unwrap();
            let path = format!(
                "../root/{}/{}/subactions/index.html",
                brawl_mod.name, fighter.name
            );
            let file = File::create(path).unwrap();
            handlebars
                .render_to_write("subactions", &page, file)
                .unwrap();
        });
    }
}

#[derive(Serialize)]
struct SubactionsPage<'a> {
    assets: &'a AssetPaths,
    mod_links: &'a [NavLink],
    fighter_links: Vec<NavLink>,
    subaction_links: SubactionLinks,
    title: String,
}

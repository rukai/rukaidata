use std::fs;
use std::fs::File;

use handlebars::Handlebars;
use rayon::prelude::*;

use crate::assets::AssetPaths;
use crate::brawl_data::BrawlMods;
use crate::page::NavLink;

pub fn generate(handlebars: &Handlebars, brawl_mods: &BrawlMods, assets: &AssetPaths) {
    for brawl_mod in &brawl_mods.mods {
        let mod_links = brawl_mods.gen_mod_links(brawl_mod.name.clone());
        brawl_mod.fighters.par_iter().for_each(|fighter| {
            let fighter = &fighter.fighter;
            let page = ScriptsPage {
                mod_links: &mod_links,
                title: format!("{} - {} - Subroutines", brawl_mod.name, fighter.name),
                fighter_links: brawl_mod.gen_fighter_links(&fighter.name),
                script_fragment_fighter_links: brawl_mod
                    .gen_script_fragment_fighter_links(fighter, 0),
                script_fragment_common_links: brawl_mod
                    .gen_script_fragment_common_links(fighter, 0),
                script_section_links: brawl_mod.gen_script_section_links(fighter, ""),
                assets,
            };

            fs::create_dir_all(format!(
                "../root/{}/{}/scripts",
                brawl_mod.name, fighter.name
            ))
            .unwrap();
            let path = format!(
                "../root/{}/{}/scripts/index.html",
                brawl_mod.name, fighter.name
            );
            let file = File::create(path).unwrap();
            handlebars.render_to_write("scripts", &page, file).unwrap();
        });
    }
}

#[derive(Serialize)]
struct ScriptsPage<'a> {
    assets: &'a AssetPaths,
    mod_links: &'a [NavLink],
    title: String,
    fighter_links: Vec<NavLink>,
    script_fragment_fighter_links: Vec<NavLink>,
    script_fragment_common_links: Vec<NavLink>,
    script_section_links: Vec<NavLink>,
}

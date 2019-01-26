use std::fs::File;
use std::fs;

use handlebars::Handlebars;
use rayon::prelude::*;

use crate::brawl_data::BrawlMods;
use crate::page::NavLink;

pub fn generate(handlebars: &Handlebars, brawl_mods: &BrawlMods) {
    for brawl_mod in &brawl_mods.mods {
        let mod_links = brawl_mods.gen_mod_links(brawl_mod.name.clone());
        brawl_mod.fighters.par_iter().for_each(|fighter| {
            let page = ScriptsPage {
                mod_links:     &mod_links,
                title:         format!("{} - {} - Scripts", brawl_mod.name, fighter.name),
                fighter_links: brawl_mod.gen_fighter_links(),
                script_links:  brawl_mod.gen_script_links(fighter, String::from("")),
            };

            fs::create_dir_all(format!("../root/{}/{}", brawl_mod.name, fighter.name)).unwrap();
            let path = format!("../root/{}/{}/scripts.html", brawl_mod.name, fighter.name);
            let file = File::create(path).unwrap();
            handlebars.render_to_write("scripts", &page, file).unwrap();
        });
    }
}

#[derive(Serialize)]
struct ScriptsPage<'a> {
    mod_links:     &'a [NavLink],
    title:         String,
    fighter_links: Vec<NavLink>,
    script_links:  Vec<NavLink>,
}

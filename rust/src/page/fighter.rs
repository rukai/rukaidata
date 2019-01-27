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
            let fighter = &fighter.fighter;
            let page = FighterPage {
                mod_links:     &mod_links,
                title:         format!("{} - {}", brawl_mod.name, fighter.name),
                fighter_links: brawl_mod.gen_fighter_links(&fighter.name),
            };

            fs::create_dir_all(format!("../root/{}/{}", brawl_mod.name, fighter.name)).unwrap();
            let path = format!("../root/{}/{}/index.html", brawl_mod.name, fighter.name);
            let file = File::create(path).unwrap();
            handlebars.render_to_write("fighter", &page, file).unwrap();
        });
    }
}

#[derive(Serialize)]
struct FighterPage<'a> {
    mod_links:     &'a [NavLink],
    fighter_links: Vec<NavLink>,
    title:         String,
}

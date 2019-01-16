use std::fs::File;
use std::fs;

use handlebars::Handlebars;

use crate::brawl_data::BrawlMods;
use crate::page::NavLink;

pub fn generate(handlebars: &Handlebars, brawl_mods: &BrawlMods) {
    for brawl_mod in &brawl_mods.mods {
        let page = ModPage {
            mod_links:     brawl_mods.gen_mod_links(brawl_mod.name.clone()),
            title:         format!("{} Fighters", brawl_mod.name),
            fighter_links: brawl_mod.gen_fighter_links(),
        };

        fs::create_dir_all(format!("npm-webpack/dist/framedata/{}", brawl_mod.name)).unwrap();
        let path = format!("npm-webpack/dist/framedata/{}/index.html", brawl_mod.name);
        let file = File::create(path).unwrap();
        handlebars.render_to_write("mod", &page, file).unwrap();
    }
}

#[derive(Serialize)]
struct ModPage {
    mod_links:     Vec<NavLink>,
    fighter_links: Vec<NavLink>,
    title:         String,
}

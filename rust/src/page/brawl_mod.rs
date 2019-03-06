use std::fs::File;
use std::fs;

use handlebars::Handlebars;

use crate::brawl_data::BrawlMods;
use crate::page::NavLink;
use crate::assets::AssetPaths;

pub fn generate(handlebars: &Handlebars, brawl_mods: &BrawlMods, assets: &AssetPaths) {
    for brawl_mod in &brawl_mods.mods {
        let mut fighter_links = vec!();
        for fighter in &brawl_mod.fighters {
            fighter_links.push(NavLink {
                name:    fighter.fighter.name.clone(),
                link:    format!("/{}/{}", brawl_mod.name, fighter.fighter.name),
                current: false,
            });
        }
        fighter_links.push(NavLink {
            name:    "Common Fighter".into(),
            link:    format!("/{}/common", brawl_mod.name),
            current: false,
        });

        let page = ModPage {
            mod_links:     brawl_mods.gen_mod_links(brawl_mod.name.clone()),
            title:         format!("{} Fighters", brawl_mod.name),
            fighter_links,
            assets,
        };

        fs::create_dir_all(format!("../root/{}", brawl_mod.name)).unwrap();
        let path = format!("../root/{}/index.html", brawl_mod.name);
        let file = File::create(path).unwrap();
        handlebars.render_to_write("mod", &page, file).unwrap();
    }
}

#[derive(Serialize)]
struct ModPage<'a> {
    assets:        &'a AssetPaths,
    mod_links:     Vec<NavLink>,
    fighter_links: Vec<NavLink>,
    title:         String,
}

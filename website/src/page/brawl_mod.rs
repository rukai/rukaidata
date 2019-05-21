use std::fs::File;
use std::path::Path;
use std::fs;

use handlebars::Handlebars;

use crate::brawl_data::BrawlMods;
use crate::page::NavLink;
use crate::assets::AssetPaths;

pub fn generate(handlebars: &Handlebars, brawl_mods: &BrawlMods, assets: &AssetPaths) {
    for brawl_mod in &brawl_mods.mods {
        fs::create_dir_all(format!("../root/{}", brawl_mod.name)).unwrap();
        let index_path = format!("../root/{}/index.html", brawl_mod.name);

        if brawl_mod.dummy {
            // If the page exists, do not write ANYTHING.
            // Dummy mods are used to retain the pages for an already generated mod.
            // However we still want to create a placeholder page when the mod is new so the user doesnt get linked to a 404.
            //
            // One case we cant handle is updating a placeholder page with a new placeholder page.
            // But this is unlikely to be needed so not worth worrying about.
            if !Path::new(&index_path).exists() {
                let page = ModPage {
                    mod_links:     brawl_mods.gen_mod_links(brawl_mod.name.clone()),
                    title:         format!("{} Coming Soon!", brawl_mod.name),
                    fighter_links: vec!(),
                    assets,
                };
                let file = File::create(index_path).unwrap();
                handlebars.render_to_write("mod", &page, file).unwrap();
            }
        }
        else {
            let mut fighter_links = vec!();
            for fighter in &brawl_mod.fighters {
                fighter_links.push(NavLink {
                    name:    fighter.fighter.name.clone(),
                    link:    format!("/{}/{}", brawl_mod.name, fighter.fighter.name),
                    current: false,
                });
            }

            let page = ModPage {
                mod_links:     brawl_mods.gen_mod_links(brawl_mod.name.clone()),
                title:         format!("{} Fighters", brawl_mod.name),
                fighter_links,
                assets,
            };

            let file = File::create(index_path).unwrap();
            handlebars.render_to_write("mod", &page, file).unwrap();
        }
    }
}

#[derive(Serialize)]
struct ModPage<'a> {
    assets:        &'a AssetPaths,
    mod_links:     Vec<NavLink>,
    fighter_links: Vec<NavLink>,
    title:         String,
}

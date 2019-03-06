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
                    link:    format!("/{}/{}/scripts", brawl_mod.name, other_name),
                    current: other_name == &fighter.fighter.name,
                });
            }
            fighter_links.push(NavLink {
                name:    "Fighter Common".into(),
                link:    format!("/{}/common/scripts", brawl_mod.name),
                current: false,
            });

            fighter.fighter.scripts_fragment_fighter.par_iter().for_each(|script| {
                let page = ScriptPage {
                    mod_links:            &mod_links,
                    title:                format!("{} - {} - Subroutine - {}", brawl_mod.name, fighter.fighter.name, script.offset),
                    script_fighter_links: brawl_mod.gen_script_fragment_fighter_links(&fighter.fighter, script.offset),
                    script_common_links:  brawl_mod.gen_script_fragment_common_links(&brawl_mod.fighter_common.scripts_fragment, script.offset),
                    script:               process_scripts::process_events(&script.block.events, brawl_mod, &fighter),
                    fighter_links:        &fighter_links,
                    assets,
                };

                fs::create_dir_all(format!("../root/{}/{}/scripts", brawl_mod.name, fighter.fighter.name)).unwrap();
                let path = format!("../root/{}/{}/scripts/{}.html",
                    brawl_mod.name, fighter.fighter.name, script.offset);
                let file = File::create(path).unwrap();
                handlebars.render_to_write("script", &page, file).unwrap();
                info!("{} {} {}", brawl_mod.name, fighter.fighter.name, script.offset);
            });
        }

        let mut fighter_links = vec!();
        for other_fighter in &brawl_mod.fighters {
            let other_name = &other_fighter.fighter.name;
            fighter_links.push(NavLink {
                name:    other_name.clone(),
                link:    format!("/{}/{}/scripts", brawl_mod.name, other_name),
                current: false,
            });
        }
        fighter_links.push(NavLink {
            name:    "Fighter Common".into(),
            link:    format!("/{}/common/scripts", brawl_mod.name),
            current: true,
        });

        brawl_mod.fighter_common.scripts_fragment.par_iter().for_each(|script| {
            let page = ScriptPage {
                mod_links:            &mod_links,
                title:                format!("{} - Common Fighter - Subroutine {}", brawl_mod.name, script.offset),
                script_fighter_links: vec!(),
                script_common_links:  brawl_mod.gen_script_fragment_common_links(&brawl_mod.fighter_common.scripts_fragment, script.offset),
                script:               process_scripts::process_events_common(&script.block.events, brawl_mod, &brawl_mod.script_lookup_common),
                fighter_links:        &fighter_links,
                assets,
            };

            fs::create_dir_all(format!("../root/{}/common/scripts", brawl_mod.name)).unwrap();
            let path = format!("../root/{}/common/scripts/{}.html",
                brawl_mod.name, script.offset);
            let file = File::create(path).unwrap();
            handlebars.render_to_write("script", &page, file).unwrap();
            info!("{} Common Fighter {}", brawl_mod.name, script.offset);
        });
    }
}

#[derive(Serialize)]
pub struct ScriptPage<'a> {
    assets:               &'a AssetPaths,
    mod_links:            &'a [NavLink],
    fighter_links:        &'a [NavLink],
    script_common_links:  Vec<NavLink>,
    script_fighter_links: Vec<NavLink>,
    title:                String,
    script:               String,
}

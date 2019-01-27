use std::fs::File;
use std::fs;

use handlebars::Handlebars;
use rayon::prelude::*;

use crate::brawl_data::BrawlMods;
use crate::page::NavLink;
use crate::process_scripts;

pub fn generate(handlebars: &Handlebars, brawl_mods: &BrawlMods) {
    for brawl_mod in &brawl_mods.mods {
        let mod_links = brawl_mods.gen_mod_links(brawl_mod.name.clone());

        for fighter in &brawl_mod.fighters {
            let mut fighter_links = vec!();
            for other_fighter in &brawl_mod.fighters {
                let other_name = &other_fighter.fighter.name;
                fighter_links.push(NavLink {
                    name:    other_name.clone(),
                    link:    format!("/{}/{}/scripts", brawl_mod.name, fighter.fighter.name),
                    current: other_name == &fighter.fighter.name,
                });
            }

            fighter.fighter.scripts_fragment_fighter.par_iter().for_each(|script| {
                let page = ScriptPage {
                    mod_links:            &mod_links,
                    title:                format!("{} - {} - Subroutine - {}", brawl_mod.name, fighter.fighter.name, script.offset),
                    script_fighter_links: brawl_mod.gen_script_fragment_fighter_links(&fighter.fighter, script.offset),
                    script_common_links:  brawl_mod.gen_script_fragment_common_links(&fighter.fighter, script.offset),
                    script:               process_scripts::process_events(&script.block.events, brawl_mod, &fighter),
                    fighter_links:        &fighter_links,
                };

                fs::create_dir_all(format!("../root/{}/{}/scripts", brawl_mod.name, fighter.fighter.name)).unwrap();
                let path = format!("../root/{}/{}/scripts/{}.html",
                    brawl_mod.name, fighter.fighter.name, script.offset);
                let file = File::create(path).unwrap();
                handlebars.render_to_write("script", &page, file).unwrap();
                info!("{} {} {}", brawl_mod.name, fighter.fighter.name, script.offset);
            });

            fighter.fighter.scripts_fragment_common.par_iter().for_each(|script| {
                let page = ScriptPage {
                    mod_links:            &mod_links,
                    title:                format!("{} - {} - Common Subroutine {}", brawl_mod.name, fighter.fighter.name, script.offset),
                    script_fighter_links: brawl_mod.gen_script_fragment_fighter_links(&fighter.fighter, script.offset),
                    script_common_links:  brawl_mod.gen_script_fragment_common_links(&fighter.fighter, script.offset),
                    script:               process_scripts::process_events(&script.block.events, brawl_mod, &fighter),
                    fighter_links:        &fighter_links,
                };

                fs::create_dir_all(format!("../root/{}/{}/scripts", brawl_mod.name, fighter.fighter.name)).unwrap();
                let path = format!("../root/{}/{}/scripts/{}.html",
                    brawl_mod.name, fighter.fighter.name, script.offset);
                let file = File::create(path).unwrap();
                handlebars.render_to_write("script", &page, file).unwrap();
                info!("{} {} {}", brawl_mod.name, fighter.fighter.name, script.offset);
            });
        }
    }
}

#[derive(Serialize)]
pub struct ScriptPage<'a> {
    mod_links:            &'a [NavLink],
    fighter_links:        &'a [NavLink],
    script_common_links:  Vec<NavLink>,
    script_fighter_links: Vec<NavLink>,
    title:                String,
    script:               String,
}

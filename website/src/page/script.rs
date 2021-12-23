use std::fs;
use std::fs::File;

use handlebars::Handlebars;
use rayon::prelude::*;

use crate::assets::AssetPaths;
use crate::brawl_data::BrawlMods;
use crate::page::NavLink;
use crate::process_scripts;

pub fn generate(handlebars: &Handlebars, brawl_mods: &BrawlMods, assets: &AssetPaths) {
    for brawl_mod in &brawl_mods.mods {
        let mod_links = brawl_mods.gen_mod_links(brawl_mod.name.clone());

        for fighter in &brawl_mod.fighters {
            let mut fighter_links = vec![];
            for other_fighter in &brawl_mod.fighters {
                let other_name = &other_fighter.fighter.name;
                fighter_links.push(NavLink {
                    name: other_name.clone(),
                    link: format!("/{}/{}/scripts", brawl_mod.name, fighter.fighter.name),
                    current: other_name == &fighter.fighter.name,
                });
            }

            fighter
                .fighter
                .scripts_fragment_fighter
                .par_iter()
                .for_each(|script| {
                    let page = ScriptPage {
                        mod_links: &mod_links,
                        title: format!(
                            "{} - {} - Subroutine - 0x{:x}",
                            brawl_mod.name, fighter.fighter.name, script.offset
                        ),
                        script_fighter_links: brawl_mod
                            .gen_script_fragment_fighter_links(&fighter.fighter, script.offset),
                        script_common_links: brawl_mod
                            .gen_script_fragment_common_links(&fighter.fighter, 0),
                        script_section_links: brawl_mod
                            .gen_script_section_links(&fighter.fighter, ""),
                        script: process_scripts::process_events(
                            &script.block.events,
                            false,
                            brawl_mod,
                            fighter,
                        ),
                        fighter_links: &fighter_links,
                        assets,
                    };

                    fs::create_dir_all(format!(
                        "../root/{}/{}/scripts",
                        brawl_mod.name, fighter.fighter.name
                    ))
                    .unwrap();
                    let path = format!(
                        "../root/{}/{}/scripts/0x{:x}.html",
                        brawl_mod.name, fighter.fighter.name, script.offset
                    );
                    let file = File::create(path).unwrap();
                    handlebars.render_to_write("script", &page, file).unwrap();
                    info!(
                        "{} {} 0x{:x}",
                        brawl_mod.name, fighter.fighter.name, script.offset
                    );
                });

            fighter
                .fighter
                .scripts_fragment_common
                .par_iter()
                .for_each(|script| {
                    let page = ScriptPage {
                        mod_links: &mod_links,
                        title: format!(
                            "{} - {} - Common Subroutine 0x{:x}",
                            brawl_mod.name, fighter.fighter.name, script.offset
                        ),
                        script_fighter_links: brawl_mod
                            .gen_script_fragment_fighter_links(&fighter.fighter, 0),
                        script_common_links: brawl_mod
                            .gen_script_fragment_common_links(&fighter.fighter, script.offset),
                        script_section_links: brawl_mod
                            .gen_script_section_links(&fighter.fighter, ""),
                        script: process_scripts::process_events(
                            &script.block.events,
                            true,
                            brawl_mod,
                            fighter,
                        ),
                        fighter_links: &fighter_links,
                        assets,
                    };

                    fs::create_dir_all(format!(
                        "../root/{}/{}/scripts_common",
                        brawl_mod.name, fighter.fighter.name
                    ))
                    .unwrap();
                    let path = format!(
                        "../root/{}/{}/scripts_common/0x{:x}.html",
                        brawl_mod.name, fighter.fighter.name, script.offset
                    );
                    let file = File::create(path).unwrap();
                    handlebars.render_to_write("script", &page, file).unwrap();
                    info!(
                        "{} {} 0x{:x}",
                        brawl_mod.name, fighter.fighter.name, script.offset
                    );
                });

            fighter
                .fighter
                .scripts_section
                .par_iter()
                .for_each(|script| {
                    let page = ScriptPage {
                        mod_links: &mod_links,
                        title: format!(
                            "{} - {} - Common Section {}",
                            brawl_mod.name, fighter.fighter.name, script.name
                        ),
                        script_fighter_links: brawl_mod
                            .gen_script_fragment_fighter_links(&fighter.fighter, 0),
                        script_common_links: brawl_mod
                            .gen_script_fragment_common_links(&fighter.fighter, 0),
                        script_section_links: brawl_mod
                            .gen_script_section_links(&fighter.fighter, &script.name),
                        script: process_scripts::process_events(
                            &script.script.block.events,
                            true,
                            brawl_mod,
                            fighter,
                        ),
                        fighter_links: &fighter_links,
                        assets,
                    };

                    fs::create_dir_all(format!(
                        "../root/{}/{}/scripts_common",
                        brawl_mod.name, fighter.fighter.name
                    ))
                    .unwrap();
                    let path = format!(
                        "../root/{}/{}/scripts_common/{}.html",
                        brawl_mod.name, fighter.fighter.name, script.name
                    );
                    let file = File::create(path).unwrap();
                    handlebars.render_to_write("script", &page, file).unwrap();
                    info!(
                        "{} {} {}",
                        brawl_mod.name, fighter.fighter.name, script.name
                    );
                });
        }
    }
}

#[derive(Serialize)]
pub struct ScriptPage<'a> {
    assets: &'a AssetPaths,
    mod_links: &'a [NavLink],
    fighter_links: &'a [NavLink],
    script_fighter_links: Vec<NavLink>,
    script_common_links: Vec<NavLink>,
    script_section_links: Vec<NavLink>,
    title: String,
    script: String,
}

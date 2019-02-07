use std::fs::File;
use std::fs;

use handlebars::Handlebars;
use rayon::prelude::*;

use crate::brawl_data::{BrawlMods, SubactionLinks};
use crate::page::NavLink;
use crate::process_scripts;
use crate::assets::AssetPaths;

pub fn generate(handlebars: &Handlebars, brawl_mods: &BrawlMods, assets: &AssetPaths) {
    for brawl_mod in &brawl_mods.mods {
        let mod_links = brawl_mods.gen_mod_links(brawl_mod.name.clone());

        for fighter in &brawl_mod.fighters {
            fighter.fighter.subactions.par_iter().for_each(|subaction| {
                let fighter_name = &fighter.fighter.name;
                // Originally tried to handle scripts as a table of frame,main,gfx,sfx,other but
                // that would require simulating the scripts and with what inputs???
                // How to handle infinite loops???
                //
                // It would be nice to simulate the whole script displaying the current line in browser but that would need wasm and stuff.
                // Im not rewriting ScriptRunner in javascript!!!
                //
                // So instead I just dump the scripts one by one, linking to other pages for external function calls
                // Then one day I can come and add script running via wasm.
                let script_main  = process_scripts::process_events(&subaction.scripts.script_main.block.events, brawl_mod, fighter);
                let script_gfx   = process_scripts::process_events(&subaction.scripts.script_gfx.block.events, brawl_mod, fighter);
                let script_sfx   = process_scripts::process_events(&subaction.scripts.script_sfx.block.events, brawl_mod, fighter);
                let script_other = process_scripts::process_events(&subaction.scripts.script_other.block.events, brawl_mod, fighter);

                let mut frame_buttons = vec!();
                for (mut index, frame) in subaction.frames.iter().enumerate() {
                    index += 1;
                    let class = if !frame.hit_boxes.is_empty() {
                        String::from("hitbox-frame-button")
                    } else if index > subaction.iasa {
                        String::from("iasa-frame-button")
                    } else {
                        String::from("standard-frame-button")
                    };
                    frame_buttons.push(FrameButton { index, class });
                }

                // Despite being a twitter description, this is designed for use on discord over twitter.
                // We make use of the 78 lines that discord displays.
                // Ignoring that twitter only displays the first 3 lines.
                let mut twitter_description = String::new();
                twitter_description.push_str(&format!("IASA: {}", subaction.iasa));
                twitter_description.push_str(&format!("\nFrames: {}", subaction.frames.len()));
                // TODO: Add a landing_lag: Option<u32> field to HighLevelAction, it should check if the name of the subaction is AttackAirN etc. Then grab the appropriate landing lag from the attributes
                //twitter_description.push_str(&format!("\nLanding Lag: {}", 0));
                twitter_description.push_str("\n\nCurrent gif is a placeholder:");

                // generate fighter links
                let mut fighter_links = vec!();
                for nav_fighter in &brawl_mod.fighters {
                    let nav_fighter = &nav_fighter.fighter;
                    let subaction_name = if nav_fighter.subactions.iter().find(|x| x.name == subaction.name).is_some() {
                        subaction.name.clone()
                    } else {
                        String::from("Wait1")
                    };
                    fighter_links.push(NavLink {
                        name:    nav_fighter.name.clone(),
                        link:    format!("/{}/{}/subactions/{}.html", brawl_mod.name, nav_fighter.name, subaction_name),
                        current: fighter_name == &nav_fighter.name,
                    });
                }

                let mut subaction_extent = subaction.hurt_box_extent();
                subaction_extent.extend(&subaction.hit_box_extent());

                let page = SubactionPage {
                    assets,
                    fighter_link:     format!("/{}/{}", brawl_mod.name, fighter_name),
                    mod_links:        &mod_links,
                    title:            format!("{} - {} - Subaction - {}", brawl_mod.name, fighter_name, subaction.name),
                    subaction_links:  brawl_mod.gen_subaction_links(&fighter.fighter, subaction.name.clone()),
                    subaction:        serde_json::to_string(&subaction).unwrap(),
                    subaction_extent: serde_json::to_string(&subaction_extent).unwrap(),
                    fighter_links,
                    script_main,
                    script_gfx,
                    script_sfx,
                    script_other,
                    frame_buttons,
                    twitter_image: String::from("/assets_static/meta-banner.gif"),
                    twitter_description,
                };

                fs::create_dir_all(format!("../root/{}/{}/subactions", brawl_mod.name, fighter_name)).unwrap();
                let path = format!("../root/{}/{}/subactions/{}.html", brawl_mod.name, fighter_name, subaction.name);
                let file = File::create(path).unwrap();
                handlebars.render_to_write("subaction", &page, file).unwrap();
                info!("{} {} {}", brawl_mod.name, fighter_name, subaction.name);
            });
        }
    }
}

#[derive(Serialize)]
pub struct SubactionPage<'a> {
    assets:              &'a AssetPaths,
    mod_links:           &'a [NavLink],
    fighter_links:       Vec<NavLink>,
    subaction_links:     SubactionLinks,
    fighter_link:        String,
    title:               String,
    subaction:           String,
    subaction_extent:    String,
    script_main:         String,
    script_gfx:          String,
    script_sfx:          String,
    script_other:        String,
    frame_buttons:       Vec<FrameButton>,
    twitter_description: String,
    twitter_image:       String,
}

#[derive(Serialize)]
pub struct FrameButton {
    index: usize,
    class: String,
}

use std::fs::File;
use std::fs;

use handlebars::Handlebars;
use rayon::prelude::*;

use crate::brawl_data::BrawlMods;
use crate::page::NavLink;

pub fn generate(handlebars: &Handlebars, brawl_mods: &BrawlMods) {
    for brawl_mod in &brawl_mods.mods {
        let mod_links = brawl_mods.gen_mod_links(brawl_mod.name.clone());
        for fighter in &brawl_mod.fighters {
            fighter.actions.par_iter().for_each(|action| {
                let mut frame_buttons = vec!();
                for (index, frame) in action.frames.iter().enumerate() {
                    let class = if !frame.hit_boxes.is_empty() {
                        String::from("hitbox-frame-button")
                    } else if index > action.iasa {
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
                twitter_description.push_str(&format!("IASA: {}", action.iasa));
                twitter_description.push_str(&format!("\nFrames: {}", action.frames.len()));
                // TODO: Add a landing_lag: Option<u32> field to HighLevelAction, it should check if the name of the action is AttackAirN etc. Then grab the appropriate landing lag from the attributes
                //twitter_description.push_str(&format!("\nLanding Lag: {}", 0));
                twitter_description.push_str("Current gif is a placeholder:");

                let page = ActionPage {
                    fighter_link:  format!("/{}/{}", brawl_mod.name, fighter.name),
                    mod_links:     &mod_links,
                    title:         format!("{} - {} - {}", brawl_mod.name, fighter.name, action.name),
                    fighter_links: brawl_mod.gen_fighter_links_action(fighter.name.clone(), action.name.clone()),
                    action_links:  brawl_mod.gen_action_links(fighter, action.name.clone()),
                    action:        serde_json::to_string(&action).unwrap(),
                    frame_buttons,
                    twitter_image: String::from("/assets_static/meta-banner.gif"),
                    twitter_description,
                };

                fs::create_dir_all(format!("../root/{}/{}", brawl_mod.name, fighter.name)).unwrap();
                let path = format!("../root/{}/{}/{}.html",
                    brawl_mod.name, fighter.name, action.name);
                let file = File::create(path).unwrap();
                handlebars.render_to_write("action", &page, file).unwrap();
                info!("{} {} {}", brawl_mod.name, fighter.name, action.name);
            });
        }
    }
}

#[derive(Serialize)]
pub struct ActionPage<'a> {
    mod_links:           &'a [NavLink],
    fighter_links:       Vec<NavLink>,
    action_links:        Vec<NavLink>,
    fighter_link:        String,
    title:               String,
    action:              String,
    frame_buttons:       Vec<FrameButton>,
    twitter_description: String,
    twitter_image:       String,
}

#[derive(Serialize)]
pub struct FrameButton {
    index: usize,
    class: String,
}

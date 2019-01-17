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

                let page = ActionPage {
                    fighter_link:  format!("/{}/{}", brawl_mod.name, fighter.name),
                    mod_links:     &mod_links,
                    title:         format!("{} - {} - {}", brawl_mod.name, fighter.name, action.name),
                    fighter_links: brawl_mod.gen_fighter_links_action(fighter.name.clone(), action.name.clone()),
                    action_links:  brawl_mod.gen_action_links(fighter, action.name.clone()),
                    action:        serde_json::to_string(&action).unwrap(),
                    frame_buttons,
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
    mod_links:     &'a [NavLink],
    fighter_links: Vec<NavLink>,
    action_links:  Vec<NavLink>,
    fighter_link:  String,
    title:         String,
    action:        String,
    frame_buttons: Vec<FrameButton>,
}

#[derive(Serialize)]
pub struct FrameButton {
    index: usize,
    class: String,
}


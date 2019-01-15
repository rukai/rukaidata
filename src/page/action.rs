use rocket_contrib::templates::Template;
use rocket::State;

use crate::brawl_data::BrawlMods;
use crate::page::error::ErrorPage;
use crate::page::NavLink;

#[get("/framedata/<mod_name>/<fighter_name>/<action_name>")]
pub fn serve(brawl_mods: State<BrawlMods>, mod_name: String, fighter_name: String, action_name: String) -> Template {
    let mod_links = brawl_mods.gen_mod_links(mod_name.clone());
    if let Some(brawl_mod) = brawl_mods.mods.iter().find(|x| x.name == mod_name) {
        if let Some(fighter) = brawl_mod.fighters.iter().find(|x| x.name == fighter_name) {
            if let Some(action) = fighter.actions.iter().find(|x| x.name == action_name) {
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
                    fighter_link:  format!("/framedata/{}/{}", mod_name, fighter_name),
                    mod_links,
                    title:         format!("{} - {} - {}", mod_name, fighter_name, action_name),
                    fighter_links: brawl_mod.gen_fighter_links_action(fighter_name, action_name.clone()),
                    action_links:  brawl_mod.gen_action_links(fighter, action_name),
                    action:        serde_json::to_string(&action).unwrap(),
                    frame_buttons,
                };
                Template::render("action", page)
            } else {
                let error = format!("The action {} does not exist in fighter {} in mod {}.", action_name, fighter_name, mod_name);
                Template::render("error", ErrorPage { mod_links, error })
            }
        } else {
            let error = format!("The Fighter {} does not exist in mod {}.", fighter_name, mod_name);
            Template::render("error", ErrorPage { mod_links, error })
        }
    } else {
        let error = format!("The mod {} does not exist.", mod_name);
        Template::render("error", ErrorPage { mod_links, error })
    }
}

#[derive(Serialize)]
pub struct ActionPage {
    mod_links:     Vec<NavLink>,
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


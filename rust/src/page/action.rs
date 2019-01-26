use std::fs::File;
use std::fs;

use handlebars::Handlebars;
use rayon::prelude::*;
use brawllib_rs::script_ast::{EventAst, IfStatement, Expression, UnaryExpression, BinaryExpression};

use crate::brawl_data::BrawlMods;
use crate::page::NavLink;

pub fn generate(handlebars: &Handlebars, brawl_mods: &BrawlMods) {
    for brawl_mod in &brawl_mods.mods {
        let mod_links = brawl_mods.gen_mod_links(brawl_mod.name.clone());

        for fighter in &brawl_mod.fighters {
            fighter.actions.par_iter().for_each(|action| {
                // Originally tried to handle scripts as a table of frame,main,gfx,sfx,other but
                // that would require simulating the scripts and with what inputs???
                // How to handle infinite loops???
                //
                // It would be nice to simulate the whole script displaying the current line in browser but that would need wasm and stuff.
                // Im not rewriting ScriptRunner in javascript!!!
                //
                // So instead I just dump the scripts one by one, linking to other pages for external function calls
                // Then one day I can come and add script running via wasm.
                let mut script_main  = String::new();
                let mut script_gfx   = String::new();
                let mut script_sfx   = String::new();
                let mut script_other = String::new();
                if let Some(action_scripts) = &action.scripts {
                    script_main  = process_events(&action_scripts.script_main.block.events);
                    script_gfx   = process_events(&action_scripts.script_gfx.block.events);
                    script_sfx   = process_events(&action_scripts.script_sfx.block.events);
                    script_other = process_events(&action_scripts.script_other.block.events);

                    // TODO: Filter out empty ScriptFrame
                }

                let mut frame_buttons = vec!();
                for (mut index, frame) in action.frames.iter().enumerate() {
                    index += 1;
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
                twitter_description.push_str("\n\nCurrent gif is a placeholder:");

                let page = ActionPage {
                    fighter_link:  format!("/{}/{}", brawl_mod.name, fighter.name),
                    mod_links:     &mod_links,
                    title:         format!("{} - {} - {}", brawl_mod.name, fighter.name, action.name),
                    fighter_links: brawl_mod.gen_fighter_links_action(fighter.name.clone(), action.name.clone()),
                    action_links:  brawl_mod.gen_action_links(fighter, action.name.clone()),
                    action:        serde_json::to_string(&action).unwrap(),
                    script_main,
                    script_gfx,
                    script_sfx,
                    script_other,
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

fn process_events(events: &[EventAst]) -> String {
    let mut result = String::from("<ol>");
    for event in events {
        match event {
            EventAst::Nop => { }
            EventAst::ChangeAction { action, test } => {
                result.push_str(&format!("<li>ChangeAction {{ action: {}, when: ({}) }}</li>", action, process_expression(test)));
            }
            EventAst::IfStatement ( IfStatement { test, then_branch, else_branch } ) => {
                result.push_str(&format!("<li>if ({})", process_expression(test)));
                result.push_str(&process_events(&then_branch.events));
                result.push_str("</li>");

                if let Some(else_branch) = else_branch {
                    result.push_str("<li>else");
                    result.push_str(&process_events(&else_branch.events));
                    result.push_str("</li>");
                }
            }
            EventAst::Unknown (event) => result.push_str(&format!("<li>Unknown event {:x?}</li>", event)),
            _ => result.push_str(&format!("<li>{:?}</li>", event)),
        }
    }
    result.push_str("</ol>");
    result
}

fn process_expression(expr: &Expression) -> String {
    match expr {
        Expression::Nullary (requirement) => format!("{:?}", requirement),
        Expression::Unary (UnaryExpression { requirement, value })
            => format!("{:?} {}", requirement, process_expression(value)),
        Expression::Binary (BinaryExpression { left, operator, right })
            => format!("{} {:?} {}", process_expression(left), operator, process_expression(right)),
        Expression::Not (expr) => format!("not({})", process_expression(expr)),
        Expression::Variable (variable) => format!("variable(0x{:x})", variable),
        Expression::Value (value) => format!("value({})", value),
        Expression::Scalar (scalar) => format!("scalar({})", scalar),
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

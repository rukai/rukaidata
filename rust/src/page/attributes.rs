use std::fs::File;
use std::fs;

use handlebars::Handlebars;
use rayon::prelude::*;

use brawllib_rs::sakurai::fighter_data::FighterAttributes;

use crate::brawl_data::BrawlMods;
use crate::page::NavLink;

pub fn generate(handlebars: &Handlebars, brawl_mods: &BrawlMods) {
    for brawl_mod in &brawl_mods.mods {
        let mod_links = brawl_mods.gen_mod_links(brawl_mod.name.clone());
        brawl_mod.fighters.par_iter().for_each(|fighter| {
            let fighter = &fighter.fighter;
            let page = AttributesPage {
                mod_links:     &mod_links,
                title:         format!("{} - {} - Attributes", brawl_mod.name, fighter.name),
                fighter_links: brawl_mod.gen_fighter_links(&fighter.name),
                attributes:    attributes_to_strings(&fighter.attributes),
            };

            fs::create_dir_all(format!("../root/{}/{}", brawl_mod.name, fighter.name)).unwrap();
            let path = format!("../root/{}/{}/attributes.html", brawl_mod.name, fighter.name);
            let file = File::create(path).unwrap();
            handlebars.render_to_write("attributes", &page, file).unwrap();
        });
    }
}

fn attributes_to_strings(attributes: &FighterAttributes) -> Vec<Attribute> {
    let mut result = vec!();
    result.push(Attribute { name: "walk init vel", value: attributes.walk_init_vel.to_string() });
    result.push(Attribute { name: "walk acc", value: attributes.walk_acc.to_string() });
    result.push(Attribute { name: "walk max vel", value: attributes.walk_max_vel.to_string() });
    result.push(Attribute { name: "ground friction", value: attributes.ground_friction.to_string() });
    result.push(Attribute { name: "dash init vel", value: attributes.dash_init_vel.to_string() });
    result.push(Attribute { name: "dash run acc a", value: attributes.dash_run_acc_a.to_string() });
    result.push(Attribute { name: "dash run acc b", value: attributes.dash_run_acc_b.to_string() });
    result.push(Attribute { name: "dash run term vel", value: attributes.dash_run_term_vel.to_string() });
    result.push(Attribute { name: "grounded max x vel", value: attributes.grounded_max_x_vel.to_string() });
    result.push(Attribute { name: "dash cancel frame window", value: attributes.dash_cancel_frame_window.to_string() });
    result.push(Attribute { name: "guard on max momentum", value: attributes.guard_on_max_momentum.to_string() });
    result.push(Attribute { name: "jump squat frames", value: attributes.jump_squat_frames.to_string() });
    result.push(Attribute { name: "jump x init vel", value: attributes.jump_x_init_vel.to_string() });
    result.push(Attribute { name: "jump y init vel", value: attributes.jump_y_init_vel.to_string() });
    result.push(Attribute { name: "jump x vel ground mult", value: attributes.jump_x_vel_ground_mult.to_string() });
    result.push(Attribute { name: "jump x init term vel", value: attributes.jump_x_init_term_vel.to_string() });
    result.push(Attribute { name: "jump y init vel short", value: attributes.jump_y_init_vel_short.to_string() });
    result.push(Attribute { name: "air jump x mult", value: attributes.air_jump_x_mult.to_string() });
    result.push(Attribute { name: "air jump y mult", value: attributes.air_jump_y_mult.to_string() });
    result.push(Attribute { name: "footstool init vel", value: attributes.footstool_init_vel.to_string() });
    result.push(Attribute { name: "footstool init vel short", value: attributes.footstool_init_vel_short.to_string() });
    result.push(Attribute { name: "meteor cancel delay", value: attributes.meteor_cancel_delay.to_string() });
    result.push(Attribute { name: "num jumps", value: attributes.num_jumps.to_string() });
    result.push(Attribute { name: "gravity", value: attributes.gravity.to_string() });
    result.push(Attribute { name: "term vel", value: attributes.term_vel.to_string() });
    result.push(Attribute { name: "air friction y", value: attributes.air_friction_y.to_string() });
    result.push(Attribute { name: "air y term vel", value: attributes.air_y_term_vel.to_string() });
    result.push(Attribute { name: "air mobility a", value: attributes.air_mobility_a.to_string() });
    result.push(Attribute { name: "air mobility b", value: attributes.air_mobility_b.to_string() });
    result.push(Attribute { name: "air x term vel", value: attributes.air_x_term_vel.to_string() });
    result.push(Attribute { name: "air friction x", value: attributes.air_friction_x.to_string() });
    result.push(Attribute { name: "fastfall velocity", value: attributes.fastfall_velocity.to_string() });
    result.push(Attribute { name: "air x term vel hard", value: attributes.air_x_term_vel_hard.to_string() });
    result.push(Attribute { name: "glide frame window", value: attributes.glide_frame_window.to_string() });
    result.push(Attribute { name: "jab2 window", value: attributes.jab2_window.to_string() });
    result.push(Attribute { name: "jab3 window", value: attributes.jab3_window.to_string() });
    result.push(Attribute { name: "ftilt2 window", value: attributes.ftilt2_window.to_string() });
    result.push(Attribute { name: "ftilt3 window", value: attributes.ftilt3_window.to_string() });
    result.push(Attribute { name: "fsmash2 window", value: attributes.fsmash2_window.to_string() });
    result.push(Attribute { name: "flip dir frame", value: attributes.flip_dir_frame.to_string() });
    result.push(Attribute { name: "weight", value: attributes.weight.to_string() });
    result.push(Attribute { name: "size", value: attributes.size.to_string() });
    result.push(Attribute { name: "results screen size", value: attributes.results_screen_size.to_string() });
    result.push(Attribute { name: "shield size", value: attributes.shield_size.to_string() });
    result.push(Attribute { name: "shield break vel", value: attributes.shield_break_vel.to_string() });
    result.push(Attribute { name: "shield strength", value: attributes.shield_strength.to_string() });
    result.push(Attribute { name: "respawn platform size", value: attributes.respawn_platform_size.to_string() });
    result.push(Attribute { name: "edge jump x vel", value: attributes.edge_jump_x_vel.to_string() });
    result.push(Attribute { name: "edge jump y vel", value: attributes.edge_jump_y_vel.to_string() });
    result.push(Attribute { name: "item throw strength", value: attributes.item_throw_strength.to_string() });
    result.push(Attribute { name: "projectile item move speed", value: attributes.projectile_item_move_speed.to_string() });
    result.push(Attribute { name: "projectile item move speed dash f", value: attributes.projectile_item_move_speed_dash_f.to_string() });
    result.push(Attribute { name: "projectile item move speed dash b", value: attributes.projectile_item_move_speed_dash_b.to_string() });
    result.push(Attribute { name: "light landing lag", value: attributes.light_landing_lag.to_string() });
    result.push(Attribute { name: "normal landing lag", value: attributes.normal_landing_lag.to_string() });
    result.push(Attribute { name: "nair landing lag", value: attributes.nair_landing_lag.to_string() });
    result.push(Attribute { name: "fair landing lag", value: attributes.fair_landing_lag.to_string() });
    result.push(Attribute { name: "bair landing lag", value: attributes.bair_landing_lag.to_string() });
    result.push(Attribute { name: "uair landing lag", value: attributes.uair_landing_lag.to_string() });
    result.push(Attribute { name: "dair landing lag", value: attributes.dair_landing_lag.to_string() });
    result.push(Attribute { name: "term vel hard frames", value: attributes.term_vel_hard_frames.to_string() });
    result.push(Attribute { name: "hip n bone", value: attributes.hip_n_bone.to_string() });
    result.push(Attribute { name: "tag height value", value: attributes.tag_height_value.to_string() });
    result.push(Attribute { name: "walljump x vel", value: attributes.walljump_x_vel.to_string() });
    result.push(Attribute { name: "walljump y vel", value: attributes.walljump_y_vel.to_string() });
    result.push(Attribute { name: "lhand n bone", value: attributes.lhand_n_bone.to_string() });
    result.push(Attribute { name: "rhand n bone", value: attributes.rhand_n_bone.to_string() });
    result.push(Attribute { name: "water y acc", value: attributes.water_y_acc.to_string() });
    result.push(Attribute { name: "spit star size", value: attributes.spit_star_size.to_string() });
    result.push(Attribute { name: "spit star damage", value: attributes.spit_star_damage.to_string() });
    result.push(Attribute { name: "egg size", value: attributes.egg_size.to_string() });
    result.push(Attribute { name: "hip n bone2", value: attributes.hip_n_bone2.to_string() });
    result.push(Attribute { name: "x rot n bone", value: attributes.x_rot_n_bone.to_string() });
    result
}

#[derive(Serialize)]
struct AttributesPage<'a> {
    mod_links:     &'a [NavLink],
    fighter_links: Vec<NavLink>,
    attributes:    Vec<Attribute>,
    title:         String,
}

#[derive(Serialize)]
struct Attribute {
    name: &'static str,
    value: String,
}

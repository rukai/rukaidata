use brawllib_rs::sakurai::FighterAttributes;

use rocket_contrib::templates::Template;
use rocket::State;

use crate::brawl_data::BrawlMods;
use crate::page::error::ErrorPage;
use crate::page::NavLink;

#[get("/framedata/<mod_name>/<fighter_name>")]
pub fn serve(brawl_mods: State<BrawlMods>, mod_name: String, fighter_name: String) -> Template {
    let mod_links = brawl_mods.gen_mod_links(mod_name.clone());
    if let Some(brawl_mod) = brawl_mods.mods.iter().find(|x| x.name == mod_name) {
        if let Some(fighter) = brawl_mod.fighters.iter().find(|x| x.name == fighter_name) {
            let page = FighterPage {
                mod_links,
                title:         format!("{} - {}", mod_name, fighter_name),
                fighter_links: brawl_mod.gen_fighter_links(),
                action_links:  brawl_mod.gen_action_links(fighter, String::from("")),
                attributes:    attributes_to_strings(&fighter.attributes),
            };
            Template::render("fighter", page)
        } else {
            let error = format!("The Fighter {} does not exist in mod {}.", fighter_name, mod_name);
            Template::render("error", ErrorPage { mod_links, error })
        }
    } else {
        let error = format!("The mod {} does not exist.", mod_name);
        Template::render("error", ErrorPage { mod_links, error })
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
struct FighterPage {
    mod_links:     Vec<NavLink>,
    fighter_links: Vec<NavLink>,
    action_links:  Vec<NavLink>,
    attributes:    Vec<Attribute>,
    title:         String,
}

#[derive(Serialize)]
struct Attribute {
    name: &'static str,
    value: String,
}

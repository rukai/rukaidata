use std::fs::File;
use std::fs;
use std::io::Write;

use brawllib_rs::renderer;

use crate::brawl_data::BrawlMods;

pub fn generate(brawl_mods: &BrawlMods) {
    let mut state = renderer::WgpuState::new();
    for brawl_mod in &brawl_mods.mods {
        for fighter in &brawl_mod.fighters {
            let fighter_name = &fighter.fighter.name;
            fs::create_dir_all(format!("../root/{}/{}/subactions", brawl_mod.name, fighter_name)).unwrap();

            for (index, subaction) in fighter.fighter.subactions.iter().enumerate() {
                if subaction.frames.len() > 0 {
                    let twitter_image = format!("/{}/{}/subactions/{}.gif", brawl_mod.name, fighter_name, subaction.name);

                    let gif = renderer::render_gif(&mut state, &fighter.fighter, index);
                    let path = format!("../root{}", twitter_image);
                    let mut file = File::create(path).unwrap();
                    file.write_all(&gif).unwrap();

                    info!("{} {} {} GIF", brawl_mod.name, fighter_name, subaction.name);
                }
            }
        }
    }
}

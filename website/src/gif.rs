use std::fs::File;
use std::fs;
use std::io::Write;
use std::sync::mpsc::Receiver;

use brawllib_rs::renderer;
use brawllib_rs::renderer::WgpuState;

use crate::brawl_data::BrawlMods;

struct GifWait {
    path: String,
    rx: Receiver<Vec<u8>>,
}

impl GifWait {
    fn wait(self) {
        let bytes = self.rx.recv().unwrap();
        let mut file = File::create(self.path).unwrap();
        file.write_all(&bytes).unwrap();
    }
}

pub fn generate(brawl_mods: &BrawlMods) {
    let mut state = futures::executor::block_on(WgpuState::new_for_gif());
    let mut gif_waits = vec!();
    for brawl_mod in &brawl_mods.mods {
        for fighter in &brawl_mod.fighters {
            let fighter_name = &fighter.fighter.name;
            fs::create_dir_all(format!("../root/{}/{}/subactions", brawl_mod.name, fighter_name)).unwrap();

            for (index, subaction) in fighter.fighter.subactions.iter().enumerate() {
                if subaction.frames.len() > 0 {
                    let twitter_image = format!("/{}/{}/subactions/{}.gif", brawl_mod.name, fighter_name, subaction.name);
                    let path = format!("../root{}", twitter_image);

                    let rx = futures::executor::block_on(renderer::render_gif(&mut state, &fighter.fighter, index));
                    gif_waits.push(GifWait {
                        path,
                        rx,
                    });

                    info!("{} {} {} GIF started", brawl_mod.name, fighter_name, subaction.name);

                    if gif_waits.len() >= num_cpus::get() {
                        gif_waits.remove(0).wait();
                    }
                }
            }
        }
    }

    for gif_wait in gif_waits {
        gif_wait.wait();
    }
}

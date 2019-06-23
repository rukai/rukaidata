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
    fn wait(self, state: &WgpuState) {
        state.poll();
        loop {
            match self.rx.try_recv() {
                Err(_) => {
                    state.poll();
                }
                Ok(bytes) => {
                    let mut file = File::create(self.path).unwrap();
                    file.write_all(&bytes).unwrap();
                    break;
                }
            }
        }
    }
}

pub fn generate(brawl_mods: &BrawlMods) {
    let mut state = WgpuState::new();
    let mut gif_waits = vec!();
    for brawl_mod in &brawl_mods.mods {
        for fighter in &brawl_mod.fighters {
            let fighter_name = &fighter.fighter.name;
            fs::create_dir_all(format!("../root/{}/{}/subactions", brawl_mod.name, fighter_name)).unwrap();

            for (index, subaction) in fighter.fighter.subactions.iter().enumerate() {
                if subaction.frames.len() > 0 {
                    let twitter_image = format!("/{}/{}/subactions/{}.gif", brawl_mod.name, fighter_name, subaction.name);
                    let path = format!("../root{}", twitter_image);

                    let rx = renderer::render_gif(&mut state, &fighter.fighter, index);
                    gif_waits.push(GifWait {
                        path,
                        rx,
                    });

                    info!("{} {} {} GIF started", brawl_mod.name, fighter_name, subaction.name);

                    if gif_waits.len() >= 8 {
                        gif_waits.remove(0).wait(&state);
                    }
                }
            }
        }
    }

    for gif_wait in gif_waits {
        gif_wait.wait(&state);
    }
}

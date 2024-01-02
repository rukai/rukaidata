use crate::brawl_data::BrawlMods;
use crate::output::OutDir;
use brawllib_rs::renderer;
use brawllib_rs::renderer::WgpuState;
use std::sync::mpsc::Receiver;

struct GifWait {
    dir: OutDir,
    file_name: String,
    rx: Receiver<Vec<u8>>,
}

impl GifWait {
    fn wait(self) {
        self.dir
            .create_compressed_file(&self.file_name, &self.rx.recv().unwrap());
    }
}

pub fn generate(brawl_mods: &BrawlMods) {
    let mut state = futures::executor::block_on(WgpuState::new_for_gif());
    let mut gif_waits = vec![];
    for brawl_mod in &brawl_mods.mods {
        for fighter in &brawl_mod.fighters {
            let fighter_name = &fighter.fighter.name;
            let dir = OutDir::new(&format!("{}/{}/subactions", brawl_mod.name, fighter_name));

            for (index, subaction) in fighter.fighter.subactions.iter().enumerate() {
                if !subaction.frames.is_empty() {
                    let rx = renderer::render_gif(&mut state, &fighter.fighter, index);
                    gif_waits.push(GifWait {
                        file_name: format!("{}.gif", subaction.name),
                        rx,
                        dir: dir.clone(),
                    });

                    info!(
                        "{} {} {} GIF started",
                        brawl_mod.name, fighter_name, subaction.name
                    );

                    if gif_waits.len()
                        >= std::thread::available_parallelism()
                            .map(|x| x.into())
                            .unwrap_or(1)
                    {
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

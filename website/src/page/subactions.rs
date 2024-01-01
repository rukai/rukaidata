use crate::assets::AssetPaths;
use crate::brawl_data::{BrawlMods, SubactionLinks};
use crate::output::OutDir;
use crate::page::NavLink;
use handlebars::Handlebars;
use rayon::prelude::*;

pub fn generate(handlebars: &Handlebars, brawl_mods: &BrawlMods, assets: &AssetPaths) {
    for brawl_mod in &brawl_mods.mods {
        let mod_links = brawl_mods.gen_mod_links(brawl_mod.name.clone());
        brawl_mod.fighters.par_iter().for_each(|fighter| {
            let fighter = &fighter.fighter;
            let page = SubactionsPage {
                mod_links: &mod_links,
                title: format!("{} - {} - Subactions", brawl_mod.name, fighter.name),
                fighter_links: brawl_mod.gen_fighter_links(&fighter.name),
                subaction_links: brawl_mod.gen_subaction_links(fighter, String::from("")),
                assets,
            };

            let file = OutDir::new(&format!("{}/{}/subactions", brawl_mod.name, fighter.name))
                .compressed_file_writer("index.html");
            handlebars
                .render_to_write("subactions", &page, file)
                .unwrap();
        });
    }
}

#[derive(Serialize)]
struct SubactionsPage<'a> {
    assets: &'a AssetPaths,
    mod_links: &'a [NavLink],
    fighter_links: Vec<NavLink>,
    subaction_links: SubactionLinks,
    title: String,
}

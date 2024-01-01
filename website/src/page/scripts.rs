use crate::assets::AssetPaths;
use crate::brawl_data::BrawlMods;
use crate::output::OutDir;
use crate::page::NavLink;
use handlebars::Handlebars;
use rayon::prelude::*;

pub fn generate(handlebars: &Handlebars, brawl_mods: &BrawlMods, assets: &AssetPaths) {
    for brawl_mod in &brawl_mods.mods {
        let mod_links = brawl_mods.gen_mod_links(brawl_mod.name.clone());
        brawl_mod.fighters.par_iter().for_each(|fighter| {
            let fighter = &fighter.fighter;
            let page = ScriptsPage {
                mod_links: &mod_links,
                title: format!("{} - {} - Subroutines", brawl_mod.name, fighter.name),
                fighter_links: brawl_mod.gen_fighter_links(&fighter.name),
                script_fragment_fighter_links: brawl_mod
                    .gen_script_fragment_fighter_links(fighter, 0),
                script_fragment_common_links: brawl_mod
                    .gen_script_fragment_common_links(fighter, 0),
                script_section_links: brawl_mod.gen_script_section_links(fighter, ""),
                assets,
            };

            let file = OutDir::new(&format!("{}/{}/scripts", brawl_mod.name, fighter.name))
                .compressed_file_writer("index.html");
            handlebars.render_to_write("scripts", &page, file).unwrap();
        });
    }
}

#[derive(Serialize)]
struct ScriptsPage<'a> {
    assets: &'a AssetPaths,
    mod_links: &'a [NavLink],
    title: String,
    fighter_links: Vec<NavLink>,
    script_fragment_fighter_links: Vec<NavLink>,
    script_fragment_common_links: Vec<NavLink>,
    script_section_links: Vec<NavLink>,
}

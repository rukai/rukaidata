use crate::assets::AssetPaths;
use crate::brawl_data::BrawlMods;
use crate::output::OutDir;
use crate::page::NavLink;
use handlebars::Handlebars;

pub fn generate(handlebars: &Handlebars, brawl_mods: &BrawlMods, assets: &AssetPaths) {
    for brawl_mod in &brawl_mods.mods {
        let mut fighter_links = vec![];
        for fighter in &brawl_mod.fighters {
            fighter_links.push(NavLink {
                name: fighter.fighter.name.clone(),
                link: format!("/{}/{}", brawl_mod.name, fighter.fighter.name),
                current: false,
            });
        }

        let page = ModPage {
            mod_links: brawl_mods.gen_mod_links(brawl_mod.name.clone()),
            title: format!("{} Fighters", brawl_mod.name),
            fighter_links,
            assets,
        };

        let file = OutDir::new(&brawl_mod.name).compressed_file_writer("index.html");
        handlebars.render_to_write("mod", &page, file).unwrap();
    }
}

#[derive(Serialize)]
struct ModPage<'a> {
    assets: &'a AssetPaths,
    mod_links: Vec<NavLink>,
    fighter_links: Vec<NavLink>,
    title: String,
}

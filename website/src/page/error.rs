use crate::assets::AssetPaths;
use crate::brawl_data::BrawlMods;
use crate::output::OutDir;
use crate::page::NavLink;
use handlebars::Handlebars;

pub fn generate(handlebars: &Handlebars, brawl_mods: &BrawlMods, assets: &AssetPaths) {
    let page = ErrorPage {
        assets,
        mod_links: brawl_mods.gen_mod_links(String::new()),
    };
    let file = OutDir::new(".").compressed_file_writer("error.html");
    handlebars.render_to_write("error", &page, file).unwrap();
}

#[derive(Serialize)]
pub struct ErrorPage<'a> {
    assets: &'a AssetPaths,
    pub mod_links: Vec<NavLink>,
}

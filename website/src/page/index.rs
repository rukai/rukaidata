use std::fs::File;

use handlebars::Handlebars;

use crate::assets::AssetPaths;
use crate::brawl_data::BrawlMods;
use crate::page::NavLink;

pub fn generate(handlebars: &Handlebars, brawl_mods: &BrawlMods, assets: &AssetPaths) {
    let page = IndexPage {
        title: "Rukai Data",
        mod_links: brawl_mods.gen_mod_links(String::new()),
        assets,
    };
    let file = File::create("../root/index.html").unwrap();
    handlebars.render_to_write("index", &page, file).unwrap();
}

#[derive(Serialize)]
struct IndexPage<'a> {
    assets: &'a AssetPaths,
    mod_links: Vec<NavLink>,
    title: &'static str,
}

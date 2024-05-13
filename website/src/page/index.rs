use crate::assets::AssetPaths;
use crate::brawl_data::BrawlMods;
use crate::output::OutDir;
use crate::page::NavLink;
use handlebars::Handlebars;

pub fn generate(handlebars: &Handlebars, brawl_mods: &BrawlMods, assets: &AssetPaths) {
    let page = IndexPage {
        title: "Rukai Data",
        mod_links: brawl_mods.gen_mod_links(String::new()),
        assets,
    };
    let writer =
        OutDir::new(assets.root_index.trim_start_matches('/')).compressed_file_writer("index.html");
    handlebars.render_to_write("index", &page, writer).unwrap();
}

#[derive(Serialize)]
struct IndexPage<'a> {
    assets: &'a AssetPaths,
    mod_links: Vec<NavLink>,
    title: &'static str,
}

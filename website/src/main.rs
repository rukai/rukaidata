#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

use handlebars::Handlebars;

pub mod assets;
pub mod brawl_data;
pub mod cli;
pub mod gif;
pub mod logger;
pub mod page;
pub mod process_scripts;

use assets::AssetPaths;
use brawl_data::BrawlMods;

fn main() {
    logger::init();
    if let Some(cli) = cli::parse_cli() {
        if let Some(brawl_mods) = BrawlMods::new(&cli) {
            info!("brawl files loaded");

            if cli.generate_web {
                let mut handlebars = Handlebars::new();
                handlebars
                    .register_templates_directory(".html.hbs", "templates")
                    .unwrap();
                info!("handlebars templates loaded");

                let assets = AssetPaths::new(&cli);
                page::index::generate(&handlebars, &brawl_mods, &assets);
                page::error::generate(&handlebars, &brawl_mods, &assets);
                page::brawl_mod::generate(&handlebars, &brawl_mods, &assets);
                page::fighter::generate(&handlebars, &brawl_mods, &assets);
                page::attributes::generate(&handlebars, &brawl_mods, &assets);
                page::actions::generate(&handlebars, &brawl_mods, &assets);
                page::action::generate(&handlebars, &brawl_mods, &assets);
                page::subactions::generate(&handlebars, &brawl_mods, &assets);
                page::subaction::generate(&handlebars, &brawl_mods, &assets, cli.wasm_mode);
                page::script::generate(&handlebars, &brawl_mods, &assets);
                page::scripts::generate(&handlebars, &brawl_mods, &assets);
                page::variables::generate(&handlebars, &brawl_mods, &assets);
            }

            if cli.generate_gifs {
                gif::generate(&brawl_mods);
            }
        }
    }
}

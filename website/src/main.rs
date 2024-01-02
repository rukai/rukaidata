// This lint is stupid.
// I need an import and an unwrap to use `write!`, make the API more ergnomic before forcing it on me.
#![allow(clippy::format_push_string)]

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
pub mod output;
pub mod page;
pub mod process_scripts;
mod serve;

use assets::AssetPaths;
use brawl_data::BrawlMods;

fn main() {
    logger::init();
    let args = cli::args();

    if let Some(brawl_mods) = BrawlMods::new(&args) {
        info!("brawl files loaded");

        if args.generate_web {
            let mut handlebars = Handlebars::new();
            handlebars
                .register_templates_directory(".html.hbs", "templates")
                .unwrap();
            info!("handlebars templates loaded");

            let assets = AssetPaths::new(&args);
            page::index::generate(&handlebars, &brawl_mods, &assets);
            page::error::generate(&handlebars, &brawl_mods, &assets);
            page::brawl_mod::generate(&handlebars, &brawl_mods, &assets);
            page::fighter::generate(&handlebars, &brawl_mods, &assets);
            page::attributes::generate(&handlebars, &brawl_mods, &assets);
            page::actions::generate(&handlebars, &brawl_mods, &assets);
            page::action::generate(&handlebars, &brawl_mods, &assets);
            page::subactions::generate(&handlebars, &brawl_mods, &assets);
            page::subaction::generate(&handlebars, &brawl_mods, &assets, args.legacy_renderer);
            page::script::generate(&handlebars, &brawl_mods, &assets);
            page::scripts::generate(&handlebars, &brawl_mods, &assets);
            page::variables::generate(&handlebars, &brawl_mods, &assets);
        }

        if args.generate_gifs {
            gif::generate(&brawl_mods);
        }

        if args.serve {
            serve::serve();
        }
    }
}

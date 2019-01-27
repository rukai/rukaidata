#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;

use handlebars::Handlebars;

pub mod cli;
pub mod logger;
pub mod page;
pub mod brawl_data;
pub mod process_scripts;

use brawl_data::BrawlMods;

fn main() {
    logger::init();
    if let Some(cli) = cli::parse_cli() {
        if let Some(brawl_mods) = BrawlMods::new(&cli) {
            info!("brawl files loaded");

            let mut handlebars = Handlebars::new();
            handlebars.register_templates_directory(".html.hbs", "templates").unwrap();
            info!("handlebars templates loaded");

            page::index::generate(&handlebars, &brawl_mods);
            page::brawl_mod::generate(&handlebars, &brawl_mods);
            page::fighter::generate(&handlebars, &brawl_mods);
            page::attributes::generate(&handlebars, &brawl_mods);
            page::actions::generate(&handlebars, &brawl_mods);
            page::action::generate(&handlebars, &brawl_mods);
            page::subactions::generate(&handlebars, &brawl_mods);
            page::subaction::generate(&handlebars, &brawl_mods);
            page::script::generate(&handlebars, &brawl_mods);
            page::scripts::generate(&handlebars, &brawl_mods);
            page::variables::generate(&handlebars, &brawl_mods);
        }
    }
}

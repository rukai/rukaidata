use rocket_contrib::templates::Template;
use rocket::State;

use crate::brawl_data::BrawlMods;
use crate::page::NavLink;

#[get("/")]
pub fn serve(brawl_mods: State<BrawlMods>) -> Template {
    let page = IndexPage {
        title:     "Brawl Mod Frame Data",
        mod_links: brawl_mods.gen_mod_links(String::new()),
    };
    Template::render("index", page)
}

#[derive(Serialize)]
struct IndexPage {
    mod_links:     Vec<NavLink>,
    title:         &'static str,
}

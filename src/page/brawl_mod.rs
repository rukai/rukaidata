use rocket_contrib::templates::Template;
use rocket::State;

use crate::brawl_data::BrawlMods;
use crate::page::error::ErrorPage;
use crate::page::NavLink;

#[get("/framedata/<mod_name>")]
pub fn serve(brawl_mods: State<BrawlMods>, mod_name: String) -> Template {
    let mod_links = brawl_mods.gen_mod_links(mod_name.clone());
    if let Some(brawl_mod) = brawl_mods.mods.iter().find(|x| x.name == mod_name) {
        let page = ModPage {
            mod_links,
            title:         format!("{} Fighters", mod_name),
            fighter_links: brawl_mod.gen_fighter_links(),
        };
        Template::render("mod", page)
    } else {
        let error = format!("The mod {} does not exist.", mod_name);
        Template::render("error", ErrorPage { mod_links, error })
    }
}

#[derive(Serialize)]
struct ModPage {
    mod_links:     Vec<NavLink>,
    fighter_links: Vec<NavLink>,
    title:         String,
}

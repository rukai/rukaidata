use crate::page::NavLink;
use crate::assets::AssetPaths;

#[derive(Serialize)]
pub struct ErrorPage<'a> {
    assets:        &'a AssetPaths,
    pub mod_links: Vec<NavLink>,
    pub error:     String,
}

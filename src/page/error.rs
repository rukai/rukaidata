use crate::page::NavLink;

#[derive(Serialize)]
pub struct ErrorPage {
    pub mod_links: Vec<NavLink>,
    pub error:     String,
}

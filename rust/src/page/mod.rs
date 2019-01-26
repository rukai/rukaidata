pub mod action;
pub mod actions;
pub mod brawl_mod;
pub mod error;
pub mod fighter;
pub mod attributes;
pub mod index;
pub mod scripts;
pub mod variables;

#[derive(Serialize)]
pub struct NavLink {
    pub name:    String,
    pub link:    String,
    pub current: bool,
}

pub mod action;
pub mod actions;
pub mod subaction;
pub mod subactions;
pub mod brawl_mod;
pub mod error;
pub mod fighter;
pub mod attributes;
pub mod index;
pub mod script;
pub mod scripts;
pub mod variables;

#[derive(Clone, Serialize)]
pub struct NavLink {
    pub name:    String,
    pub link:    String,
    pub current: bool,
}

pub mod action;
pub mod brawl_mod;
pub mod error;
pub mod fighter;
pub mod index;

#[derive(Serialize)]
pub struct NavLink {
    pub name:    String,
    pub link:    String,
    pub current: bool,
}

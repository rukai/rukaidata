use std::fs;
use std::io::Write;

use sha2::{Digest, Sha256};

impl AssetPaths {
    pub fn new() -> AssetPaths {
        fs::create_dir_all("../root/assets_static").unwrap();

        let style_css = {
            let contents = include_str!("style.css");

            let minified = minifier::css::minify(&contents).unwrap();

            let mut hasher = Sha256::default();
            hasher.update(&minified);
            let hash: String = hasher
                .finalize()
                .iter()
                .map(|x| format!("{:x}", x))
                .collect();

            let path = format!("/assets_static/{}.css", hash);
            fs::write(format!("../root/{}", path), minified).unwrap();
            path
        };

        let spritesheet_png = {
            let contents = include_bytes!("spritesheet.png");

            let mut hasher = Sha256::default();
            hasher.write_all(contents).unwrap();
            let hash: String = hasher
                .finalize()
                .iter()
                .map(|x| format!("{:x}", x))
                .collect();

            let path = format!("/assets_static/{}.png", hash);
            fs::write(format!("../root/{}", path), contents).unwrap();
            path
        };

        let favicon_png = {
            let contents = include_bytes!("favicon.png");

            let mut hasher = Sha256::default();
            hasher.write_all(contents).unwrap();
            let hash: String = hasher
                .finalize()
                .iter()
                .map(|x| format!("{:x}", x))
                .collect();

            let path = format!("/assets_static/{}.png", hash);
            fs::write(format!("../root/{}", path), contents).unwrap();
            path
        };

        let subaction_render_js = {
            let contents = include_str!("subaction_render.js");

            let minified = contents;
            // let minified = minifier::js::minify(&contents); // TODO: Welp ... this is very clearly broken.
            // Can't complain though, the minifier repo does say its not ready yet :P

            let mut hasher = Sha256::default();
            hasher.update(&minified);
            let hash: String = hasher
                .finalize()
                .iter()
                .map(|x| format!("{:x}", x))
                .collect();

            let path = format!("/assets_static/{}.js", hash);
            fs::write(format!("../root/{}", path), minified.as_bytes()).unwrap();
            path
        };

        AssetPaths {
            favicon_png,
            spritesheet_png,
            style_css,
            subaction_render_js,
        }
    }
}

#[derive(Serialize)]
pub struct AssetPaths {
    pub favicon_png: String,
    pub spritesheet_png: String,
    pub style_css: String,
    pub subaction_render_js: String,
}

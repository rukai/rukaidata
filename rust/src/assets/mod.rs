use std::fs::File;
use std::io::Write;

use sha2::{Sha256, Digest};

impl AssetPaths {
    pub fn new() -> AssetPaths {
        let style_css = {
            let contents = include_str!("style.css");

            let minified = minifier::css::minify(&contents).unwrap();

            let mut hasher = Sha256::default();
            hasher.input(&minified);
            let hash: String = hasher.result().iter().map(|x| format!("{:x}", x)).collect();

            let path = format!("/assets_static/{}.css", hash);
            File::create(format!("../root/{}", path)).unwrap().write_all(minified.as_bytes()).unwrap();
            path
        };

        let subaction_render_js = {
            let contents = include_str!("subaction_render.js");

            let minified = contents;
            // let minified = minifier::js::minify(&contents); // TODO: Welp ... this is very clearly broken.
            // Can't complain though, the minifier repo does say its not ready yet :P

            let mut hasher = Sha256::default();
            hasher.input(&minified);
            let hash: String = hasher.result().iter().map(|x| format!("{:x}", x)).collect();

            let path = format!("/assets_static/{}.js", hash);
            File::create(format!("../root/{}", path)).unwrap().write_all(minified.as_bytes()).unwrap();
            path
        };

        AssetPaths { style_css, subaction_render_js }
    }
}

#[derive(Serialize)]
pub struct AssetPaths {
    pub style_css: String,
    pub subaction_render_js: String,
}

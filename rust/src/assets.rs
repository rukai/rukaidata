use std::fs::File;
use std::io::{Read, Write};

pub fn generate() {
    let mut contents = String::new();
    File::open("web/style.css").unwrap().read_to_string(&mut contents).unwrap();
    let minified = minifier::css::minify(&contents).unwrap();
    File::create("../root/assets_static/style.css").unwrap().write_all(minified.as_bytes()).unwrap();

    contents.clear();
    File::open("web/subaction-render.js").unwrap().read_to_string(&mut contents).unwrap();
    let minified = contents;
    // let minified = minifier::js::minify(&contents); // TODO: Welp ... this is very clearly broken.
    // Can't complain though, the minifier repo does say its not ready yet :P
    File::create("../root/assets_static/subaction-render.js").unwrap().write_all(minified.as_bytes()).unwrap();
}

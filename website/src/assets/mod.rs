use std::fs;
use std::io::Write;
use subprocess::{Exec, Redirection};

use sha2::{Digest, Sha256};

fn run_command_in_dir(command: &str, args: &[&str], dir: &str) {
    let data = Exec::cmd(command)
        .args(args)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .cwd(dir)
        .capture()
        .unwrap();

    if !data.exit_status.success() {
        panic!(
            "command {} {:?} exited with {:?} and output:\n{}",
            command,
            args,
            data.exit_status,
            data.stdout_str()
        )
    }
}

fn run_command(command: &str, args: &[&str]) {
    let data = Exec::cmd(command)
        .args(args)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .capture()
        .unwrap();

    if !data.exit_status.success() {
        panic!(
            "command {} {:?} exited with {:?} and output:\n{}",
            command,
            args,
            data.exit_status,
            data.stdout_str()
        )
    }
}

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

        //TODO: install wasm-bindgen
        //TODO: hash fighter_renderer.wasm file name
        let fighter_renderer_js = {
            // TODO: this will be nicer when --profile is stabilized
            //let all_args = ["run", "--profile", env!("PROFILE"), "--", "-t", topology_path];

            let all_args = if env!("PROFILE") == "release" {
                vec!["build", "--release"]
            } else {
                vec!["build"]
            };
            info!("Compiling fighter_renderer to wasm");
            run_command_in_dir("cargo", &all_args, "../fighter_renderer");

            let wasm_path = format!(
                "../fighter_renderer/target/wasm32-unknown-unknown/{}/fighter_renderer.wasm",
                env!("PROFILE")
            );
            run_command(
                "wasm-bindgen",
                &[
                    "--out-dir",
                    "../fighter_renderer/target/generated",
                    "--web",
                    &wasm_path,
                ],
            );

            let contents =
                fs::read("../fighter_renderer/target/generated/fighter_renderer_bg.wasm").unwrap();
            let path = format!("/assets_static/{}", "fighter_renderer_bg.wasm");
            fs::write(format!("../root/{}", path), contents).unwrap();

            let contents =
                fs::read("../fighter_renderer/target/generated/fighter_renderer.js").unwrap();
            let mut hasher = Sha256::default();
            hasher.update(&contents);
            let hash: String = hasher
                .finalize()
                .iter()
                .map(|x| format!("{:x}", x))
                .collect();

            let path = format!("/assets_static/{}.js", hash);
            fs::write(format!("../root/{}", path), contents).unwrap();
            path
        };

        AssetPaths {
            favicon_png,
            spritesheet_png,
            style_css,
            subaction_render_js,
            fighter_renderer_js,
        }
    }
}

#[derive(Serialize)]
pub struct AssetPaths {
    pub favicon_png: String,
    pub spritesheet_png: String,
    pub style_css: String,
    pub subaction_render_js: String,
    pub fighter_renderer_js: String,
}

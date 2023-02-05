use crate::cli::CLIResults;
use std::fs;
use std::io::Write;
use std::path::Path;
use subprocess::{Exec, Redirection};

use sha2::{Digest, Sha256};

fn run_command_in_dir(command: &str, args: &[&str], dir: &str) {
    let data = Exec::cmd(command)
        .args(args)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .cwd(dir)
        .capture()
        .unwrap_or_else(|e| panic!("Failed to run the command {command} {args:?}\n{e}"));

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
    #[allow(clippy::new_without_default)]
    pub fn new(cli: &CLIResults) -> AssetPaths {
        fs::create_dir_all("../root/assets_static").unwrap();

        let style_css = {
            let contents = include_str!("style.css");

            let minified = minifier::css::minify(contents).unwrap();

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
            hasher.update(minified);
            let hash: String = hasher
                .finalize()
                .iter()
                .map(|x| format!("{:x}", x))
                .collect();

            let path = format!("/assets_static/{}.js", hash);
            fs::write(format!("../root/{}", path), minified.as_bytes()).unwrap();
            path
        };

        const WASM_FILE_NAME: &str = "fighter_renderer_bg.wasm";
        let fighter_renderer_wasm = if cli.wasm_mode {
            {
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
                let destination_dir = "../fighter_renderer/target/generated";
                let mut bindgen = wasm_bindgen_cli_support::Bindgen::new();
                bindgen
                    .web(true)
                    .unwrap()
                    .omit_default_module_path(false)
                    .input_path(&wasm_path)
                    .generate(destination_dir)
                    .unwrap();

                run_command_in_dir(
                    "wasm-opt",
                    &["-Oz", "-o", WASM_FILE_NAME, WASM_FILE_NAME],
                    "../fighter_renderer/target/generated/",
                );
            }

            {
                let contents = fs::read(&format!(
                    "../fighter_renderer/target/generated/{}",
                    WASM_FILE_NAME
                ))
                .unwrap();
                let mut hasher = Sha256::default();
                hasher.update(&contents);
                let hash: String = hasher
                    .finalize()
                    .iter()
                    .map(|x| format!("{:x}", x))
                    .collect();
                let path = format!("/assets_static/{}.wasm", hash);
                fs::write(format!("../root/{}", path), contents).unwrap();
                path
            }
        } else {
            String::new()
        };

        let fighter_renderer_js = if cli.wasm_mode {
            let mut contents =
                fs::read_to_string("../fighter_renderer/target/generated/fighter_renderer.js")
                    .unwrap();
            let wasm_file_name = Path::new(&fighter_renderer_wasm)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap();
            assert!(contents.contains(WASM_FILE_NAME));
            contents = contents.replace(WASM_FILE_NAME, wasm_file_name);

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
        } else {
            String::new()
        };

        AssetPaths {
            favicon_png,
            spritesheet_png,
            style_css,
            subaction_render_js,
            fighter_renderer_wasm,
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
    pub fighter_renderer_wasm: String,
    pub fighter_renderer_js: String,
}

use crate::{config::Config, output::OutDir};
use std::fmt::Write;
use std::fs;
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
fn hash(value: &[u8]) -> String {
    let mut hasher = Sha256::default();
    hasher.update(value);
    hasher.finalize().iter().fold(String::new(), |mut out, x| {
        write!(out, "{x:x}").unwrap();
        out
    })
}

impl AssetPaths {
    pub fn new(config: &Config) -> AssetPaths {
        let dir = OutDir::new_path(
            Path::new(&config.web_root)
                .join("assets_static")
                .strip_prefix("/")
                .unwrap(),
        );

        let root_index = config.web_root.clone();

        let style_css = {
            let contents = include_str!("style.css");

            let minified = minifier::css::minify(contents).unwrap().to_string();

            let hash = hash(minified.as_bytes());
            dir.create_compressed_file(&format!("{hash}.css"), minified.as_bytes())
        };

        let spritesheet_png = {
            let contents = include_bytes!("spritesheet.png");
            let hash = hash(contents);
            dir.create_compressed_file(&format!("{hash}.png"), contents)
        };

        let favicon_png = {
            let contents = include_bytes!("favicon.png");
            let hash = hash(contents);
            dir.create_compressed_file(&format!("{hash}.png"), contents)
        };

        let legacy_subaction_render_js = if config.legacy_renderer {
            let contents = include_str!("subaction_render.js");

            let hash = hash(contents.as_bytes());
            dir.create_compressed_file(&format!("{hash}.js"), contents.as_bytes())
        } else {
            String::new()
        };

        const WASM_FILE_NAME: &str = "fighter_renderer_bg.wasm";
        let fighter_renderer_wasm = if config.legacy_renderer {
            String::new()
        } else {
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
                let contents = fs::read(format!(
                    "../fighter_renderer/target/generated/{}",
                    WASM_FILE_NAME
                ))
                .unwrap();
                let hash = hash(&contents);
                dir.create_compressed_file(&format!("{hash}.wasm"), &contents)
            }
        };

        let fighter_renderer_js = if config.legacy_renderer {
            String::new()
        } else {
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

            let hash = hash(contents.as_bytes());
            dir.create_compressed_file(&format!("{hash}.js"), contents.as_bytes())
        };

        AssetPaths {
            root_index,
            favicon_png,
            spritesheet_png,
            style_css,
            legacy_subaction_render_js,
            fighter_renderer_wasm,
            fighter_renderer_js,
        }
    }
}

#[derive(Serialize)]
pub struct AssetPaths {
    pub root_index: String,
    pub favicon_png: String,
    pub spritesheet_png: String,
    pub style_css: String,
    pub legacy_subaction_render_js: String,
    pub fighter_renderer_wasm: String,
    pub fighter_renderer_js: String,
}

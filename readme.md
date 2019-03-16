# Brawl Frame Data Web [![dependency status](https://deps.rs/repo/github/rukai/brawl-frame-data-web/status.svg)](https://deps.rs/repo/github/rukai/brawl-frame-data-web) [![Build Status](https://travis-ci.com/rukai/brawl-frame-data-web.svg?branch=master)](https://travis-ci.com/rukai/brawl-frame-data-web)

Uses [brawllib_rs](https://github.com/rukai/brawllib_rs) to display frame data on characters.

## To run

1.  Install stable rust via https://rustup.rs/
2.  Right click brawl in dolphin game list -> Properties -> Filesystem -> Disc -> Partition 1 -> right click fighter -> Extract Files... -> select the directory data/Brawl/fighter
3.  Copy any mod fighter directories to the directories data/MODNAMEHERE/fighter **(optional)**
4.  Open a terminal and `cd` to the directory this readme is in.
5.  Run the command: `cd rust`
6.  Run the command: `cargo run --release` This generates the website into the `root` directory.
7.  Run a webserver on the `root` directory e.g. Install python, make sure to add python to your PATH, there should be a tickbox for this in the installer, otherwise you can lookup online how to do it manually. Then run `serve.sh` for linux/mac or `serve.bat` for windows.
8.  Navigate to http://localhost:8000 in your webbrowser.

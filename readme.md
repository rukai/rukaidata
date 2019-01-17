# Brawl Frame Data Web [![dependency status](https://deps.rs/repo/github/rukai/brawl-frame-data-web/status.svg)](https://deps.rs/repo/github/rukai/brawl-frame-data-web)

Uses [brawllib_rs](https://github.com/rukai/brawllib_rs) to display frame data on characters.

## To run

1.  Install stable rust via https://rustup.rs/
2.  Install npm
3.  Right click brawl in dolphin game list -> Properties -> Filesystem -> Disc -> Partition 1 -> right click fighter -> Extract Files... -> select the directory data/Brawl/fighter
4.  Copy any mod fighter directories to the directories data/MODNAMEHERE/fighter **(optional)**
5.  Open a terminal and `cd` to the directory this readme is in.
6.  Run the command: `cd rust`
6.  Run the command: `cargo run --release`
7.  Compiled and generated files are put in the `root` directory
8.  Run a webserver on the `root` directory e.g. Use the `serve.py` script.

## Original setup

Originally setup using steps similar to: https://anderspitman.net/blog/static-react-rust-webapp/

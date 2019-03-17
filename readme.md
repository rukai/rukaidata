# Brawl Frame Data Web [![dependency status](https://deps.rs/repo/github/rukai/brawl-frame-data-web/status.svg)](https://deps.rs/repo/github/rukai/brawl-frame-data-web) [![Build Status](https://travis-ci.com/rukai/brawl-frame-data-web.svg?branch=master)](https://travis-ci.com/rukai/brawl-frame-data-web)

Uses [brawllib_rs](https://github.com/rukai/brawllib_rs) to display frame data on characters.

## To run

1.  Install stable rust via https://rustup.rs/
2.  Right click brawl in dolphin game list -> Properties -> Filesystem -> Disc -> Right click Partition 1 -> Extract Files... -> select the directory `data/Brawl`
3.  Copy the entire contents of a brawl mod sd card to the directories `data/MODNAMEHERE` **(optional)**
4.  Open a terminal and `cd` to the directory this readme is in.
5.  Run the command: `cd rust`
6.  Run the command: `cargo run --release` This generates the website into the `root` directory.
7.  Run a webserver on the `root` directory to access the generated website e.g.
    1.  Install python. Make sure to add python to your PATH environment variable, there should be a checkbox for this in the windows installer other OSs should handle this by default.
    2.  Run `serve.sh` for linux/mac or `serve.bat` for windows.
    3.  Navigate to http://localhost:8000 in your webbrowser.

## `data` folder cleanup

Most of the files in the `data` folder are actually unused by brawl-frame-data-web, so after you get everything working, feel free to delete unused files, maintaining the same directory tree structure.
The following files and folders must be kept:
*   `*.pac` files in the `fighter` folder
*   `codes/RSBE01.gct`

However I may make other files required in the future so watch out. :)

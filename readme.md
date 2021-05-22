# Rukaidata
[![dependency status](https://deps.rs/repo/github/rukai/rukaidata/status.svg)](https://deps.rs/repo/github/rukai/rukaidata)

Uses [brawllib_rs](https://github.com/rukai/brawllib_rs) to display frame data on characters.

How does it work? Read the [writeup](docs/writeup.md).

## Run the rukaidata website on your machine

1.  Install stable rust via https://rustup.rs/ (use the default settings)
2.  Right click brawl in dolphin game list -> Properties -> Filesystem -> Disc -> Right click Partition 1 -> Extract Files... -> select the directory `data/Brawl`
3.  Copy the entire contents of a brawl mod sd card to the directory `data/MODNAMEHERE` **(optional)**
4.  Open a terminal and `cd` to the directory this readme is in.
5.  Run the command: `cd website`
6.  Run the command: `cargo run --release -- -w` This generates the website into the `root` directory.
7.  Run a webserver on the `root` directory to access the generated website e.g.
    1.  Install python. Make sure to add python to your PATH environment variable, there should be a checkbox for this in the windows installer other OSs should handle this by default.
    2.  Run `serve.sh` for linux/mac or `serve.bat` for windows.
    3.  Navigate to http://localhost:8000 in your webbrowser.

## Filters

You can use the following arguments to specify filters:

*   `-m` List of mod folders in `data/` to use
*   `-f` List of fighters to use, specified by their internal name

e.g. To only generate framedata for PM3.6 marth and squirtle run this command:

`cargo run --release -- -mpm3.6 -fmarth,pokezenigame -w`

Using filters will save you generation time and disk space.

## Webpage and gif generation

By default rukaidata will generate no output, however you use the following flags to additively specify what to generate:

*   `-w` Generate webpages
*   `-g` Generate subaction gifs`

e.g. To generate webpages and gifs for everything run this command:

`cargo run --release -- -wg`


## `data` folder cleanup

Most of the files in the `data` folder are actually unused by rukaidata, so after you get everything working, feel free to delete unused files, maintaining the same directory tree structure.
The following files and folders must be kept:
*   `*.pac` files in the `fighter` folder
*   `RSBE01.gct`

However I may make other files required in the future so watch out. :)

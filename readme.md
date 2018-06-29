# Brawl Frame Data Web [![dependency status](https://deps.rs/repo/github/rukai/brawl-frame-data-web/status.svg)](https://deps.rs/repo/github/rukai/brawl-frame-data-web)

Uses [brawllib_rs](https://github.com/rukai/brawllib_rs) to display frame data on characters.

## To run

1.  install rustup https://rustup.rs/
2.  right click brawl in dolphin game list -> Properties -> Filesystem -> Disc -> Partition 1 -> right click fighter -> Extract Files... -> select the directory data/brawl/fighter
3.  copy any mod fighter directories to the directories data/MODNAMEHERE/fighter **(optional)**
4.  open a terminal and `cd` to the directory this readme is in.
5.  run the command: `cargo run --release`
6.  This will take a long time to complete.
7.  By default serves at localhost:8000

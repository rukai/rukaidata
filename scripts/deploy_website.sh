#!/bin/sh

ssh vps "rm -r brawl-frame-data-web/npm-webpack"
ssh vps "rm -r brawl-frame-data-web/templates"

sftp vps <<EOF
cd brawl-frame-data-web
lcd brawl-frame-data-web
put /home/rubic/Projects/Crates/brawl-frame-data-web/brawl-frame-data-web/target/release/brawl-frame-data-web
put /home/rubic/Projects/Crates/brawl-frame-data-web/Rocket.toml

mkdir data

put -r templates

mkdir npm-webpack
cd npm-webpack
mkdir dist
cd dist
put /home/rubic/Projects/Crates/brawl-frame-data-web/brawl-frame-data-web/npm-webpack/dist/main.js

EOF

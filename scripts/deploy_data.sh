#!/bin/sh

ssh vps "rm -r brawl-frame-data-web/data"

sftp vps <<EOF
cd brawl-frame-data-web
lcd brawl-frame-data-web
put -r data
EOF

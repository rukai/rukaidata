cd brawl-frame-data-web/npm-webpack
npm run release

sftp vps <<EOF
cd serve/framedata/dist
lcd dist
rename main.js main.js.bak
put main.js
EOF

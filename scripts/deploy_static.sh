cd brawl-frame-data-web/npm-webpack
npm run release

cd ../../static
httrack localhost:8000 --update

rsync --delete -e ssh -rvuh /home/rubic/Projects/Crates/brawl-frame-data-web/static/localhost_8000/ vps:/home/rubic/serve/framedata/

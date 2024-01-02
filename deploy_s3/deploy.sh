#!/bin/sh

set -e -u
cd "$( dirname -- "${BASH_SOURCE[0]}" )/../root"

# `cache-control: max-age=31536000` results in the browser never requesting the file until the cache is cleaned or cleared.
echo -e "\ndeploy assets_static"
aws s3 sync assets_static s3://$OUTPUT_BUCKET_NAME/assets_static --cache-control max-age=31536000 --content-encoding gzip

# `cache-control: no-cache` results in:
# 1.   a 5 minute period in which no requests are made to the server, relying entirely on cached data (browser dependent)
# 2.   requests are then made to the server to verify if the cache is up to date (HTTP 304) or not up to date (HTTP 200)
echo -e "\ndeploy everything else"
aws s3 sync . s3://$OUTPUT_BUCKET_NAME --exclude "assets_static/*" --cache-control no-cache --content-encoding gzip
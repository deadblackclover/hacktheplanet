#!/bin/sh

appName=$1
imageId=$(docker inspect registry.heroku.com/$appName/web --format={{.Id}})
payload='{"updates":[{"type":"web","docker_image":"'"$imageId"'"}]}'

curl -n -X PATCH https://api.heroku.com/apps/$appName/formation \
-d "$payload" \
-H "Content-Type: application/json" \
-H "Accept: application/vnd.heroku+json; version=3.docker-releases" \
-H "Authorization: Bearer $HEROKU_API_KEY"
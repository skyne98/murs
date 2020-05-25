#!/bin/bash

set -o errexit -o nounset

rev=$(git rev-parse --short HEAD)

# assemble the output
mkdir output
mv book/book/* output/

# now deploy
cd output

git init
git config user.name "skyne98"
git config user.email "ahalahan@gmail.com"

git remote add upstream "https://$GH_TOKEN@github.com/skyne98/murs.git"
git fetch upstream
git reset upstream/gh-pages

touch .
touch .nojekyll

git add -A .
git commit -m "Rebuild pages at ${rev}"
git push -q upstream HEAD:gh-pages
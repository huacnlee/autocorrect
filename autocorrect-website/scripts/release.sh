# abort on errors
set -e

rm -Rf dist/
# build
bun install
bun run build

# navigate into the build output directory
cp dist/index.html dist/404.html
cd dist

# place .nojekyll to bypass Jekyll processing
echo > .nojekyll

git init
git checkout -B main
git add -A
git commit -m 'deploy'

git push -f git@github.com:huacnlee/autocorrect.git main:gh-pages

cd -
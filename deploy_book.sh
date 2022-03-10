#!/usr/bin/sh

git checkout gh-pages

git checkout main "book/src/*" "book/book.toml"

rm -rf src

mv ./book/book.toml ./book/src .

rm -rf book

mdbook build

git add .

git commit -m "rust book built automatically"

git push github gh-pages

echo "deployed book with code=0"

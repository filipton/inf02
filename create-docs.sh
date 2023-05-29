#!/bin/bash

git clone https://github.com/filipton/inf02-docs docs
cd docs ; git pull ; cd ..
cp -r ./docs/images/* ./frontend/static/images

cd docs-creator ; cargo run ; cd ..

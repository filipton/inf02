#!/bin/bash

cd docs ; git pull ; cd ..
cd docs-creator ; cargo run ; cd ..

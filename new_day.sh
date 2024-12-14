#!/bin/bash

echo "
[[bin]] 
name = \"$1\"
path = \"src/$1/main.rs\"" >> Cargo.toml

cp -r template src/$1

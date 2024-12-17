#!/bin/bash

echo "
[[bin]] 
name = \"$1\"
path = \"src/$1/main.rs\"
tes = false" >> Cargo.toml

cp -r template src/$1

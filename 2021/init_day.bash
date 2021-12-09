#!/bin/env bash

day=$1
dir="day$day"

mkdir "$dir"
cd "$dir"
npx tsc --init
ln -s ../prettierrc.json .prettierrc.json
ln -s ../eslintrc.json .eslintrc.json

cp ../package.json .
npm i

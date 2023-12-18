#!/bin/bash
set -euo pipefail

if [[ $# -eq 0 ]]
then
    echo "no arguments"
    exit 1
fi

NUM=$1
if [[ ${#NUM} -gt 2 ]]
then
    echo "not 2 chars"
    exit 1
fi

if [[ ! $1 =~ ^[0-9]+$ ]]
then
    echo "not digits"
    exit 1
fi

mkdir $NUM
cp template.rs $NUM/a.rs
cp template.toml $NUM/Cargo.toml
sed -i '' -e "s/<NUMBER>/$NUM/g" $NUM/Cargo.toml
cd $NUM

# TRIMNUM=$((10#$NUM))
# curl -sL "https://adventofcode.com/2023/day/$TRIMNUM/input" > input.txt
# rustc a.rs
# ./a < input.txt > output.txt
# diff input.txt output.txt
# cd ..

echo "tada!"

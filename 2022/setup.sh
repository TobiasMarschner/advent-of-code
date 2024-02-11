#!/bin/bash
set -euo pipefail

if [[ "$#" != "1" ]]; then
  echo "  Usage: ./setup.sh <day>"
  echo "Example: ./setup.sh 13"
  exit 1
fi

ident="day${1}-part1"
cp -r aoc_template "$ident"
sed -i -e "s/aoc_template/$ident/g" "$ident/Cargo.toml"

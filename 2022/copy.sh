#!/bin/bash
set -euo pipefail

if [[ "$#" != "1" ]]; then
  echo "  Usage: ./copy.sh <day>"
  echo "Example: ./copy.sh 13"
  exit 1
fi

from="day${1}-part1"
to="day${1}-part2"
cp -r "$from" "$to"
sed -i -e "s/$from/$to/g" "$to/Cargo.toml"

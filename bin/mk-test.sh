#!/bin/sh

set -eu
cd "$(dirname "$0")"
cd ..

panic() {
  echo "$1"
  exit 1
}

if [ "$#" -eq 0 ]; then
  panic "usage: $0 <test>..."
fi

for x in "$@"; do
  mkdir "tests/$x"
  touch "tests/$x/inp.txt"
  touch "tests/$x/out.txt"
done

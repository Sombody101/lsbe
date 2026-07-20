#!/bin/bash

source "$LS/bashext/debugging/prof.sh"

readonly TEST='gecho "[blue on blinking dunderlined black]Hello, World! [/]" >/dev/null'
readonly LOOPS=50000

echo "test: $TEST"
echo "loops: $LOOPS"

echo
echo "Benching binary"
echo "----------------"
#enable -d gecho >/dev/null
prof.bench $LOOPS "$TEST"

echo
echo "Benching builtin"
echo "----------------"
enable -f ./target/debug/liblsbe.so gecho >/dev/null
prof.bench $LOOPS "$TEST"
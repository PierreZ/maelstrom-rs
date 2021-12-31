#!/usr/bin/env bash

mkdir -p "target"

wget -c https://github.com/jepsen-io/maelstrom/releases/download/0.2.0/maelstrom.tar.bz2 -O ./target/maelstrom.tar.bz2
tar -xjf ./target/maelstrom.tar.bz2 -C ./target
#!/bin/sh
if [ $(id -u) != 0 ]; then
  echo "You have to run this script as root"
  exit
fi
cp ./target/release/devi /usr/bin/devi
echo "success: devi installed at /usr/bin/devi"
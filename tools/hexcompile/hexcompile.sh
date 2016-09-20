#!/bin/bash

# This tools compiles the .hex files in this project

cat $1 | sed 's/#.*$//g' | xxd -r -p
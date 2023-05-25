#!/bin/bash
#
cbindgen --config cbindgen.toml --crate tokenizers-c --lang c --output include/tokenizers.h

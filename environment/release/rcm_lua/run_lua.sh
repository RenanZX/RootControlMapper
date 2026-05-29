#!/bin/bash

DIRETORIO_ATUAL="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

export LUA_PATH="$DIRETORIO_ATUAL/lua_modules/?.lua;$DIRETORIO_ATUAL/lua_modules/?/init.lua;;"
export LUA_CPATH="$DIRETORIO_ATUAL/c_modules/?.so;;"

export LD_LIBRARY_PATH="$DIRETORIO_ATUAL/lib:$LD_LIBRARY_PATH"

$DIRETORIO_ATUAL/bin/lua "$@"

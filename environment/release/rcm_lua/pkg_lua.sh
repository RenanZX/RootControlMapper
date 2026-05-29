#!/bin/bash

DIRETORIO_ATUAL="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

export PATH="$DIRETORIO_ATUAL:$PATH"

$DIRETORIO_ATUAL/bin/luarocks --tree "$DIRETORIO_ATUAL/lua_modules" install "$@"

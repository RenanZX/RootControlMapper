#!/bin/sh

DIR_ATUAL="$1/rcm_lua"
DIR_DESTINO="$2"

VERSION="lua-5.5.0"

mkdir -p "$DIR_DESTINO"
cp -r "$DIR_ATUAL/bin" "$DIR_DESTINO/"
cp "$DIR_ATUAL/run_lua.sh" "$DIR_ATUAL/pkg_lua.sh" "$DIR_DESTINO/"
cd $DIR_ATUAL
wget "http://www.lua.org/ftp/$VERSION.tar.gz"
tar -xvzf "$VERSION.tar.gz"
cd "$VERSION"
make linux
strip src/lua src/luac
mv src/lua src/luac "$DIR_DESTINO/bin/"
cd ..
rm -rf "$VERSION"
rm -f "$VERSION.tar.gz"
mkdir -p "$DIR_DESTINO/c_modules" "$DIR_DESTINO/lua_modules"

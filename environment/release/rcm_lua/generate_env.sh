#!/bin/sh

DIR_ATUAL="$1/rcm_lua"
DIR_DESTINO="$2"

LUA_VERSION="lua-5.4.8"
ROCKS_VERSION="luarocks-3.13.0"

gen_lua_bin() {
  cd $DIR_ATUAL
  wget "http://www.lua.org/ftp/$LUA_VERSION.tar.gz"
  tar -xvzf "$LUA_VERSION.tar.gz"
  cd "$LUA_VERSION"
  make linux
  strip src/lua src/luac
  mv src/lua src/luac "$DIR_DESTINO/bin/"
  cp src/lua.h src/luaconf.h src/lualib.h src/lauxlib.h src/lua.hpp "$DIR_DESTINO/include/"
  cd ..
  rm -rf "$LUA_VERSION"
  rm -f "$LUA_VERSION.tar.gz"
}

gen_luarocks_bin() {
  cd $DIR_ATUAL
  wget "https://luarocks.org/releases/$ROCKS_VERSION.tar.gz"
  tar zxpf "$ROCKS_VERSION.tar.gz"
  cd "$ROCKS_VERSION"
  ./configure --prefix="$DIR_DESTINO" --with-lua-bin="$DIR_DESTINO/bin" --with-lua-include="$DIR_DESTINO/include"
  make && make install
  cd ..
  rm -rf "$ROCKS_VERSION"
  rm -f "$ROCKS_VERSION.tar.gz"
}

mkdir -p "$DIR_DESTINO/bin" "$DIR_DESTINO/include"
cp "$DIR_ATUAL/run_lua.sh" "$DIR_ATUAL/pkg_lua.sh" "$DIR_DESTINO/"
gen_lua_bin
gen_luarocks_bin
mkdir -p "$DIR_DESTINO/c_modules" "$DIR_DESTINO/lua_modules"

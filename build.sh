#!/bin/bash

SOURCE_RELEASE="environment/release"
GEN_ENV_PY="$SOURCE_RELEASE/rcm_env/generate_env.sh"
SOURCE_DEBUG="environment/debug"
GEN_ENV_LUA="$SOURCE_RELEASE/rcm_lua/generate_env.sh"
ABS_DEBUG="$(pwd)/$SOURCE_DEBUG"
ABS_BUILD="$(pwd)/$SOURCE_RELEASE"

tar_name() {
  NAME_TAR=$(cargo pkgid | cut -d# -f2 | cut -d: -f2 | sed 's/_/-/g; s/@/-/')
  echo "$NAME_TAR-linux-x86_64.tar.gz"
}

tar_build() {
  NAME_TAR=$(tar_name)
  cd build && tar -czvf "../$NAME_TAR" .
}

debug_env() {
  if [[ ! -d "$SOURCE_DEBUG/rcm_lua" || "$1" = "rebuild" ]]; then
    chmod +x "$GEN_ENV_LUA"
    "$GEN_ENV_LUA" "$ABS_BUILD" "$ABS_DEBUG/rcm_lua"
    mkdir -p "$SOURCE_DEBUG/fix"
    cp -f "$GEN_ENV_LUA" "$SOURCE_DEBUG/fix/env_lua.sh"
  fi
  if [[ ! -d "$SOURCE_DEBUG/rcm_py" || "$1" = "rebuild" ]]; then
    chmod +x "$GEN_ENV_PY"
    "$GEN_ENV_PY" "$SOURCE_DEBUG" "$SOURCE_RELEASE/rcm_env"
  fi
}

if [[ $1 == "--release" ]]; then
  ROOT_BUILD="./build/root-cmap"
  mkdir -p "$ROOT_BUILD/fix"
  cargo build --release
  mv ./target/release/root_ctrl_mapper "$ROOT_BUILD/"
  cp -r "$SOURCE_RELEASE/data/." "$ROOT_BUILD/"
  chmod +x "$GEN_ENV_LUA"
  "$GEN_ENV_LUA" "$ABS_BUILD" "$(pwd)/build/root-cmap/rcm_lua"
  cp -f "$GEN_ENV_LUA" "$ROOT_BUILD/fix/env_lua.sh"
  cp -r "$SOURCE_RELEASE/rcm_env" ./build/
  cp "$SOURCE_RELEASE/install.sh" "$SOURCE_RELEASE/root-ctrl-mapper" ./build/
  if [[ $2 == "tar" ]]; then
    tar_build 
  fi
elif [[ $1 == "--tar" ]]; then
  if [ -d "build" ]; then
    tar_build 
  fi
elif [[ $1 == "--debug" ]]; then
  if [ ! -d "$SOURCE_DEBUG" ]; then
    mkdir -p "$SOURCE_DEBUG"
    cp -r "$SOURCE_RELEASE/data/." "$SOURCE_DEBUG/"
    debug_env 
    
  elif [[ $2 == "preserve-json" ]]; then
    mkdir -p "$SOURCE_DEBUG"
    cp -r "$SOURCE_RELEASE/data/scripts" "$SOURCE_DEBUG/"
    cp -r "$SOURCE_RELEASE/data/sfx" "$SOURCE_DEBUG/"

    debug_env "rebuild"
  fi
elif [[ $1 == "--clear" ]]; then
  TAR_NAME=$(tar_name)
  echo "$TAR_NAME"
  [ -d build ] && rm -rf build
  [ -f $TAR_NAME ] && rm "$TAR_NAME"
elif [[ $1 == "--help" ]]; then
  cat <<EOF
--release: build app and with tar parameter will build app and make the tar release file
--tar: generate tar file of the build
--debug: generate debug enviorment to test the app
--clear: clear the build files and tar
EOF
fi

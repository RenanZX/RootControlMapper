#!/bin/bash

SOURCE_BUILD="environment/release"
SOURCE_DEBUG="environment/debug"

tar_name() {
  NAME_TAR=$(cargo pkgid | cut -d# -f2 | cut -d: -f2 | sed 's/_/-/g; s/@/-/')
  echo "$NAME_TAR-linux-x86_64.tar.gz"
}

tar_build() {
  NAME_TAR=$(tar_name)
  cd build && tar -czvf "../$NAME_TAR" .
}

if [[ $1 == "--release" ]]; then
  mkdir -p ./build/root-cmap
  cargo build --release
  mv ./target/release/root_ctrl_mapper ./build/root-cmap/
  cp -r "$SOURCE_BUILD/json" "$SOURCE_BUILD/scripts" ./build/root-cmap/
  cp "$SOURCE_BUILD/install.sh" "$SOURCE_BUILD/root-ctrl-mapper" ./build/
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
    cp -r "$SOURCE_BUILD/json" "$SOURCE_BUILD/scripts" "$SOURCE_DEBUG/"
  fi
elif [[ $1 == "--clear" ]]; then
  TAR_NAME=$(tar_name)
  [ -d build ] && rm -rf build
  [ -f $TAR_NAME ] && rm "$TAR_NAME"
fi

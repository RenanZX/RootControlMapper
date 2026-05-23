#!/bin/bash

SOURCE_BUILD="environment/release"
GEN_ENV_PY="$SOURCE_BUILD/rcm_env/generate_env.sh"
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
  cp -r "$SOURCE_BUILD/data/." ./build/root-cmap/
  cp -r "$SOURCE_BUILD/rcm_env" ./build/
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
    cp -r "$SOURCE_BUILD/data/." "$SOURCE_DEBUG/"
    chmod +x "$GEN_ENV_PY"
    "$GEN_ENV_PY" "$SOURCE_DEBUG" "$SOURCE_BUILD/rcm_env"
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

#!/bin/bash

INSTALL_PATH="$HOME/.local/share"
SCRIPT_PATH="$HOME/.local/bin"

if [[ $1 == "-u" ]]; then
  rm -rf "$INSTALL_PATH/root-cmap"
  rm -f "$SCRIPT_PATH/root-ctrl-mapper"
else
  mkdir -p "$INSTALL_PATH"
  mkdir -p "$SCRIPT_PATH"

  if [ -d "$INSTALL_PATH/root-cmap" ]; then
    rm -rf "$INSTALL_PATH/root-cmap"
  fi

  chmod +x root-ctrl-mapper
  chmod +x root-cmap/root_ctrl_mapper

  cp -r root-cmap "$INSTALL_PATH/"
  cp root-ctrl-mapper "$SCRIPT_PATH/"
fi

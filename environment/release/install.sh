#!/bin/bash

# Terminal Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

INSTALL_PATH="$HOME/.local/share"
SCRIPT_PATH="$HOME/.local/bin"
ABS_PATH=$(pwd)
GEN_PYENV="./rcm_env/generate_env.sh"

if [[ $1 == "-u" ]]; then
  echo -e "${RED}🗑️  Uninstalling Root Control Mapper...${NC}"
  rm -rf "$INSTALL_PATH/root-cmap"
  rm -f "$SCRIPT_PATH/root-ctrl-mapper"
  echo -e "${GREEN}✅ Done! Uninstallation completed.${NC}"
else
  echo -e "${BLUE}🎮 Starting Root Control Mapper installation...${NC}"
  mkdir -p "$INSTALL_PATH"
  mkdir -p "$SCRIPT_PATH"

  # Executable permissions
  chmod +x root-ctrl-mapper
  chmod +x "$GEN_PYENV"
  chmod +x root-cmap/root_ctrl_mapper

  if [ -d "$INSTALL_PATH/root-cmap" ]; then
    echo -e "${YELLOW}🔄 Existing installation found. Updating...${NC}"
    cp root_ctrl_mapper "$SCRIPT_PATH/"
    cp -r "root-cmap/scripts" "$INSTALL_PATH"
    cp -r "root-cmap/sfx" "$INSTALL_PATH"
    
    if [ ! -d "$INSTALL_PATH/root-cmap/rcm_py" ]; then
      echo -e "⚙️  Setting up Python environment..."
      "$GEN_PYENV" "$INSTALL_PATH/root-cmap" "$ABS_PATH/rcm_env"  
    fi
    echo -e "${GREEN}✨ Update completed successfully!${NC}"
  else
    echo -e "⚙️  Creating environment and copying files...${NC}"
    cp -r "root-cmap" "$INSTALL_PATH/root-cmap"
    "$GEN_PYENV" "$INSTALL_PATH/root-cmap" "$ABS_PATH/rcm_env" 
    cp root-ctrl-mapper "$SCRIPT_PATH/"
    echo -e "${GREEN} Installation completed successfully!${NC}"
  fi
fi

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
RULES_FILE="/etc/udev/rules.d/99-z-mm-virtual-controller.rules"

add_group_settings() {
  sudo groupadd inputrcm

  cat << 'EOF' | sudo tee "$RULES_FILE" > /dev/null
KERNEL=="event*", SUBSYSTEM=="input", ATTRS{name}=="Xbox Mouse Mode", MODE="0660", OWNER="root", GROUP="inputrcm", ENV{ID_INPUT_JOYSTICK}="1", ENV{SYSTEMD_READY}="1"
KERNEL=="event*", SUBSYSTEM=="input", ATTRS{name}=="Xbox Mouse Mode", ENV{ACL_MANAGED}="0"
EOF

  sudo udevadm control --reload-rules && sudo udevadm trigger

}

add_env() {
  if [[ "$SHELL" == */zsh ]]; then
      RC_FILE="$HOME/.zshrc"
  elif [[ "$SHELL" == */bash ]]; then
      RC_FILE="$HOME/.bashrc"
  else
      RC_FILE="$HOME/.bashrc"
  fi

  ENV_LINE='export SDL_GAMECONTROLLER_IGNORE_DEVICES=0x2934/0x5690'

  if ! grep -qF "$ENV_LINE" "$RC_FILE" 2>/dev/null; then
      echo "" >> "$RC_FILE"
      echo "# Enviorment for mouse mode of Root Control Mapper" >> "$RC_FILE"
      echo "$ENV_LINE" >> "$RC_FILE"
  fi
}

if [[ $1 == "-u" ]]; then
  echo -e "${RED}🗑️  Uninstalling Root Control Mapper...${NC}"
  rm -rf "$INSTALL_PATH/root-cmap"
  rm -f "$SCRIPT_PATH/root-ctrl-mapper"
  echo -e "${GREEN}✅ Done! Uninstallation completed.${NC}"
elif [[ $1 == "-ap" ]]; then
  echo -e "${BLUE} Adding Permitions to Root Control Mapper (Make sure you are using sudo)...${NC}"
  add_group_settings
  echo -e "${GREEN} Permissions added successfully!${NC}"
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
    cp -f root-ctrl-mapper "$SCRIPT_PATH/"
    cp -rf "root-cmap/scripts" "$INSTALL_PATH/root-cmap/"
    cp -rf "root-cmap/sfx" "$INSTALL_PATH/root-cmap/"
    cp -f "root-cmap/root_ctrl_mapper" "$INSTALL_PATH/"
    
    if [ ! -d "$INSTALL_PATH/root-cmap/rcm_py" ]; then
      echo -e "⚙️  Setting up Python environment..."
      "$GEN_PYENV" "$INSTALL_PATH/root-cmap" "$ABS_PATH/rcm_env"  
    fi
    add_env
    echo -e "${GREEN}✨ Update completed successfully!${NC}"
  else
    echo -e "⚙️  Creating environment and copying files...${NC}"
    cp -r "root-cmap" "$INSTALL_PATH/"
    "$GEN_PYENV" "$INSTALL_PATH/root-cmap" "$ABS_PATH/rcm_env" 
    cp root-ctrl-mapper "$SCRIPT_PATH/"
    add_env
    echo -e "${GREEN} Installation completed successfully!${NC}"
  fi

fi

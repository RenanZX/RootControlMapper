#!/bin/bash

# Define o nome da pasta do ambiente virtual e o arquivo de dependências
INSTALL_DIR=$1
REQ_PATH=$2

if [ -n "$INSTALL_DIR" ] && [ -d "$INSTALL_DIR" ]; then
  ENV_NAME="$INSTALL_DIR/rcm_py"
else
  ENV_NAME="rcm_py"
fi

if [ -n "$REQ_PATH" ] && [ -d "$REQ_PATH" ]; then
  REQ_FILE="$REQ_PATH/requirements.txt"
else
  REQ_FILE="requirements.txt"
fi

# echo "🔎 Verificando instalação do Python..."

echo "🎮 Starting Root Control Mapper Py env setup..."

# 1. Verifica qual comando do Python está disponível no sistema
if command -v python3 &> /dev/null; then
    PYTHON_CMD="python3"
elif command -v python &> /dev/null; then
    # Garante que o comando 'python' é da versão 3
    VERSION=$($PYTHON_CMD -V 2>&1 | awk '{print $2}')
    if [[ "$VERSION" == 3* ]]; then
        PYTHON_CMD="python"
    else
        echo "❌ Error: Only Python 2 was found. Please install Python 3."
        exit 1
    fi
else
    echo "❌ Error: Python is not installed on this system."
    exit 1
fi

echo "✅ Python found: $($PYTHON_CMD -V)"

# 2. Verifica se o módulo venv está instalado (comum faltar no Ubuntu/Debian)
if ! $PYTHON_CMD -c "import venv" &> /dev/null; then
    echo "❌ Error: The 'venv' module is not installed."
    echo "👉 Run: sudo apt install python3-venv (or your system's equivalent)"
    exit 1
fi

# 3. Cria o ambiente virtual se ele não existir
if [ ! -d "$ENV_NAME" ]; then
    echo "📦 Creating virtual env '$ENV_NAME'..."
    $PYTHON_CMD -m venv "$ENV_NAME"
else
    echo "ℹ️ Virtual env '$ENV_NAME' already exists. Skipping creation."
fi

# 4. Ativa o ambiente virtual
# echo "🔄 Ativando o ambiente virtual..."
source "$ENV_NAME/bin/activate"

# 5. Instala as dependências se o arquivo requirements.txt existir
if [ -f "$REQ_FILE" ]; then
    echo "📥 Installing dependencies..."
    pip install --upgrade pip
    pip install -r "$REQ_FILE"
    echo "🏁 Finished."
else
    echo "⚠️ Warning: File '$REQ_FILE' not found. No libraries were installed."
fi

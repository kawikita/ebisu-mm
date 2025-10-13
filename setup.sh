#!/bin/bash
# This script sets up the development environment by installing necessary packages and tools.

# 必要なパッケージのインストール
function install_apt_packages() {
    sudo apt-get update
    sudo apt-get install -y git curl build-essential ca-certificates pkg-config libssl-dev maven jq
    sudo apt-get autoremove -y
    sudo apt-get clean
}

# Rustのインストール
function install_rust() {
    if command -v rustc >/dev/null 2>&1; then
        echo "Rust is already installed."
    else
        curl https://sh.rustup.rs -sSf | sh -s -- -y
    fi
    insert_env_into_rc_files ".cargo/env"
}

# OpenAPI Generator CLIのインストール
function install_openapi_generator() {
    if command -v $OPENAPI_GENERATOR_CLI >/dev/null 2>&1; then
        echo "OpenAPI Generator CLI is already installed."
    else
        rm -rf "$OPENAPI_GENERATOR_DIR"
        mkdir -p "$OPENAPI_GENERATOR_DIR"
        curl -o "$OPENAPI_GENERATOR_PATH" "$OPENAPI_GENERATOR_URL"
        chmod +x "$OPENAPI_GENERATOR_PATH"
        $OPENAPI_GENERATOR_PATH version
    fi
    create_openapi_generator_env_file
    insert_env_into_rc_files ".openapi/env"
}

# OpenAPI Generator CLIの環境設定ファイルを作成
function create_openapi_generator_env_file() {
    if [ ! -f "$HOME/.openapi/env" ]; then
        cat > "$HOME/.openapi/env" <<EOF
#!/bin/sh
# openapi-generator-cli shell setup
# affix colons on either side of \$PATH to simplify matching
DOT_OPENAPI_BIN="\$HOME/.openapi/bin"
case ":\${PATH}:" in
    *:"\$DOT_OPENAPI_BIN":*)
        ;;
    *)
        # Prepending path in case a system-installed openapi-generator-cli needs to be overridden
        export PATH="\$DOT_OPENAPI_BIN:\$PATH"
        ;;
esac
EOF
    fi
}

# 指定された環境変数設定ファイルをすべてのシェル設定ファイルに追記
function insert_env_into_rc_files() {
    ENV_FILE_PATH="$1"
    for rc_file in $RC_FILES; do
        if ! grep -e "source \$HOME/$ENV_FILE_PATH" "$rc_file"; then
            echo "source \$HOME/$ENV_FILE_PATH" >> "$rc_file"
        fi
    done
}

########
# main #
########
OPENAPI_GENERATOR_CLI="openapi-generator-cli"
OPENAPI_GENERATOR_DIR="$HOME/.openapi/bin"
OPENAPI_GENERATOR_PATH="$OPENAPI_GENERATOR_DIR/$OPENAPI_GENERATOR_CLI"
OPENAPI_GENERATOR_URL="https://raw.githubusercontent.com/OpenAPITools/openapi-generator/master/bin/utils/openapi-generator-cli.sh"
RC_FILES=$(ls -1 $HOME/.*shrc)

install_apt_packages
install_rust
install_openapi_generator

echo "==============================="
echo "Setup completed."
echo "Please restart your terminal or run"
echo " 'source ~/.*shrc'(Generally, it's .bashrc or .zshrc.)"
echo " to apply changes."
apt-get update
apt-get install -y git curl build-essential ca-certificates pkg-config libssl-dev
apt-get autoremove -y
apt-get clean

curl https://sh.rustup.rs -sSF | sh -s -- -y


if [ -f "$HOME/.bashrc" ]; then
    echo 'source $HOME/.cargo/env' >> $HOME/.bashrc
fi
if [ -f "$HOME/.zshrc" ]; then
    echo 'source $HOME/.cargo/env' >> $HOME/.zshrc
fi
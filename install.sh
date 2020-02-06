# cargo install cargo-web
# sudo apt-get install apt-transport-https

# Install yarn:
curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | sudo apt-key add -
echo "deb https://dl.yarnpkg.com/debian/ stable main" | sudo tee /etc/apt/sources.list.d/yarn.list

sudo apt update && sudo apt install yarn

# Assuming npm is already installed...
npm install --save-dev webpack webpack-cli webpack-dev-server

yarn add --dev @wasm-tool/wasm-pack-plugin

cargo install wasm-pack

#!/bin/bash
set -e

echo "Building CUI app to Wasm..."
cd app
wasm-pack build --target web
cd ..

echo "Build complete!"
echo "  HTML: app/target/html/index.html"
echo "  Wasm: app/pkg/"
echo ""
echo "To serve locally:"
echo "  cargo run -p cascading-ui-net-server"

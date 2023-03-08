# Web App

Full-stack web template in Rust using [rust-actix-web-api](https://github.com/axgonz/rust-actix-web-api/blob/main/README.md#dev-dependencies) and [rust-leptos-spa](https://github.com/axgonz/rust-leptos-spa/blob/main/README.md#dev-dependencies) templates.

## Dev dependencies

Web Assembly build target
```bash
# Install wasm32 target
rustup target add wasm32-unknown-unknown
```

Dev tools
```bash
# Install degit
cargo install degit

# Install trunk
cargo install trunk

# Install tailwind
pushd spa && npm install && popd
```

## Setup

```bash
# Define the new project's name 
proj_name="my_rust_app"

# Use `this` template 
degit https://github.com/axgonz/rust-web-template $proj_name && pushd $proj_name

# Use frontend and backend templates
degit https://github.com/axgonz/rust-actix-web-api api && rm -r api/.vscode
degit https://github.com/axgonz/rust-leptos-spa spa && rm -r spa/.vscode
```

In a new terminal called `actix`
```bash
# Start actix-web service
cargo run
```

In a new terminal called `tailwind`
```bash
pushd spa && npx tailwindcss -i styles/input.css -o styles/tailwind.css --watch
```

In a new terminal called `trunk`
```bash
# Start trunk dev service
export TRUNK_BUILD_RELEASE=false && trunk serve
```

## Build

```bash
# Build backend
cargo build --release

# Build css
pushd spa && npx tailwindcss -i styles/input.css -o styles/tailwind.css --minify && popd

# Build frontend
export TRUNK_BUILD_RELEASE=true && trunk build

# Show build artifacts
ls -lsd target/release/rust-actix-web-api dist
```
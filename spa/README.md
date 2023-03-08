# Leptos App

SPA template in Rust using Leptos framework.

## Overview

Template includes:
- Routing
- Global state
- Components, Models and Pages module structure
- Tailwind CSS

## Dev dependencies

Web Assembly build target
```bash
rustup target add wasm32-unknown-unknown
```

Leptos docs (optional)
```bash
cargo install mdbook

mkdir -p $HOME/repo
pushd $HOME/repo

git clone --depth 1 https://github.com/leptos-rs/leptos.git && cd leptos/docs/book && mdbook serve

popd

```
## Setup

By default Trunk will serve on address *127.0.0.1* and port *8080*. To change these set with `TRUNK_SERVE_ADDRESS`
and/or `TRUNK_SERVE_PORT` environment variables.

Tailwind dev service
```bash
npm install
npx tailwindcss -i ./styles/input.css -o ./styles/tailwind.css --watch
```

Trunk dev service
```bash
cargo install trunk
trunk serve
```

## Build
```bash
npx tailwindcss -i ./styles/input.css -o ./styles/tailwind.css --minify

trunk build

ls -la dist
```


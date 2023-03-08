# Actix-web App

API template in Rust using Actix-web framework.

## Overview

Template includes:
- Routing
- Global state
- App, Routes and Services module structure

## Setup

By default Actix-web will serve on address *127.0.0.1* and port *3030*. To change these set with `ACTIXWEB_SERVE_ADDRESS`
and/or `ACTIXWEB_SERVE_PORT` environment variables.

```bash
cargo run
```

## Build 

```bash
cargo build --release
```